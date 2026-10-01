#!/usr/bin/env python3
"""Fail-closed Herdr lane creation for ZHOOR controller sessions."""

from __future__ import annotations

import argparse
import fcntl
import json
import os
import re
import shutil
import stat
import subprocess
import sys
import tempfile
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable, Literal


STATE_ROOT = Path(os.environ.get("XDG_STATE_HOME", Path.home() / ".local/state"))
REGISTRY = STATE_ROOT / "zhoor-orchestrator" / "lane-registry.json"
PI_AUTH_STORE = Path.home() / ".pi/agent/auth.json"
AGENT_NAME = re.compile(r"^[a-z][a-z0-9_-]{0,31}$")
TERMINAL_LANE_STATES = {
    "COMPLETED", "STOPPED", "CANCELLED", "TOPOLOGY_FAILED",
    "AGENT_START_FAILED", "ROLE_HANDSHAKE_FAILED", "QUARANTINED",
}
OPENAI_CHATGPT_DIRECT_SCOPE = "chatgpt.tokens.use.direct"
REVIEW_SYSTEM_PROMPT = (
    "You are an independent ZHOOR reviewer working in a read-only lane. "
    "Use only the provided read-only file tools, and base claims about repository "
    "contents or command outcomes on their results."
)
PROMPT_ENVELOPE = "ZHOOR reviewer task text follows as literal user content:\n\n"
Runner = Callable[..., subprocess.CompletedProcess[str]]
AuditTier = Literal["primary", "secondary"]


@dataclass(frozen=True)
class AuditRoute:
    audit_tier: AuditTier
    provider: str
    model: str
    reasoning: str


PRIMARY_AUDIT_ROUTE = AuditRoute("primary", "openai", "gpt-6.1-sol", "high")
SECONDARY_AUDIT_ROUTE = AuditRoute("secondary", "openai", "gpt-6.1-sol", "xhigh")


class LaneError(RuntimeError):
    pass


def resolve_audit_route(audit_tier: str) -> AuditRoute:
    if audit_tier == "primary":
        return PRIMARY_AUDIT_ROUTE
    if audit_tier == "secondary":
        return SECONDARY_AUDIT_ROUTE
    raise LaneError("audit tier must be primary or secondary")


def _is_valid_openai_chatgpt_oauth(provider: str, credential: Any) -> bool:
    if provider != "openai" or not isinstance(credential, dict):
        return False
    access = credential.get("access")
    refresh = credential.get("refresh")
    expires = credential.get("expires")
    client_id = credential.get("clientId")
    scopes = credential.get("scopes")
    return (
        credential.get("type") == "oauth"
        and isinstance(access, str) and bool(access.strip())
        and isinstance(refresh, str) and bool(refresh.strip())
        and type(expires) is int and 0 < expires <= 9_007_199_254_740_991
        and isinstance(client_id, str) and bool(client_id.strip())
        and isinstance(scopes, list)
        and all(isinstance(scope, str) and bool(scope) for scope in scopes)
        and OPENAI_CHATGPT_DIRECT_SCOPE in scopes
    )


def _validate_pi_launch_args(agent_args: list[str], route: AuditRoute) -> None:
    route_values = {
        "--provider": route.provider,
        "--model": route.model,
        "--thinking": route.reasoning,
        "--tools": "read,grep,find,ls",
        "--name": None,
    }
    required_flags = {
        "--no-extensions", "--no-skills", "--no-prompt-templates",
        "--no-themes", "--no-context-files", "--no-approve",
    }
    seen_values: dict[str, str] = {}
    seen_flags: set[str] = set()
    index = 0
    while index < len(agent_args):
        flag = agent_args[index]
        if flag in required_flags:
            if flag in seen_flags:
                raise LaneError("Pi launch contains a duplicate containment flag")
            seen_flags.add(flag)
            index += 1
            continue
        if flag not in route_values:
            raise LaneError(f"unapproved Pi launch option or argument: {flag}")
        if flag in seen_values or index + 1 >= len(agent_args):
            raise LaneError("Pi launch is missing or duplicates a required option value")
        value = agent_args[index + 1]
        if not value:
            raise LaneError("Pi launch contains an empty required option value")
        expected = route_values[flag]
        if expected is not None and value != expected:
            if flag == "--tools":
                raise LaneError("Pi containment requires exactly the read-only file tools")
            raise LaneError(f"Pi launch does not match audit tier {route.audit_tier}")
        seen_values[flag] = value
        index += 2

    if seen_flags != required_flags:
        raise LaneError("Pi launch is missing required containment flags")
    if set(seen_values) != set(route_values):
        raise LaneError("Pi launch is missing required route/tool/session options")
    if seen_values["--tools"] != "read,grep,find,ls":
        raise LaneError("Pi containment requires exactly the read-only file tools")


def _validate_scratch_directory(path: str | Path) -> Path:
    scratch = Path(path).absolute()
    try:
        resolved = scratch.resolve(strict=True)
        scratch_stat = scratch.lstat()
    except OSError:
        raise LaneError("assigned scratch directory does not exist or cannot be inspected") from None
    if resolved != scratch:
        raise LaneError("scratch directory must not contain symlink components")
    if not stat.S_ISDIR(scratch_stat.st_mode):
        raise LaneError("assigned scratch path is not a real directory")
    if scratch_stat.st_uid != os.getuid() or stat.S_IMODE(scratch_stat.st_mode) & 0o077:
        raise LaneError("assigned scratch directory must be owned by the controller user and private")
    try:
        with os.scandir(scratch) as entries:
            if next(entries, None) is not None:
                raise LaneError("assigned scratch directory must be fresh empty scratch")
    except OSError:
        raise LaneError("assigned scratch directory cannot be inspected") from None
    return scratch


def _claim_fresh_scratch(scratch: Path) -> None:
    flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC
    try:
        directory_fd = os.open(scratch, flags)
    except OSError:
        raise LaneError("scratch directory changed or contains symlink components") from None
    try:
        opened = os.fstat(directory_fd)
        current = scratch.lstat()
        if (
            not stat.S_ISDIR(opened.st_mode)
            or opened.st_dev != current.st_dev
            or opened.st_ino != current.st_ino
            or opened.st_uid != os.getuid()
            or stat.S_IMODE(opened.st_mode) & 0o077
            or scratch.resolve(strict=True) != scratch
        ):
            raise LaneError("scratch directory changed or is not private")
        if os.listdir(directory_fd):
            raise LaneError("assigned scratch directory must be fresh empty scratch")
        claim_fd = os.open(
            ".zhoor-lane-claim",
            os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC,
            0o600,
            dir_fd=directory_fd,
        )
        os.close(claim_fd)
    except FileExistsError:
        raise LaneError("assigned scratch directory has already been claimed") from None
    except OSError:
        raise LaneError("assigned scratch directory could not be claimed safely") from None
    finally:
        os.close(directory_fd)


def _prepare_pi_containment(
    agent_args: list[str], *, route: AuditRoute, scratch: Path,
) -> tuple[list[str], dict[str, str]]:
    """Install a clean-environment Pi launcher and a read-only path guard."""
    if route not in (PRIMARY_AUDIT_ROUTE, SECONDARY_AUDIT_ROUTE):
        raise LaneError("review route is not a registered audit tier")
    _validate_pi_launch_args(agent_args, route)
    provider = route.provider

    # Confirm that the approved provider-scoped ChatGPT OAuth credential exists
    # before a tab is created. Values stay in controller memory and are never rendered.
    try:
        auth = json.loads(PI_AUTH_STORE.read_text())
        credential = auth.get(provider) if isinstance(auth, dict) else None
        if not _is_valid_openai_chatgpt_oauth(provider, credential):
            raise ValueError
    except (OSError, json.JSONDecodeError, ValueError, TypeError):
        raise LaneError("OpenAI ChatGPT OAuth credential is missing or invalid") from None

    pi_executable = shutil.which("pi")
    if not pi_executable:
        raise LaneError("Pi executable cannot be resolved")
    pi_executable_path = Path(pi_executable).resolve()
    if not pi_executable_path.is_file() or not os.access(pi_executable_path, os.X_OK):
        raise LaneError("resolved Pi executable is not runnable")

    guard_source = Path(__file__).with_name("zhoor_pi_readonly_guard.ts")
    if not guard_source.is_file():
        raise LaneError("controller-authored Pi read-only guard is missing")
    private_directories = (
        scratch / "home",
        scratch / "tmp",
        scratch / "xdg",
        scratch / "pi-config",
        scratch / "pi-sessions",
        scratch / "bin",
        scratch / "xdg" / "config",
        scratch / "xdg" / "cache",
        scratch / "xdg" / "state",
    )
    try:
        for directory in private_directories:
            directory.mkdir(mode=0o700)
            os.chmod(directory, 0o700)
    except OSError:
        raise LaneError("fresh reviewer runtime directories could not be created safely") from None

    pi_config = scratch / "pi-config"
    extension_dir = pi_config / "extensions"
    extension_dir.mkdir(mode=0o700)
    os.chmod(extension_dir, 0o700)
    guard_path = extension_dir / "zhoor-readonly-guard.ts"
    shutil.copyfile(guard_source, guard_path)
    os.chmod(guard_path, 0o400)

    bin_dir = scratch / "bin"
    wrapper = bin_dir / "pi"
    isolated = {
        "HOME": scratch / "home",
        "SHELL": "/bin/sh",
        "PATH": "/usr/bin:/bin",
        "TERM": "xterm-256color",
        "LANG": "C.UTF-8",
        "PI_CODING_AGENT_DIR": pi_config,
        "PI_CODING_AGENT_SESSION_DIR": scratch / "pi-sessions",
        "XDG_CONFIG_HOME": scratch / "xdg" / "config",
        "XDG_CACHE_HOME": scratch / "xdg" / "cache",
        "XDG_STATE_HOME": scratch / "xdg" / "state",
        "TMPDIR": scratch / "tmp",
        "PI_OFFLINE": "1",
        "PI_SKIP_VERSION_CHECK": "1",
        "PI_TELEMETRY": "0",
    }
    auth_link = pi_config / "auth.json"
    auth_link.symlink_to("/proc/self/fd/3")

    # Herdr starts the executable found in the lane PATH. This small launcher
    # reads only the approved provider entry, copies it to an anonymous memfd,
    # and execs Pi with fd 3 inherited. Pi may update this isolated copy when
    # refreshing OAuth; the controller's source credential remains untouched.
    # Pi's auth.json symlink points at the descriptor, so credential values do
    # not enter argv, environment, or a regular temporary file.
    launcher_config = {
        "auth_store": str(PI_AUTH_STORE.resolve()),
        "provider": provider,
        "required_scope": OPENAI_CHATGPT_DIRECT_SCOPE,
        "pi_executable": str(pi_executable_path),
        "environment": {key: str(value) for key, value in isolated.items()},
    }
    wrapper_text = "#!/usr/bin/python3\n" + """import fcntl
import json
import os
import sys
from pathlib import Path

CONFIG = __CONFIG__
def valid_chatgpt_oauth(credential):
    if not isinstance(credential, dict):
        return False
    access = credential.get("access")
    refresh = credential.get("refresh")
    expires = credential.get("expires")
    client_id = credential.get("clientId")
    scopes = credential.get("scopes")
    return (
        credential.get("type") == "oauth"
        and isinstance(access, str) and bool(access.strip())
        and isinstance(refresh, str) and bool(refresh.strip())
        and type(expires) is int and 0 < expires <= 9007199254740991
        and isinstance(client_id, str) and bool(client_id.strip())
        and isinstance(scopes, list)
        and all(isinstance(scope, str) and bool(scope) for scope in scopes)
        and CONFIG["required_scope"] in scopes
    )
try:
    auth = json.loads(Path(CONFIG["auth_store"]).read_text())
    credential = auth.get(CONFIG["provider"]) if isinstance(auth, dict) else None
    if CONFIG["provider"] != "openai" or not valid_chatgpt_oauth(credential):
        raise ValueError
    payload = json.dumps({CONFIG["provider"]: credential}, separators=(",", ":")).encode()
    fd = os.memfd_create("pi-provider-auth", os.MFD_ALLOW_SEALING)
    os.write(fd, payload)
    os.lseek(fd, 0, os.SEEK_SET)
    if fd != 3:
        os.dup2(fd, 3, inheritable=True)
        os.close(fd)
    else:
        os.set_inheritable(3, True)
    fcntl.fcntl(3, fcntl.F_ADD_SEALS, fcntl.F_SEAL_SEAL)
    executable = CONFIG["pi_executable"]
    os.execve(executable, [executable, *sys.argv[1:]], CONFIG["environment"])
except Exception:
    print("ZHOOR isolated provider credential handoff failed", file=sys.stderr)
    raise SystemExit(125)
""".replace("__CONFIG__", repr(launcher_config))
    wrapper.write_text(wrapper_text)
    os.chmod(wrapper, 0o700)
    sandbox_args = ["--system-prompt", REVIEW_SYSTEM_PROMPT, "--extension", str(guard_path)]
    launch_env = {
        "PATH": f"{bin_dir}:/usr/bin:/bin",
        "HOME": str(scratch / "home"),
        "TMPDIR": str(scratch / "tmp"),
        "PI_CODING_AGENT_DIR": str(pi_config),
        "PI_CODING_AGENT_SESSION_DIR": str(scratch / "pi-sessions"),
        "XDG_CONFIG_HOME": str(scratch / "xdg" / "config"),
        "XDG_CACHE_HOME": str(scratch / "xdg" / "cache"),
        "XDG_STATE_HOME": str(scratch / "xdg" / "state"),
    }
    return sandbox_args, launch_env


def run(argv: list[str], *, cwd: str | None = None, runner: Runner = subprocess.run) -> str:
    """Run a Herdr/read-only validation command without shell interpolation."""
    result = runner(argv, cwd=cwd, text=True, capture_output=True, check=False)
    if result.returncode:
        # Do not surface terminal output or argv on failure: both can contain secrets.
        command = " ".join(argv[1:3]) if argv[0] == "herdr" and len(argv) > 2 else argv[0]
        raise LaneError(f"command failed ({command}): exit {result.returncode}")
    return result.stdout


def json_result(output: str, label: str) -> dict[str, Any]:
    try:
        value = json.loads(output)
    except json.JSONDecodeError as exc:
        raise LaneError(f"{label} returned invalid JSON") from exc
    if value.get("error"):
        raise LaneError(f"{label} returned an API error")
    result = value.get("result")
    if not isinstance(result, dict):
        raise LaneError(f"{label} omitted its result")
    return result


def now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def _load_registry(path: Path) -> dict[str, Any]:
    if not path.exists():
        return {"version": 1, "lanes": []}
    try:
        value = json.loads(path.read_text())
    except (OSError, json.JSONDecodeError) as exc:
        raise LaneError("lane registry is unreadable; refusing to spawn") from exc
    if value.get("version") != 1 or not isinstance(value.get("lanes"), list):
        raise LaneError("lane registry has an unsupported shape")
    return value


def _write_registry(path: Path, registry: dict[str, Any]) -> None:
    path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    fd, temp_name = tempfile.mkstemp(prefix=".lane-registry-", dir=path.parent)
    try:
        os.fchmod(fd, 0o600)
        with os.fdopen(fd, "w") as stream:
            json.dump(registry, stream, indent=2, sort_keys=True)
            stream.write("\n")
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temp_name, path)
    finally:
        try:
            os.unlink(temp_name)
        except FileNotFoundError:
            pass


def _live_role(registry: dict[str, Any], role: str) -> bool:
    return any(lane.get("logical_role") == role and lane.get("status") not in TERMINAL_LANE_STATES for lane in registry["lanes"])


def _validate_checkout(cwd: Path, worktree: Path, starting_sha: str, runner: Runner) -> None:
    if not cwd.is_dir() or not worktree.is_dir():
        raise LaneError("assigned CWD/worktree does not exist")
    real_cwd, real_worktree = cwd.resolve(), worktree.resolve()
    if real_cwd != real_worktree:
        raise LaneError("Pi review CWD must equal assigned worktree root")
    try:
        top = Path(run(["git", "-C", str(real_cwd), "rev-parse", "--show-toplevel"], runner=runner).strip()).resolve()
        head = run(["git", "-C", str(real_worktree), "rev-parse", "HEAD"], runner=runner).strip()
    except LaneError as exc:
        raise LaneError("assigned path is not a readable Git worktree") from exc
    if top != real_worktree:
        raise LaneError("CWD is outside the assigned worktree")
    if head != starting_sha:
        raise LaneError("worktree HEAD does not match starting SHA")


def _snapshot(runner: Runner) -> dict[str, Any]:
    result = json_result(run(["herdr", "api", "snapshot"], runner=runner), "Herdr snapshot")
    return result.get("snapshot", {})


def _tab_inventory(workspace: str, runner: Runner) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    tabs = json_result(run(["herdr", "tab", "list", "--workspace", workspace], runner=runner), "tab list").get("tabs", [])
    panes = json_result(run(["herdr", "pane", "list", "--workspace", workspace], runner=runner), "pane list").get("panes", [])
    return tabs, panes


def _tab_panes(tab_id: str, tabs: list[dict[str, Any]], panes: list[dict[str, Any]]) -> list[dict[str, Any]]:
    if not any(tab.get("tab_id") == tab_id for tab in tabs):
        raise LaneError("created tab is absent from live tab inventory")
    return [pane for pane in panes if pane.get("tab_id") == tab_id]


def _is_managed_ui_pane(pane: dict[str, Any]) -> bool:
    """Positively identify Herdr's Sidebar; unknown panes remain workload panes."""
    tokens = pane.get("tokens")
    return (
        pane.get("label") == "Sidebar"
        and isinstance(tokens, dict)
        and bool(tokens)
        and all(isinstance(name, str) and name.startswith("herdr-sidebar-") for name in tokens)
    )


def _interactive_pane_check(tab_id: str, root_pane_id: str, tabs: list[dict[str, Any]], panes: list[dict[str, Any]]) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    tab_panes = _tab_panes(tab_id, tabs, panes)
    managed_ui = [pane for pane in tab_panes if _is_managed_ui_pane(pane)]
    interactive = [pane for pane in tab_panes if not _is_managed_ui_pane(pane)]
    roots = [pane for pane in interactive if pane.get("pane_id") == root_pane_id]
    if len(roots) != 1 or len(interactive) != 1:
        raise LaneError("tab must contain exactly one assigned interactive pane; extra pane was not positively identified as Herdr UI")
    return interactive, managed_ui


def _restore_focus(controller_tab_id: str | None, *, runner: Runner) -> tuple[bool, bool]:
    """Return (focus_changed, focus_restored), restoring only the recorded controller tab."""
    current = _snapshot(runner).get("focused_tab_id")
    changed = current != controller_tab_id
    if not changed:
        return False, True
    if not controller_tab_id:
        return True, False
    run(["herdr", "tab", "focus", controller_tab_id], runner=runner)
    verified = _snapshot(runner).get("focused_tab_id") == controller_tab_id
    return True, verified


def spawn_agent(args: argparse.Namespace, *, runner: Runner = subprocess.run, registry_path: Path = REGISTRY) -> dict[str, Any]:
    route = resolve_audit_route(args.audit_tier)
    if not AGENT_NAME.fullmatch(args.name):
        raise LaneError("agent name is not valid for Herdr")
    if not args.cwd or not args.worktree or not args.scratch or not args.starting_sha:
        raise LaneError("CWD, worktree, scratch directory, and starting SHA are required")
    scratch = _validate_scratch_directory(args.scratch)
    if args.kind != "pi":
        raise LaneError("no verified containment profile is installed for this agent harness")
    agent_args = list(args.agent_args)
    if agent_args[:1] == ["--"]:
        agent_args = agent_args[1:]
    if not agent_args:
        raise LaneError("explicit native agent launch arguments are required")
    _validate_pi_launch_args(agent_args, route)

    registry_path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    lock_path = registry_path.with_suffix(registry_path.suffix + ".lock")
    with lock_path.open("a") as lock:
        os.chmod(lock_path, 0o600)
        fcntl.flock(lock.fileno(), fcntl.LOCK_EX)
        registry = _load_registry(registry_path)
        if _live_role(registry, args.role):
            raise LaneError(f"logical role already has a live lane: {args.role}")
        if any(lane.get("agent_name") == args.name and lane.get("status") not in TERMINAL_LANE_STATES for lane in registry["lanes"]):
            raise LaneError(f"agent name already has a live lane: {args.name}")

        cwd, worktree = Path(args.cwd), Path(args.worktree)
        _validate_checkout(cwd, worktree, args.starting_sha, runner)
        real_worktree = worktree.resolve()
        if scratch == real_worktree or scratch in real_worktree.parents or real_worktree in scratch.parents:
            raise LaneError("assigned scratch directory must be separate from the worktree")
        runtime_paths = {
            "HOME": scratch / "home",
            "TMPDIR": scratch / "tmp",
            "XDG_CONFIG_HOME": scratch / "xdg" / "config",
            "XDG_CACHE_HOME": scratch / "xdg" / "cache",
            "XDG_STATE_HOME": scratch / "xdg" / "state",
        }
        runtime_paths.update({
            "PI_CODING_AGENT_DIR": scratch / "pi-config",
            "PI_CODING_AGENT_SESSION_DIR": scratch / "pi-sessions",
        })
        _claim_fresh_scratch(scratch)
        lane_env = [f"{key}={value}" for key, value in runtime_paths.items()]
        containment_args, launch_env = _prepare_pi_containment(agent_args, route=route, scratch=scratch)
        lane_env.append(f"PATH={launch_env['PATH']}")
        lane_env += ["PI_OFFLINE=1", "PI_TELEMETRY=0", "PI_SKIP_VERSION_CHECK=1"]
        before = _snapshot(runner)
        before_focus = before.get("focused_tab_id")
        if not before_focus:
            raise LaneError("cannot establish the focused controller tab before creation")
        output = run([
            "herdr", "tab", "create", "--workspace", args.workspace,
            "--cwd", str(cwd.resolve()), "--label", args.label, "--no-focus",
            *[arg for item in lane_env for arg in ("--env", item)],
        ], runner=runner)
        created = json_result(output, "tab create")
        tab = created.get("tab", {})
        root = created.get("root_pane", {})
        tab_id, pane_id = tab.get("tab_id"), root.get("pane_id")
        if not tab_id or not pane_id:
            raise LaneError("tab create omitted tab or root pane ID; no fallback permitted")
        lane = {
            "logical_role": args.role,
            "audit_tier": route.audit_tier,
            "agent_name": args.name,
            "harness": args.kind,
            "provider": route.provider,
            "model": route.model,
            "reasoning": route.reasoning,
            "tab_id": tab_id,
            "pane_id": pane_id,
            "cwd": str(cwd.resolve()),
            "worktree": str(worktree.resolve()),
            "scratch": str(scratch),
            "runtime_directories": {key: str(value) for key, value in runtime_paths.items()},
            "starting_sha": args.starting_sha,
            "candidate_sha_if_any": None,
            "status": "TAB_CREATED_VERIFYING",
            "containment_status": "PENDING",
            "stop_method": "herdr agent send-keys <agent_name> ctrl+c",
            "created_at": now(),
        }
        registry["lanes"].append(lane)
        _write_registry(registry_path, registry)

        try:
            focus_changed, focus_restored = _restore_focus(before_focus, runner=runner)
        except LaneError:
            focus_changed, focus_restored = True, False
        lane["focus_changed_on_create"] = focus_changed
        lane["focus_restored"] = focus_restored
        topology_problem: str | None = None if focus_restored else "controller focus could not be restored and verified"
        try:
            tabs, panes = _tab_inventory(args.workspace, runner)
            interactive, managed_ui = _interactive_pane_check(tab_id, pane_id, tabs, panes)
            if Path(interactive[0].get("cwd", "")).resolve() != cwd.resolve():
                topology_problem = topology_problem or "new root pane CWD differs from assigned CWD"
            lane["interactive_pane_ids"] = [pane["pane_id"] for pane in interactive]
            lane["managed_ui_pane_ids"] = [pane["pane_id"] for pane in managed_ui]
        except LaneError as exc:
            topology_problem = topology_problem or str(exc)

        # Herdr 0.9.1 can select the new tab after the first snapshot, while
        # tab/pane inventory is being read. Recheck after inventory and restore
        # before starting the agent or sending even the role-only handshake.
        try:
            changed_after_inventory, restored_after_inventory = _restore_focus(before_focus, runner=runner)
        except LaneError:
            changed_after_inventory, restored_after_inventory = True, False
        focus_changed = focus_changed or changed_after_inventory
        focus_restored = focus_restored and restored_after_inventory
        lane["focus_changed_on_create"] = focus_changed
        lane["focus_restored"] = focus_restored
        if not focus_restored:
            topology_problem = topology_problem or "controller focus could not be restored and verified after tab inventory"

        if topology_problem:
            lane["status"] = "TOPOLOGY_FAILED"
            lane["topology_failure"] = topology_problem
            lane["focus_changed_on_create"] = focus_changed
            _write_registry(registry_path, registry)
            raise LaneError(topology_problem)

        lane["status"] = "TAB_READY_AGENT_START_PENDING"
        _write_registry(registry_path, registry)
        try:
            run(["herdr", "agent", "start", args.name, "--kind", args.kind, "--pane", pane_id, "--", *agent_args, *containment_args], runner=runner)
        except LaneError:
            lane["status"] = "AGENT_START_FAILED"
            _write_registry(registry_path, registry)
            raise

        try:
            focus_changed_after_start, focus_restored_after_start = _restore_focus(before_focus, runner=runner)
        except LaneError:
            focus_changed_after_start, focus_restored_after_start = True, False
        lane["focus_changed_after_start"] = focus_changed_after_start
        lane["focus_restored"] = lane["focus_restored"] and focus_restored_after_start
        if not lane["focus_restored"]:
            lane["status"] = "TOPOLOGY_FAILED"
            _write_registry(registry_path, registry)
            raise LaneError("controller focus could not be restored after agent start")

        # Fixed role-only handshake; no repository content or source paths are sent.
        prompt_text = (
            f"Session role handshake only. Your assigned role is {args.role}; "
            f"your agent name is {args.name}. Confirm your harness and the model "
            "and provider you observe. Confirm that you have not inspected repository "
            "source or received task content. Do not use tools or inspect files."
        )
        lane["status"] = "ROLE_HANDSHAKE_PENDING"
        _write_registry(registry_path, registry)
        try:
            run(["herdr", "agent", "prompt", args.name, prompt_text], runner=runner)
        except LaneError:
            lane["status"] = "ROLE_HANDSHAKE_FAILED"
            _write_registry(registry_path, registry)
            raise
        after = _snapshot(runner)
        if after.get("focused_tab_id") != before_focus:
            try:
                changed, restored = _restore_focus(before_focus, runner=runner)
            except LaneError:
                changed, restored = True, False
            lane["focus_changed_after_handshake"] = changed
            lane["focus_restored"] = lane["focus_restored"] and restored
        if not lane["focus_restored"]:
            lane["status"] = "TOPOLOGY_FAILED"
            _write_registry(registry_path, registry)
            raise LaneError("controller focus could not be restored after role handshake")
        final_tabs, final_panes = _tab_inventory(args.workspace, runner)
        try:
            interactive, managed_ui = _interactive_pane_check(tab_id, pane_id, final_tabs, final_panes)
        except LaneError:
            lane["status"] = "TOPOLOGY_FAILED"
            _write_registry(registry_path, registry)
            raise
        if Path(interactive[0].get("cwd", "")).resolve() != cwd.resolve():
            lane["status"] = "TOPOLOGY_FAILED"
            _write_registry(registry_path, registry)
            raise LaneError("agent startup changed the assigned workload CWD")
        lane["interactive_pane_ids"] = [pane["pane_id"] for pane in interactive]
        lane["managed_ui_pane_ids"] = [pane["pane_id"] for pane in managed_ui]
        _write_registry(registry_path, registry)
        return lane


def spawn_utility_tab(
    role: str, workspace: str, label: str, cwd: str, worktree: str, starting_sha: str,
    *, runner: Runner = subprocess.run, registry_path: Path = REGISTRY,
) -> dict[str, Any]:
    """Create one ordinary shell tab for persistent human-visible utilities."""
    registry_path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    lock_path = registry_path.with_suffix(registry_path.suffix + ".lock")
    with lock_path.open("a") as lock:
        os.chmod(lock_path, 0o600)
        fcntl.flock(lock.fileno(), fcntl.LOCK_EX)
        registry = _load_registry(registry_path)
        if _live_role(registry, role):
            raise LaneError(f"logical role already has a live lane: {role}")
        cwd_path, worktree_path = Path(cwd), Path(worktree)
        _validate_checkout(cwd_path, worktree_path, starting_sha, runner)
        before_focus = _snapshot(runner).get("focused_tab_id")
        if not before_focus:
            raise LaneError("cannot establish the focused controller tab before utility creation")
        created = json_result(run([
            "herdr", "tab", "create", "--workspace", workspace,
            "--cwd", str(cwd_path.resolve()), "--label", label, "--no-focus",
        ], runner=runner), "tab create")
        tab_id = created.get("tab", {}).get("tab_id")
        pane_id = created.get("root_pane", {}).get("pane_id")
        if not tab_id or not pane_id:
            raise LaneError("tab create omitted tab or root pane ID; no fallback permitted")
        lane = {
            "logical_role": role, "agent_name": None, "harness": "shell",
            "provider": None, "model": None, "reasoning": None,
            "tab_id": tab_id, "pane_id": pane_id, "cwd": str(cwd_path.resolve()),
            "worktree": str(worktree_path.resolve()), "scratch": None,
            "starting_sha": starting_sha, "candidate_sha_if_any": None,
            "status": "TAB_CREATED_VERIFYING", "containment_status": "NOT_APPLICABLE",
            "stop_method": "foreground command only; no persistent service",
            "created_at": now(),
        }
        registry["lanes"].append(lane)
        _write_registry(registry_path, registry)
        try:
            focus_changed, focus_restored = _restore_focus(before_focus, runner=runner)
        except LaneError:
            focus_changed, focus_restored = True, False
        lane["focus_changed_on_create"] = focus_changed
        lane["focus_restored"] = focus_restored
        topology_problem: str | None = None if focus_restored else "controller focus could not be restored and verified"
        try:
            tabs, panes = _tab_inventory(workspace, runner)
            interactive, managed_ui = _interactive_pane_check(tab_id, pane_id, tabs, panes)
            if Path(interactive[0].get("cwd", "")).resolve() != cwd_path.resolve():
                topology_problem = topology_problem or "utility root pane CWD differs from assigned CWD"
            lane["interactive_pane_ids"] = [pane["pane_id"] for pane in interactive]
            lane["managed_ui_pane_ids"] = [pane["pane_id"] for pane in managed_ui]
        except LaneError as exc:
            topology_problem = topology_problem or str(exc)
        # Recheck after pane inventory: tab creation may apply its focus change
        # asynchronously, even when the immediate snapshot still showed CTRL.
        try:
            changed_after_inventory, restored_after_inventory = _restore_focus(before_focus, runner=runner)
        except LaneError:
            changed_after_inventory, restored_after_inventory = True, False
        focus_changed = focus_changed or changed_after_inventory
        focus_restored = focus_restored and restored_after_inventory
        lane["focus_changed_on_create"] = focus_changed
        lane["focus_restored"] = focus_restored
        if not focus_restored:
            topology_problem = topology_problem or "controller focus could not be restored and verified after tab inventory"
        if topology_problem:
            lane["status"] = "TOPOLOGY_FAILED"
            lane["topology_failure"] = topology_problem
            lane["focus_changed_on_create"] = focus_changed
            _write_registry(registry_path, registry)
            raise LaneError(topology_problem)
        lane["status"] = "IDLE"
        _write_registry(registry_path, registry)
        return lane


def set_containment(role: str, status: str, *, evidence: str, registry_path: Path = REGISTRY) -> dict[str, Any]:
    if status != "PASS":
        raise LaneError("containment status can only be advanced to PASS")
    evidence_path = Path(evidence)
    if not evidence_path.is_file():
        raise LaneError("containment evidence file is missing")
    with registry_path.with_suffix(registry_path.suffix + ".lock").open("a") as lock:
        fcntl.flock(lock.fileno(), fcntl.LOCK_EX)
        registry = _load_registry(registry_path)
        matches = [lane for lane in registry["lanes"] if lane.get("logical_role") == role and lane.get("status") not in TERMINAL_LANE_STATES]
        if len(matches) != 1:
            raise LaneError("expected exactly one live lane for containment update")
        matches[0]["containment_status"] = "PASS"
        matches[0]["containment_evidence"] = str(evidence_path.resolve())
        matches[0]["containment_verified_at"] = now()
        _write_registry(registry_path, registry)
        return matches[0]


def send_prompt(role: str, text: str, *, runner: Runner = subprocess.run, registry_path: Path = REGISTRY) -> str:
    registry = _load_registry(registry_path)
    lanes = [lane for lane in registry["lanes"] if lane.get("logical_role") == role and lane.get("status") not in TERMINAL_LANE_STATES]
    if len(lanes) != 1:
        raise LaneError("expected exactly one live lane for prompt")
    lane = lanes[0]
    if lane.get("containment_status") != "PASS":
        raise LaneError("prompt refused before AGENT_CONTAINMENT=PASS")
    return run(
        ["herdr", "agent", "prompt", lane["agent_name"], f"{PROMPT_ENVELOPE}{text}"],
        runner=runner,
    )


def wait_agent(role: str, timeout_ms: int | None = None, *, runner: Runner = subprocess.run, registry_path: Path = REGISTRY) -> str:
    lane = find_live(registry_path, role)
    command = ["herdr", "agent", "wait", lane["agent_name"]]
    if timeout_ms is not None:
        command += ["--timeout", str(timeout_ms)]
    return run(command, runner=runner)


def read_agent(role: str, lines: int = 120, *, runner: Runner = subprocess.run, registry_path: Path = REGISTRY) -> str:
    lane = find_live(registry_path, role)
    return run(["herdr", "agent", "read", lane["agent_name"], "--source", "recent-unwrapped", "--lines", str(lines)], runner=runner)


def get_agent_status(role: str, *, runner: Runner = subprocess.run, registry_path: Path = REGISTRY) -> str:
    lane = find_live(registry_path, role)
    return run(["herdr", "agent", "get", lane["agent_name"]], runner=runner)


def find_live(registry_path: Path, role: str) -> dict[str, Any]:
    registry = _load_registry(registry_path)
    matches = [lane for lane in registry["lanes"] if lane.get("logical_role") == role and lane.get("status") not in TERMINAL_LANE_STATES]
    if len(matches) != 1:
        raise LaneError("expected exactly one live lane")
    return matches[0]


def stop_agent(role: str, *, runner: Runner = subprocess.run, registry_path: Path = REGISTRY) -> dict[str, Any]:
    lane = find_live(registry_path, role)
    run(["herdr", "agent", "send-keys", lane["agent_name"], "ctrl+c"], runner=runner)
    lane["status"] = "STOP_REQUESTED"
    registry = _load_registry(registry_path)
    for item in registry["lanes"]:
        if item.get("agent_name") == lane["agent_name"]:
            item["status"] = lane["status"]
    _write_registry(registry_path, registry)
    return lane


def complete_lane(role: str, evidence: str, candidate_sha: str | None = None, *, runner: Runner = subprocess.run, registry_path: Path = REGISTRY) -> dict[str, Any]:
    lane = find_live(registry_path, role)
    if not Path(evidence).is_file():
        raise LaneError("result evidence must be recorded before lane completion")
    if lane.get("agent_name"):
        result = json_result(run(["herdr", "agent", "list"], runner=runner), "agent list")
        if any(agent.get("agent") == lane["agent_name"] for agent in result.get("agents", [])):
            raise LaneError("agent is still live; keep its tab available and do not mark complete")
    registry = _load_registry(registry_path)
    for item in registry["lanes"]:
        if item.get("tab_id") == lane.get("tab_id") and item.get("logical_role") == role:
            item["status"] = "COMPLETED"
            item["result_evidence"] = str(Path(evidence).resolve())
            item["candidate_sha_if_any"] = candidate_sha
            item["completed_at"] = now()
            lane = item
            break
    _write_registry(registry_path, registry)
    return lane


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description=__doc__)
    sub = root.add_subparsers(dest="command", required=True)
    spawn = sub.add_parser("spawn", help="create a dedicated no-focus one-pane tab and start one named agent")
    for flag in ("role", "name", "kind", "workspace", "cwd", "worktree", "scratch", "starting-sha", "label"):
        spawn.add_argument("--" + flag, required=True)
    spawn.add_argument("--audit-tier", choices=("primary", "secondary"), required=True)
    spawn.add_argument("agent_args", nargs=argparse.REMAINDER)
    prompt = sub.add_parser("prompt", help="send a prompt to the registered live lane")
    prompt.add_argument("--role", required=True)
    prompt.add_argument("text")
    wait = sub.add_parser("wait", help="wait for a registered live lane")
    wait.add_argument("--role", required=True)
    wait.add_argument("--timeout", type=int)
    read = sub.add_parser("read", help="read a registered live lane")
    read.add_argument("--role", required=True)
    read.add_argument("--lines", type=int, default=120)
    status = sub.add_parser("status", help="get a registered live lane status")
    status.add_argument("--role", required=True)
    stop = sub.add_parser("stop", help="interrupt a registered live lane with ctrl+c")
    stop.add_argument("--role", required=True)
    contain = sub.add_parser("containment-pass", help="record controller-verified containment evidence")
    contain.add_argument("--role", required=True)
    contain.add_argument("--evidence", required=True)
    utility = sub.add_parser("utility-tab", help="create a persistent one-pane shell tab without focusing it")
    for flag in ("role", "workspace", "label", "cwd", "worktree", "starting-sha"):
        utility.add_argument("--" + flag, required=True)
    complete = sub.add_parser("complete", help="record consumed result evidence and complete an exited lane")
    complete.add_argument("--role", required=True)
    complete.add_argument("--evidence", required=True)
    complete.add_argument("--candidate-sha")
    return root


def main(argv: list[str] | None = None) -> int:
    args = parser().parse_args(argv)
    try:
        if args.command == "spawn":
            value = spawn_agent(args)
        elif args.command == "prompt":
            print(send_prompt(args.role, args.text), end="")
            return 0
        elif args.command == "wait":
            print(wait_agent(args.role, args.timeout), end="")
            return 0
        elif args.command == "read":
            print(read_agent(args.role, args.lines), end="")
            return 0
        elif args.command == "status":
            print(get_agent_status(args.role), end="")
            return 0
        elif args.command == "stop":
            value = stop_agent(args.role)
        elif args.command == "utility-tab":
            value = spawn_utility_tab(args.role, args.workspace, args.label, args.cwd, args.worktree, args.starting_sha)
        elif args.command == "complete":
            value = complete_lane(args.role, args.evidence, args.candidate_sha)
        else:
            value = set_containment(args.role, "PASS", evidence=args.evidence)
        print(json.dumps(value, indent=2, sort_keys=True))
        return 0
    except LaneError as exc:
        print(f"zhoor-herdr: {exc}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
