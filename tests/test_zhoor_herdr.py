from __future__ import annotations

import fcntl
import json
import os
import pty
import re
import select
import shutil
import signal
import stat
import struct
import subprocess
import tempfile
import termios
import time
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

from scripts import zhoor_herdr


SHA = "480d4e801e53480beced1a609622a027d3806828"


class FakeHerdr:
    def __init__(self, cwd: Path, *, panes: int = 2, extra_kind: str = "sidebar", focus_changes: bool = False, delayed_focus_change: bool = False, restore_fails: bool = False, fail_create: bool = False, sidebar_metadata_delay: int = 0):
        self.cwd = cwd
        self.pane_count = panes
        self.extra_kind = extra_kind
        self.focus_changes = focus_changes
        self.delayed_focus_change = delayed_focus_change
        self.restore_fails = restore_fails
        self.fail_create = fail_create
        self.sidebar_metadata_delay = sidebar_metadata_delay
        self.pane_list_calls = 0
        self.created = False
        self.focused_tab = "controller-tab"
        self.calls: list[list[str]] = []

    def __call__(self, argv, *, cwd=None, text=None, capture_output=None, check=None):
        args = list(argv)
        self.calls.append(args)
        if args[0] == "git":
            value = str(self.cwd) if args[-1] == "--show-toplevel" else SHA
            return subprocess.CompletedProcess(args, 0, value + "\n", "")
        if args[:3] == ["herdr", "api", "snapshot"]:
            result = {"snapshot": {"focused_tab_id": self.focused_tab, "focused_workspace_id": "workspace"}}
            return subprocess.CompletedProcess(args, 0, json.dumps({"result": result}), "")
        if args[:3] == ["herdr", "tab", "create"]:
            if self.fail_create:
                return subprocess.CompletedProcess(args, 1, "", "failed")
            self.created = True
            if self.focus_changes:
                self.focused_tab = "new-tab"
            result = {"tab": {"tab_id": "new-tab"}, "root_pane": {"pane_id": "root-pane"}}
            return subprocess.CompletedProcess(args, 0, json.dumps({"result": result}), "")
        if args[:3] == ["herdr", "tab", "focus"]:
            if self.restore_fails:
                return subprocess.CompletedProcess(args, 1, "", "failed")
            self.focused_tab = args[3]
            return subprocess.CompletedProcess(args, 0, "{}", "")
        if args[:3] == ["herdr", "tab", "list"]:
            tabs = [{"tab_id": "controller-tab", "pane_count": 1}]
            if self.created:
                tabs.append({"tab_id": "new-tab", "pane_count": self.pane_count})
            return subprocess.CompletedProcess(args, 0, json.dumps({"result": {"tabs": tabs}}), "")
        if args[:3] == ["herdr", "pane", "list"]:
            self.pane_list_calls += 1
            if self.created and self.delayed_focus_change:
                self.focused_tab = "new-tab"
                self.delayed_focus_change = False
            panes = []
            if self.created:
                panes = [{"tab_id": "new-tab", "pane_id": "root-pane", "cwd": str(self.cwd)}]
                for idx in range(1, self.pane_count):
                    extra = {"tab_id": "new-tab", "pane_id": f"extra-pane-{idx}", "cwd": str(self.cwd)}
                    if self.extra_kind == "sidebar" and self.pane_list_calls > self.sidebar_metadata_delay:
                        extra.update({"label": "Sidebar", "tokens": {"herdr-sidebar-explorer": "test"}})
                    elif self.extra_kind == "sidebar":
                        extra.update({"label": "Sidebar"})
                    elif self.extra_kind == "interactive":
                        extra.update({"agent": f"worker-{idx}", "agent_status": "idle"})
                    else:
                        extra.update({"label": "Unknown UI"})
                    panes.append(extra)
            return subprocess.CompletedProcess(args, 0, json.dumps({"result": {"panes": panes}}), "")
        return subprocess.CompletedProcess(args, 0, "{}", "")


def spawn_args(cwd: Path, scratch: Path) -> SimpleNamespace:
    return SimpleNamespace(
        role="ARCH",
        name="pro01-arch-review",
        kind="pi",
        audit_tier="primary",
        workspace="workspace",
        cwd=str(cwd),
        worktree=str(cwd),
        scratch=str(scratch),
        starting_sha=SHA,
        label="ARCH · Pi · Sol-high",
        agent_args=["--", "--provider", "openai", "--model", "gpt-6.1-sol", "--thinking", "high", "--no-extensions", "--no-skills", "--no-prompt-templates", "--no-themes", "--no-context-files", "--no-approve", "--tools", "read,grep,find,ls", "--name", "pro01-arch-review"],
    )


class ZhoorHerdrTopologyTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.cwd = self.root / "worktree"
        self.cwd.mkdir()
        self.scratch = self.root / "scratch"
        self.scratch.mkdir(mode=0o700)
        self.registry = self.root / "state" / "lane-registry.json"
        self.profile_dir = self.root / "reviewer-profile"
        self.profile_dir.mkdir(mode=0o700, parents=True)
        self.auth_store = self.profile_dir / "auth.json"
        self.oauth_credential = {
            "access": "synthetic-test-access", "refresh": "synthetic-test-refresh",
            "expires": 2_000_000_000_000, "type": "oauth",
            "clientId": "synthetic-test-client",
            "scopes": ["openid", "chatgpt.tokens.use.direct"],
            "accountMetadata": {"tenant": "synthetic-test-tenant"},
        }
        self.auth_store.write_text(json.dumps({"openai": self.oauth_credential}))
        os.chmod(self.profile_dir, 0o700)
        os.chmod(self.auth_store, 0o600)
        self.profile_patch = patch.multiple(
            zhoor_herdr,
            REVIEWER_PROFILE_DIR=self.profile_dir,
            REVIEWER_AUTH_STORE=self.auth_store,
        )
        self.profile_patch.start()

    def tearDown(self):
        self.profile_patch.stop()
        self.temp.cleanup()

    def assert_invalid_openai_credential_is_rejected(self, credential):
        self.auth_store.write_text(json.dumps({"openai": credential}))
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "dedicated reviewer OAuth profile.*invalid"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))

    def test_refreshing_auth_check_uses_authoritative_lock_and_sanitizes_output(self):
        fake_pi = self.root / "fake-pi-auth-check"
        report_path = self.root / "auth-check-report.json"
        fake_pi.write_text(f'''#!/usr/bin/python3
import fcntl
import json
import os
import sys
from pathlib import Path
if sys.argv[1:] == ["--version"]:
    print("0.99.1")
    raise SystemExit(0)
auth_path = Path(os.environ["PI_CODING_AGENT_DIR"]) / "auth.json"
probe = auth_path.open("rb")
try:
    fcntl.flock(probe.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
    lock_held = False
except BlockingIOError:
    lock_held = True
Path({str(report_path)!r}).write_text(json.dumps({{
    "lock_held": lock_held,
    "profile": os.environ.get("PI_CODING_AGENT_DIR"),
    "api_key_env": "OPENAI_API_KEY" in os.environ,
    "credentials_flag": "--credentials" in sys.argv,
    "no_refresh_flag": "--no-refresh" in sys.argv,
}}))
print(json.dumps({{
    "status": "ready", "provider": "openai", "authType": "oauth",
    "syntheticCredential": "SYNTHETIC_AUTH_VALUE_MUST_NOT_ESCAPE",
}}))
''')
        fake_pi.chmod(0o700)
        with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
            for no_refresh in (True, False):
                with self.subTest(no_refresh=no_refresh):
                    result = zhoor_herdr.check_reviewer_auth(no_refresh=no_refresh)
                    report = json.loads(report_path.read_text())
                    self.assertEqual(result, {"status": "ready", "provider": "openai", "authType": "oauth"})
                    self.assertTrue(report["lock_held"])
                    self.assertEqual(report["profile"], str(self.profile_dir))
                    self.assertFalse(report["api_key_env"])
                    self.assertFalse(report["credentials_flag"])
                    self.assertEqual(report["no_refresh_flag"], no_refresh)
                    self.assertNotIn("SYNTHETIC_AUTH_VALUE_MUST_NOT_ESCAPE", json.dumps(result))
        probe = self.auth_store.open("rb")
        try:
            fcntl.flock(probe.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        finally:
            probe.close()

    def test_refreshing_auth_check_fails_closed_when_reviewer_store_is_locked(self):
        lock = self.auth_store.open("rb")
        fcntl.flock(lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        calls = []
        try:
            with self.assertRaisesRegex(zhoor_herdr.LaneError, "reviewer OAuth store is already active"):
                zhoor_herdr.check_reviewer_auth(
                    no_refresh=False,
                    runner=lambda *args, **kwargs: calls.append((args, kwargs)),
                )
        finally:
            lock.close()
        self.assertEqual(calls, [])

    def test_refreshing_auth_check_process_cannot_bypass_live_reviewer_lock(self):
        fake_pi = self.root / "fake-pi-auth-check-overlap"
        lane_started = self.root / "reviewer-lane-started"
        auth_called = self.root / "auth-check-invoked-pi"
        release = self.root / "release-reviewer-lane"
        fake_pi.write_text(f'''#!/usr/bin/python3
import os
import sys
import time
from pathlib import Path
if sys.argv[1:] == ["--version"]:
    print("0.99.1")
    raise SystemExit(0)
if sys.argv[1:3] == ["auth", "check"]:
    Path({str(auth_called)!r}).write_text("called")
    print('{{"status":"ready","provider":"openai","authType":"oauth"}}')
    raise SystemExit(0)
Path({str(lane_started)!r}).write_text("started")
while not Path({str(release)!r}).exists():
    time.sleep(0.01)
''')
        fake_pi.chmod(0o700)
        scratch = self.root / "scratch-auth-overlap-lane"
        scratch.mkdir(mode=0o700)
        args = spawn_args(self.cwd, scratch)
        with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
            zhoor_herdr._prepare_pi_containment(
                args.agent_args[1:],
                route=zhoor_herdr.resolve_audit_route(args.audit_tier),
                scratch=scratch,
            )
        lane = subprocess.Popen(
            [str(scratch / "bin" / "pi")],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        deadline = time.monotonic() + 5
        while not lane_started.exists() and lane.poll() is None and time.monotonic() < deadline:
            time.sleep(0.01)
        self.assertTrue(lane_started.exists(), f"reviewer Pi did not start: {lane.poll()}")

        check_script = self.root / "run-auth-check.py"
        check_script.write_text(f'''import sys
from pathlib import Path
from scripts import zhoor_herdr
auth_store = Path(sys.argv[1])
zhoor_herdr.REVIEWER_PROFILE_DIR = auth_store.parent
zhoor_herdr.REVIEWER_AUTH_STORE = auth_store
zhoor_herdr.shutil.which = lambda name: sys.argv[2]
try:
    zhoor_herdr.check_reviewer_auth(no_refresh=False)
except zhoor_herdr.LaneError as exc:
    print("blocked" if "already active" in str(exc) else "failed")
else:
    print("unexpected-ready")
''')
        repo_root = str(Path(__file__).resolve().parents[1])
        check_env = {"PATH": "/usr/bin:/bin", "PYTHONPATH": repo_root, "LANG": "C.UTF-8"}
        try:
            check = subprocess.run(
                ["python3.14", str(check_script), str(self.auth_store), str(fake_pi)],
                env=check_env, text=True, capture_output=True, check=False,
            )
            self.assertEqual(check.returncode, 0, check.stderr)
            self.assertEqual(check.stdout.strip(), "blocked")
            self.assertFalse(auth_called.exists())
        finally:
            release.touch()
            lane.communicate(timeout=5)

    def test_profile_lock_excludes_competing_process_and_releases_after_exit_signals(self):
        def make_fake_pi(name, *, wait_for_release=None):
            executable = self.root / name
            started = self.root / f"{name}.started"
            release = self.root / f"{name}.release"
            wait_code = (
                f"while not Path({str(release)!r}).exists(): time.sleep(0.01)\n"
                if wait_for_release else ""
            )
            executable.write_text(
                "#!/usr/bin/python3\n"
                "import json, os, sys, time, fcntl\n"
                "from pathlib import Path\n"
                "if sys.argv[1:] == ['--version']:\n"
                "    print('0.99.1')\n"
                "    raise SystemExit(0)\n"
                f"Path({str(started)!r}).write_text('started')\n"
                "auth_path = Path(os.environ['PI_CODING_AGENT_DIR']) / 'auth.json'\n"
                "probe = auth_path.open('rb')\n"
                "try:\n"
                "    fcntl.flock(probe.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)\n"
                "    held = False\n"
                "except BlockingIOError:\n"
                "    held = True\n"
                f"{wait_code}"
                "print(json.dumps({'lock_held': held}))\n"
            )
            executable.chmod(0o700)
            return executable, started, release

        def prepare_lane(tag, fake_pi):
            scratch = self.root / f"scratch-{tag}"
            scratch.mkdir(mode=0o700)
            args = spawn_args(self.cwd, scratch)
            with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
                zhoor_herdr._prepare_pi_containment(
                    args.agent_args[1:],
                    route=zhoor_herdr.resolve_audit_route(args.audit_tier),
                    scratch=scratch,
                )
            return scratch / "bin" / "pi"

        def wait_for(path, process):
            deadline = time.monotonic() + 5
            while not path.exists() and time.monotonic() < deadline:
                if process.poll() is not None:
                    break
                time.sleep(0.01)
            self.assertTrue(path.exists(), f"fake Pi did not start: {process.poll()}")

        for exit_mode, termination_signal in (
            ("normal", None), ("sigterm", signal.SIGTERM), ("sigkill", signal.SIGKILL)
        ):
            with self.subTest(exit_mode=exit_mode):
                blocker, started, release = make_fake_pi(f"fake-blocker-{exit_mode}", wait_for_release=True)
                blocker_lane = prepare_lane(f"blocker-{exit_mode}", blocker)
                process = subprocess.Popen([str(blocker_lane)], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                wait_for(started, process)

                competing_fake, competing_started, _ = make_fake_pi(f"fake-competing-{exit_mode}")
                competing_lane = prepare_lane(f"competing-{exit_mode}", competing_fake)
                competing = subprocess.run([str(competing_lane)], capture_output=True, text=True, check=False)
                self.assertEqual(competing.returncode, 125)
                self.assertFalse(competing_started.exists())

                if termination_signal is None:
                    release.touch()
                    stdout, stderr = process.communicate(timeout=5)
                    self.assertEqual(process.returncode, 0, stderr)
                    self.assertIn('"lock_held": true', stdout)
                else:
                    process.send_signal(termination_signal)
                    process.communicate(timeout=5)

                successor_fake, successor_started, _ = make_fake_pi(f"fake-successor-{exit_mode}")
                successor_lane = prepare_lane(f"successor-{exit_mode}", successor_fake)
                successor = subprocess.run([str(successor_lane)], capture_output=True, text=True, check=False)
                self.assertEqual(successor.returncode, 0, successor.stderr)
                self.assertTrue(successor_started.exists())
                self.assertIn('"lock_held": true', successor.stdout)

    def test_spawn_uses_no_focus_tab_and_accepts_sidebar_as_managed_ui(self):
        fake = FakeHerdr(self.cwd)
        lane = zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        create = next(call for call in fake.calls if call[:3] == ["herdr", "tab", "create"])
        start = next(call for call in fake.calls if call[:3] == ["herdr", "agent", "start"])
        self.assertIn("--no-focus", create)
        self.assertEqual(start[start.index("--pane") + 1], "root-pane")
        self.assertEqual(lane["tab_id"], "new-tab")
        self.assertEqual(lane["pane_id"], "root-pane")
        self.assertEqual(lane["containment_status"], "PENDING")
        self.assertEqual(lane["status"], "ROLE_HANDSHAKE_PENDING")
        self.assertEqual(lane["audit_tier"], "primary")
        self.assertEqual((lane["provider"], lane["model"], lane["reasoning"]), ("openai", "gpt-6.1-sol", "high"))
        self.assertEqual(lane["interactive_pane_ids"], ["root-pane"])
        self.assertEqual(lane["managed_ui_pane_ids"], ["extra-pane-1"])
        self.assertFalse(lane["focus_changed_on_create"])
        self.assertTrue(lane["focus_restored"])
        self.assertIn(f"HOME={self.scratch}/home", create)
        self.assertIn(f"TMPDIR={self.scratch}/tmp", create)
        self.assertIn(f"PI_CODING_AGENT_DIR={self.scratch}/pi-config", create)
        self.assertIn(f"PI_CODING_AGENT_SESSION_DIR={self.scratch}/pi-sessions", create)
        start_args = start[start.index("--") + 1:]
        self.assertEqual(
            start_args[start_args.index("--system-prompt") + 1],
            zhoor_herdr.REVIEW_SYSTEM_PROMPT,
        )
        guard_index = start_args.index("--extension")
        guard_path = Path(start_args[guard_index + 1])
        wrapper = self.scratch / "bin" / "pi"
        self.assertTrue(guard_path.is_file())
        self.assertTrue(wrapper.is_file())
        wrapper_text = wrapper.read_text()
        self.assertIn("fcntl.flock", wrapper_text)
        self.assertNotIn("os.memfd_create", wrapper_text)
        self.assertNotIn("/proc/self/fd/3", wrapper_text)
        self.assertIn("os.execve", wrapper_text)
        self.assertTrue((self.scratch / "pi-config" / "auth.json").is_symlink())
        self.assertEqual(
            os.readlink(self.scratch / "pi-config" / "auth.json"),
            str(self.auth_store.resolve()),
        )
        self.assertEqual(stat.S_IMODE(self.auth_store.stat().st_mode), 0o600)
        self.assertFalse((self.profile_dir / "reviewer-session.lock").exists())
        self.assertNotIn("synthetic-test-access", wrapper_text)
        self.assertNotIn("synthetic-test-refresh", wrapper_text)
        self.assertNotIn("AGENT_BROWSER_ENCRYPTION_KEY", wrapper_text)
        self.assertIn(f"PATH={self.scratch}/bin:/usr/bin:/bin", create)
        self.assertIn("--session-dir", start_args)
        self.assertEqual(
            start_args[start_args.index("--session-dir") + 1],
            f"{self.scratch}/pi-sessions",
        )
        version = subprocess.run([str(wrapper), "--version"], text=True, capture_output=True, check=False)
        self.assertEqual(version.returncode, 0)
        self.assertIn("0.99.1", version.stdout)
        self.assertFalse(any(call[:2] == ["herdr", "pane"] and "split" in call for call in fake.calls))

    def test_spawn_waits_for_sidebar_ownership_metadata_before_agent_start(self):
        fake = FakeHerdr(self.cwd, sidebar_metadata_delay=1)
        with patch.object(time, "sleep", return_value=None):
            lane = zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertEqual(lane["status"], "ROLE_HANDSHAKE_PENDING")
        self.assertGreaterEqual(fake.pane_list_calls, 2)

    def test_spawn_fails_closed_when_sidebar_ownership_metadata_never_arrives(self):
        fake = FakeHerdr(self.cwd, sidebar_metadata_delay=100)
        with patch.object(time, "sleep", return_value=None):
            with self.assertRaisesRegex(zhoor_herdr.LaneError, "not positively identified as Herdr UI"):
                zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        lane = json.loads(self.registry.read_text())["lanes"][0]
        self.assertEqual(lane["status"], "TOPOLOGY_FAILED")
        self.assertEqual(fake.pane_list_calls, zhoor_herdr.SIDEBAR_METADATA_WAIT_ATTEMPTS)
        self.assertFalse(any(call[:3] == ["herdr", "agent", "start"] for call in fake.calls))

    def test_unreviewed_pi_version_fails_before_reviewer_runtime_setup(self):
        fake_pi = self.root / "pi-newer"
        fake_pi.write_text("#!/bin/sh\nprintf '0.99.2\\n'\n")
        fake_pi.chmod(0o700)
        args = spawn_args(self.cwd, self.scratch)
        fake = FakeHerdr(self.cwd)
        with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
            with self.assertRaisesRegex(zhoor_herdr.LaneError, "Pi version is not the audited 0.99.1 baseline"):
                zhoor_herdr.spawn_agent(args, runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))
        self.assertFalse((self.scratch / "home").exists())

    def test_missing_approved_provider_auth_fails_before_tab_creation(self):
        self.auth_store.unlink()
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "dedicated reviewer OAuth profile.*invalid"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))

    def test_caller_cannot_supply_system_prompt_or_open_fd_as_prompt_file(self):
        args = spawn_args(self.cwd, self.scratch)
        args.agent_args += ["--system-prompt", "/proc/self/fd/3"]
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "unapproved Pi launch option"):
            zhoor_herdr.spawn_agent(args, runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))

    def test_nonempty_scratch_cannot_be_reused(self):
        marker = self.scratch / "stale-lane-data"
        marker.write_text("preserve this file")
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "fresh empty scratch"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertEqual(marker.read_text(), "preserve this file")
        self.assertEqual(fake.calls, [])

    def test_symlink_scratch_root_is_rejected_without_writing_through_it(self):
        outside = self.root / "outside-scratch"
        outside.mkdir(mode=0o700)
        alias = self.root / "scratch-alias"
        alias.symlink_to(outside, target_is_directory=True)
        args = spawn_args(self.cwd, alias)
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "scratch directory must not contain symlink components"):
            zhoor_herdr.spawn_agent(args, runner=fake, registry_path=self.registry)
        self.assertEqual(list(outside.iterdir()), [])
        self.assertEqual(fake.calls, [])

    def test_reviewer_rejects_descendant_cwd_before_creating_tab(self):
        nested_cwd = self.cwd / "nested"
        nested_cwd.mkdir()
        args = spawn_args(self.cwd, self.scratch)
        args.cwd = str(nested_cwd)
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "Pi review CWD must equal assigned worktree root"):
            zhoor_herdr.spawn_agent(args, runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))

    def test_legacy_openai_codex_provider_is_rejected(self):
        auth = json.loads(self.auth_store.read_text())
        auth["openai-codex"] = {
            "type": "oauth", "access": "synthetic-legacy-access",
            "refresh": "synthetic-legacy-refresh", "expires": 2_000_000_000_000,
        }
        self.auth_store.write_text(json.dumps(auth))
        args = spawn_args(self.cwd, self.scratch)
        args.agent_args[args.agent_args.index("--provider") + 1] = "openai-codex"
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "does not match audit tier"):
            zhoor_herdr.spawn_agent(args, runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))

    def test_openai_api_key_auth_is_rejected(self):
        credential = dict(self.oauth_credential, type="api_key", key="synthetic-test-api-key")
        self.assert_invalid_openai_credential_is_rejected(credential)

    def test_openai_oauth_without_client_id_is_rejected(self):
        credential = dict(self.oauth_credential)
        credential.pop("clientId")
        self.assert_invalid_openai_credential_is_rejected(credential)

    def test_openai_oauth_without_chatgpt_direct_scope_is_rejected(self):
        credential = dict(self.oauth_credential, scopes=["openid"])
        self.assert_invalid_openai_credential_is_rejected(credential)

    def test_openai_oauth_with_boolean_expiry_is_rejected(self):
        credential = dict(self.oauth_credential, expires=True)
        self.assert_invalid_openai_credential_is_rejected(credential)

    def test_openai_oauth_with_malformed_access_token_is_rejected(self):
        credential = dict(self.oauth_credential, access=17)
        self.assert_invalid_openai_credential_is_rejected(credential)

    def test_reviewer_profile_rejects_unrelated_provider_credentials(self):
        self.auth_store.write_text(json.dumps({
            "openai": self.oauth_credential,
            "openai-codex": self.oauth_credential,
        }))
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "dedicated reviewer OAuth profile.*invalid"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))

    def test_reviewer_profile_inside_assigned_worktree_is_rejected_before_tab_creation(self):
        profile_dir = self.cwd / ".reviewer-profile"
        profile_dir.mkdir(mode=0o700)
        auth_store = profile_dir / "auth.json"
        auth_store.write_text(json.dumps({"openai": self.oauth_credential}))
        os.chmod(profile_dir, 0o700)
        os.chmod(auth_store, 0o600)
        fake = FakeHerdr(self.cwd)
        with patch.object(zhoor_herdr, "REVIEWER_PROFILE_DIR", profile_dir), patch.object(
            zhoor_herdr, "REVIEWER_AUTH_STORE", auth_store
        ):
            with self.assertRaisesRegex(zhoor_herdr.LaneError, "dedicated reviewer OAuth profile.*invalid"):
                zhoor_herdr.spawn_agent(
                    spawn_args(self.cwd, self.scratch),
                    runner=fake,
                    registry_path=self.registry,
                )
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))
        self.assertEqual(list(self.scratch.iterdir()), [])

    def test_reviewer_profile_requires_private_directory_and_file_modes(self):
        os.chmod(self.profile_dir, 0o755)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "dedicated reviewer OAuth profile.*invalid"):
            zhoor_herdr._validate_reviewer_auth_profile()
        os.chmod(self.profile_dir, 0o700)
        os.chmod(self.auth_store, 0o644)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "dedicated reviewer OAuth profile.*invalid"):
            zhoor_herdr._validate_reviewer_auth_profile()

    def test_reviewer_profile_falls_back_from_temporary_xdg_state(self):
        with patch.dict(os.environ, {"XDG_STATE_HOME": str(self.root)}):
            self.assertEqual(
                zhoor_herdr._reviewer_profile_state_root(),
                Path.home() / ".local/state",
            )

    def test_launcher_handoff_uses_only_the_dedicated_shared_oauth_store(self):
        fake_pi = self.root / "fake-pi"
        fake_pi.write_text('''#!/usr/bin/python3
import json
import fcntl
import os
import sys
from pathlib import Path
if sys.argv[1:] == ["--version"]:
    print("0.99.1")
    raise SystemExit(0)
auth_path = Path(os.environ["PI_CODING_AGENT_DIR"]) / "auth.json"
credential = json.loads(auth_path.read_text())["openai"]
lock_probe = auth_path.open("rb")
try:
    fcntl.flock(lock_probe.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
    profile_lock_held = False
except BlockingIOError:
    profile_lock_held = True
print(json.dumps({
    "type": credential.get("type"),
    "access_present": isinstance(credential.get("access"), str),
    "refresh_present": isinstance(credential.get("refresh"), str),
    "expires_integer": type(credential.get("expires")) is int,
    "client_id_present": bool(credential.get("clientId")),
    "direct_scope_present": "chatgpt.tokens.use.direct" in credential.get("scopes", []),
    "extra_metadata_preserved": credential.get("accountMetadata") == {"tenant": "synthetic-test-tenant"},
    "auth_path_is_symlink": auth_path.is_symlink(),
    "auth_target": os.readlink(auth_path),
    "profile_lock_held": profile_lock_held,
    "argv_has_synthetic_token": any("synthetic-test-" in value for value in sys.argv),
    "environment_has_synthetic_token": any("synthetic-test-" in value for value in os.environ.values()),
    "openai_api_key_env_present": "OPENAI_API_KEY" in os.environ,
}))
''')
        fake_pi.chmod(0o700)
        args = spawn_args(self.cwd, self.scratch)
        with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
            zhoor_herdr._prepare_pi_containment(
                args.agent_args[1:], route=zhoor_herdr.resolve_audit_route(args.audit_tier),
                scratch=self.scratch,
            )
        wrapper = self.scratch / "bin" / "pi"
        result = subprocess.run([str(wrapper)], text=True, capture_output=True, check=False)
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertEqual(report["type"], "oauth")
        self.assertTrue(report["access_present"])
        self.assertTrue(report["refresh_present"])
        self.assertTrue(report["expires_integer"])
        self.assertTrue(report["client_id_present"])
        self.assertTrue(report["direct_scope_present"])
        self.assertTrue(report["extra_metadata_preserved"])
        self.assertTrue(report["auth_path_is_symlink"])
        self.assertEqual(report["auth_target"], str(self.auth_store))
        self.assertTrue(report["profile_lock_held"])
        self.assertFalse(report["argv_has_synthetic_token"])
        self.assertFalse(report["environment_has_synthetic_token"])
        self.assertFalse(report["openai_api_key_env_present"])

    def test_launcher_refresh_persists_to_authoritative_reviewer_store(self):
        fake_pi = self.root / "fake-pi-refresh"
        fake_pi.write_text('''#!/usr/bin/python3
import json
import os
import sys
from pathlib import Path
if sys.argv[1:] == ["--version"]:
    print("0.99.1")
    raise SystemExit(0)
auth_path = Path(os.environ["PI_CODING_AGENT_DIR"]) / "auth.json"
auth = json.loads(auth_path.read_text())
credential = auth["openai"]
credential["access"] = "synthetic-refreshed-access"
credential["expires"] += 60_000
auth_path.write_text(json.dumps(auth))
reloaded = json.loads(auth_path.read_text())["openai"]
print(json.dumps({
    "refresh_write_succeeded": reloaded.get("access") == "synthetic-refreshed-access",
    "refresh_expiry_advanced": reloaded.get("expires") == credential.get("expires"),
    "auth_path_remains_symlink": auth_path.is_symlink(),
}))
''')
        fake_pi.chmod(0o700)
        args = spawn_args(self.cwd, self.scratch)
        with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
            zhoor_herdr._prepare_pi_containment(
                args.agent_args[1:], route=zhoor_herdr.resolve_audit_route(args.audit_tier),
                scratch=self.scratch,
            )
        result = subprocess.run(
            [str(self.scratch / "bin" / "pi")], text=True,
            capture_output=True, check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertTrue(report["refresh_write_succeeded"])
        self.assertTrue(report["refresh_expiry_advanced"])
        self.assertTrue(report["auth_path_remains_symlink"])
        self.assertEqual(json.loads(self.auth_store.read_text())["openai"]["access"], "synthetic-refreshed-access")

    def test_launcher_rechecks_oauth_structure_before_exec(self):
        fake_pi = self.root / "fake-pi"
        executed = self.root / "fake-pi-executed"
        fake_pi.write_text(
            "#!/usr/bin/python3\nimport sys\n"
            "if sys.argv[1:] == ['--version']:\n"
            "    print('0.99.1')\n"
            "    raise SystemExit(0)\n"
            "from pathlib import Path\n"
            f"Path({str(executed)!r}).write_text('executed')\n"
        )
        fake_pi.chmod(0o700)
        args = spawn_args(self.cwd, self.scratch)
        with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
            zhoor_herdr._prepare_pi_containment(
                args.agent_args[1:], route=zhoor_herdr.resolve_audit_route(args.audit_tier),
                scratch=self.scratch,
            )
        credential = dict(self.oauth_credential, type="api_key")
        self.auth_store.write_text(json.dumps({"openai": credential}))
        result = subprocess.run(
            [str(self.scratch / "bin" / "pi")], text=True,
            capture_output=True, check=False,
        )
        self.assertEqual(result.returncode, 125)
        self.assertEqual(result.stderr.strip(), "ZHOOR reviewer OAuth profile is invalid or its lock failed")
        self.assertFalse(executed.exists())

    def test_non_openai_provider_is_rejected(self):
        auth = {"wrong-provider": self.oauth_credential}
        self.auth_store.write_text(json.dumps(auth))
        args = spawn_args(self.cwd, self.scratch)
        args.agent_args[args.agent_args.index("--provider") + 1] = "wrong-provider"
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "does not match audit tier"):
            zhoor_herdr.spawn_agent(args, runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))

    def test_one_interactive_root_without_sidebar_passes(self):
        fake = FakeHerdr(self.cwd, panes=1)
        lane = zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertEqual(lane["interactive_pane_ids"], ["root-pane"])
        self.assertEqual(lane["managed_ui_pane_ids"], [])

    def test_duplicate_live_role_is_rejected_before_cli_calls(self):
        self.registry.parent.mkdir(parents=True)
        self.registry.write_text(json.dumps({"version": 1, "lanes": [{"logical_role": "ARCH", "agent_name": "older", "status": "WORKING"}]}))
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "already has a live lane"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertEqual(fake.calls, [])

    def test_quarantined_role_can_be_retried_in_a_fresh_lane(self):
        self.registry.parent.mkdir(parents=True)
        self.registry.write_text(json.dumps({"version": 1, "lanes": [{"logical_role": "ARCH", "agent_name": "pro01-arch-review", "status": "QUARANTINED"}]}))
        fake = FakeHerdr(self.cwd, panes=1)
        lane = zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertEqual(lane["status"], "ROLE_HANDSHAKE_PENDING")
        self.assertTrue(any(call[:3] == ["herdr", "agent", "start"] for call in fake.calls))

    def test_pi_spawn_rejects_shell_tool_before_creating_tab(self):
        args = spawn_args(self.cwd, self.scratch)
        args.agent_args[args.agent_args.index("read,grep,find,ls")] = "read,grep,find,ls,bash"
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "exactly the read-only file tools"):
            zhoor_herdr.spawn_agent(args, runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))

    def test_completed_retry_ignores_prior_topology_failed_lane(self):
        self.registry.parent.mkdir(parents=True)
        self.registry.write_text(json.dumps({"version": 1, "lanes": [
            {"logical_role": "TEST", "agent_name": None, "tab_id": "old-tab", "status": "TOPOLOGY_FAILED"},
            {"logical_role": "TEST", "agent_name": None, "tab_id": "new-tab", "status": "IDLE"},
        ]}))
        evidence = self.root / "result.md"
        evidence.write_text("verified result")
        lane = zhoor_herdr.complete_lane("TEST", str(evidence), registry_path=self.registry)
        self.assertEqual(lane["tab_id"], "new-tab")
        self.assertEqual(lane["status"], "COMPLETED")
        lanes = json.loads(self.registry.read_text())["lanes"]
        self.assertEqual(lanes[0]["status"], "TOPOLOGY_FAILED")

    def test_tab_creation_failure_never_falls_back_to_split(self):
        fake = FakeHerdr(self.cwd, fail_create=True)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, r"\(tab create\): exit 1"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertFalse(any("split" in call for call in fake.calls))
        self.assertFalse(any(call[:3] == ["herdr", "agent", "start"] for call in fake.calls))

    def test_two_interactive_panes_fail_closed(self):
        fake = FakeHerdr(self.cwd, panes=2, extra_kind="interactive")
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "exactly one assigned interactive pane"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        lane = json.loads(self.registry.read_text())["lanes"][0]
        self.assertEqual(lane["status"], "TOPOLOGY_FAILED")
        self.assertFalse(any(call[:3] == ["herdr", "agent", "start"] for call in fake.calls))

    def test_unknown_second_pane_fails_closed(self):
        fake = FakeHerdr(self.cwd, panes=2, extra_kind="unknown")
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "not positively identified as Herdr UI"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertEqual(json.loads(self.registry.read_text())["lanes"][0]["status"], "TOPOLOGY_FAILED")

    def test_focus_change_is_restored_and_verified(self):
        fake = FakeHerdr(self.cwd, focus_changes=True)
        lane = zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertTrue(lane["focus_changed_on_create"])
        self.assertTrue(lane["focus_restored"])
        self.assertEqual(fake.focused_tab, "controller-tab")
        self.assertIn(["herdr", "tab", "focus", "controller-tab"], fake.calls)

    def test_delayed_focus_change_after_inventory_is_restored_before_agent_start(self):
        fake = FakeHerdr(self.cwd, delayed_focus_change=True)
        lane = zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        start_index = next(i for i, call in enumerate(fake.calls) if call[:3] == ["herdr", "agent", "start"])
        restore_index = max(i for i, call in enumerate(fake.calls[:start_index]) if call[:3] == ["herdr", "tab", "focus"])
        self.assertLess(restore_index, start_index)
        self.assertTrue(lane["focus_changed_on_create"])
        self.assertTrue(lane["focus_restored"])
        self.assertEqual(fake.focused_tab, "controller-tab")

    def test_failed_focus_restoration_blocks_agent_start(self):
        fake = FakeHerdr(self.cwd, focus_changes=True, restore_fails=True)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "focus could not be restored"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        lane = json.loads(self.registry.read_text())["lanes"][0]
        self.assertEqual(lane["status"], "TOPOLOGY_FAILED")
        self.assertFalse(lane["focus_restored"])
        self.assertFalse(any(call[:3] == ["herdr", "agent", "start"] for call in fake.calls))

    def test_utility_tab_uses_same_root_and_managed_ui_classification(self):
        fake = FakeHerdr(self.cwd)
        lane = zhoor_herdr.spawn_utility_tab("TEST", "workspace", "TEST", str(self.cwd), str(self.cwd), SHA, runner=fake, registry_path=self.registry)
        self.assertEqual(lane["interactive_pane_ids"], ["root-pane"])
        self.assertEqual(lane["managed_ui_pane_ids"], ["extra-pane-1"])
        self.assertFalse(any(call[:3] == ["herdr", "agent", "start"] for call in fake.calls))

    def test_utility_tab_rejects_second_interactive_pane(self):
        fake = FakeHerdr(self.cwd, panes=2, extra_kind="interactive")
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "exactly one assigned interactive pane"):
            zhoor_herdr.spawn_utility_tab("TEST", "workspace", "TEST", str(self.cwd), str(self.cwd), SHA, runner=fake, registry_path=self.registry)
        self.assertEqual(json.loads(self.registry.read_text())["lanes"][0]["status"], "TOPOLOGY_FAILED")

    def test_utility_tab_rejects_unknown_second_pane(self):
        fake = FakeHerdr(self.cwd, panes=2, extra_kind="unknown")
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "not positively identified as Herdr UI"):
            zhoor_herdr.spawn_utility_tab("TEST", "workspace", "TEST", str(self.cwd), str(self.cwd), SHA, runner=fake, registry_path=self.registry)
        self.assertEqual(json.loads(self.registry.read_text())["lanes"][0]["status"], "TOPOLOGY_FAILED")
        self.assertFalse(any(call[:3] == ["herdr", "agent", "start"] for call in fake.calls))

    def test_utility_tab_restores_delayed_focus_change_seen_after_inventory(self):
        fake = FakeHerdr(self.cwd, delayed_focus_change=True)
        lane = zhoor_herdr.spawn_utility_tab("TEST", "workspace", "TEST", str(self.cwd), str(self.cwd), SHA, runner=fake, registry_path=self.registry)
        self.assertTrue(lane["focus_changed_on_create"])
        self.assertTrue(lane["focus_restored"])
        self.assertEqual(fake.focused_tab, "controller-tab")

    def test_prompt_is_blocked_until_containment_evidence_is_recorded(self):
        self.registry.parent.mkdir(parents=True)
        self.registry.write_text(json.dumps({"version": 1, "lanes": [{"logical_role": "ARCH", "agent_name": "pro01-arch-review", "status": "WORKING", "containment_status": "PENDING"}]}))
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "before AGENT_CONTAINMENT=PASS"):
            zhoor_herdr.send_prompt("ARCH", "source packet", runner=fake, registry_path=self.registry)
        self.assertEqual(fake.calls, [])
        evidence = self.root / "containment.md"
        evidence.write_text("sanitized test evidence")
        zhoor_herdr.set_containment("ARCH", "PASS", evidence=str(evidence), registry_path=self.registry)
        zhoor_herdr.send_prompt("ARCH", "/thinking low", runner=fake, registry_path=self.registry)
        self.assertEqual(fake.calls[-1][:3], ["herdr", "agent", "prompt"])
        self.assertTrue(fake.calls[-1][-1].startswith("ZHOOR reviewer task text follows as literal user content:\n\n/thinking low"))


class ZhoorInstalledPiAuthTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.worktree = self.root / "worktree"
        self.worktree.mkdir()
        self.scratch = self.root / "scratch"
        self.scratch.mkdir(mode=0o700)
        self.profile_dir = self.root / "reviewer-profile"
        self.profile_dir.mkdir(mode=0o700, parents=True)
        os.chmod(self.profile_dir, 0o700)
        self.auth_store = self.profile_dir / "auth.json"
        self.auth_store.write_text("{}")
        os.chmod(self.auth_store, 0o600)
        self.auth_patch = patch.multiple(
            zhoor_herdr,
            REVIEWER_PROFILE_DIR=self.profile_dir,
            REVIEWER_AUTH_STORE=self.auth_store,
        )
        self.auth_patch.start()

    def tearDown(self):
        self.auth_patch.stop()
        self.temp.cleanup()

    @staticmethod
    def installed_pi_auth_modules():
        pi = Path(shutil.which("pi") or "").resolve()
        package = next(
            (parent for parent in (pi, *pi.parents)
             if (parent / "dist/core/auth-storage.js").is_file()),
            None,
        )
        if package is None:
            raise AssertionError("installed Pi auth storage module was not found")
        auth_storage = package / "dist/core/auth-storage.js"
        pi_ai_resolver = package / "node_modules/@earendil-works/pi-ai/dist/auth/resolve.js"
        if not pi_ai_resolver.is_file():
            raise AssertionError("installed Pi auth resolver module was not found")
        return auth_storage.as_uri(), pi_ai_resolver.as_uri()

    def run_installed_auth_resolution(self):
        auth_storage_uri, resolver_uri = self.installed_pi_auth_modules()
        node_script = self.root / "exercise-installed-auth.mjs"
        node_source = r'''import { existsSync, readlinkSync } from "node:fs";
import { AuthStorage } from __AUTH_STORAGE_URI__;
import { resolveProviderAuth } from __RESOLVER_URI__;
const authPath = process.env.PI_CODING_AGENT_DIR + "/auth.json";
const storage = AuthStorage.create();
let refreshCalls = 0;
const provider = {
  id: "openai",
  auth: { oauth: {
    refresh: async (credential) => {
      refreshCalls++;
      await new Promise((resolve) => setTimeout(resolve, 40));
      return { ...credential, access: "synthetic-refreshed-access", expires: Date.now() + 3600000 };
    },
    toAuth: async (credential) => ({ token: credential.access }),
  } },
};
const context = { env: async () => undefined, fileExists: () => false };
const outcomes = await Promise.allSettled([
  resolveProviderAuth(provider, storage, context),
  resolveProviderAuth(provider, storage, context),
]);
const failedSafely = outcomes.length === 2 && outcomes.every((item) =>
  item.status === "rejected" &&
  String(item.reason?.message ?? "").includes("Credential store modify failed for openai"));
const passed = outcomes.length === 2 && outcomes.every((item) =>
  item.status === "fulfilled" && item.value?.source === "OAuth" &&
  item.value?.auth?.token === "synthetic-refreshed-access");
const persisted = await storage.read("openai");
console.log(JSON.stringify({
  refresh_calls: refreshCalls,
  oauth_results: passed,
  persistence_failed_safely: failedSafely,
  persisted_refreshed_credential: persisted?.access === "synthetic-refreshed-access",
  complete_oauth_structure_preserved: persisted?.clientId === "synthetic-test-client" &&
    persisted?.scopes?.includes("chatgpt.tokens.use.direct") === true &&
    persisted?.accountMetadata?.tenant === "synthetic-test-tenant",
  persisted_refreshed_at_authority: persisted?.access === "synthetic-refreshed-access",
  auth_path_targets_authority: readlinkSync(authPath) === __EXPECTED_AUTH_PATH__,
  lock_removed: !existsSync(authPath + ".lock"),
}));
'''.replace("__AUTH_STORAGE_URI__", json.dumps(auth_storage_uri)).replace(
            "__RESOLVER_URI__", json.dumps(resolver_uri)
        ).replace("__EXPECTED_AUTH_PATH__", json.dumps(str(self.auth_store)))
        node_script.write_text(node_source)
        fake_pi = self.root / "fake-pi"
        fake_pi.write_text(
            "#!/usr/bin/python3\n"
            "import os, subprocess, sys\n"
            "if sys.argv[1:] == ['--version']:\n"
            "    print('0.99.1')\n"
            "    raise SystemExit(0)\n"
            f"result = subprocess.run(['node', {str(node_script)!r}], "
            "text=True, capture_output=True, pass_fds=(3,), check=False)\n"
            "sys.stdout.write(result.stdout)\n"
            "if result.returncode:\n"
            "    print('installed Pi auth integration failed safely', file=sys.stderr)\n"
            "raise SystemExit(result.returncode)\n"
        )
        fake_pi.chmod(0o700)
        credential = {
            "type": "oauth",
            "access": "synthetic-test-access",
            "refresh": "synthetic-test-refresh",
            "expires": 1,
            "clientId": "synthetic-test-client",
            "scopes": ["openid", "chatgpt.tokens.use.direct"],
            "accountMetadata": {"tenant": "synthetic-test-tenant"},
        }
        self.auth_store.write_text(json.dumps({"openai": credential}))
        args = spawn_args(self.worktree, self.scratch)
        with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
            zhoor_herdr._prepare_pi_containment(
                args.agent_args[1:],
                route=zhoor_herdr.resolve_audit_route(args.audit_tier),
                scratch=self.scratch,
            )
        result = subprocess.run(
            [str(self.scratch / "bin" / "pi")],
            text=True,
            capture_output=True,
            env={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"},
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertNotIn("synthetic-test-access", result.stdout)
        self.assertNotIn("synthetic-test-refresh", result.stdout)
        auth_path = self.scratch / "pi-config" / "auth.json"
        self.assertTrue(auth_path.is_symlink())
        self.assertEqual(os.readlink(auth_path), str(self.auth_store))
        self.assertFalse(auth_path.with_name("auth.json.lock").exists())
        return report

    def test_rotating_oauth_refresh_is_shared_across_fresh_reviewer_lanes(self):
        auth_storage_uri, resolver_uri = self.installed_pi_auth_modules()
        profile_dir = self.profile_dir
        server_state = self.root / "fake-oauth-server.json"
        server_state.write_text(json.dumps({
            "accepted_refresh": "synthetic-refresh-R1",
            "refresh_attempts": [],
        }))
        self.auth_store.write_text(json.dumps({"openai": {
            "type": "oauth",
            "access": "synthetic-access-R1",
            "refresh": "synthetic-refresh-R1",
            "expires": 1,
            "clientId": "synthetic-client",
            "scopes": ["openid", "chatgpt.tokens.use.direct"],
        }}))
        os.chmod(self.auth_store, 0o600)

        node_script = self.root / "exercise-rotating-oauth.mjs"
        node_source = r'''import { readFileSync, readlinkSync, writeFileSync } from "node:fs";
import { AuthStorage } from __AUTH_STORAGE_URI__;
import { resolveProviderAuth } from __RESOLVER_URI__;
const authPath = process.env.PI_CODING_AGENT_DIR + "/auth.json";
const storage = AuthStorage.create();
const serverPath = process.env.ZHOOR_TEST_SERVER;
let refreshAttempted = false;
let providerRejection;
const provider = {
  id: "openai",
  auth: { oauth: {
    refresh: async (credential) => {
      refreshAttempted = true;
      const server = JSON.parse(readFileSync(serverPath, "utf8"));
      server.refresh_attempts.push(credential.refresh);
      if (credential.refresh !== server.accepted_refresh) {
        providerRejection = "invalid_grant";
        writeFileSync(serverPath, JSON.stringify(server));
        const error = new Error("invalid_grant");
        error.code = "invalid_grant";
        throw error;
      }
      const match = /^synthetic-refresh-R([1-9][0-9]*)$/.exec(credential.refresh);
      if (!match) {
        providerRejection = "invalid_grant";
        writeFileSync(serverPath, JSON.stringify(server));
        const error = new Error("invalid_grant");
        error.code = "invalid_grant";
        throw error;
      }
      const nextGeneration = `R${Number(match[1]) + 1}`;
      const nextRefresh = `synthetic-refresh-${nextGeneration}`;
      const nextAccess = `synthetic-access-${nextGeneration}`;
      server.accepted_refresh = nextRefresh;
      writeFileSync(serverPath, JSON.stringify(server));
      return { ...credential, access: nextAccess, refresh: nextRefresh,
        expires: Date.now() + 3600000 };
    },
    toAuth: async (credential) => ({ token: credential.access }),
  } },
};
const context = { env: async () => undefined, fileExists: () => false };
let status = "ready";
let errorCode;
try {
  await resolveProviderAuth(provider, storage, context);
} catch (error) {
  status = "error";
  errorCode = providerRejection ?? error?.code ?? "other";
}
const persisted = await storage.read("openai");
console.log(JSON.stringify({
  status,
  error_code: errorCode,
  refresh_attempted: refreshAttempted,
  persisted_refresh: persisted?.refresh,
  persisted_access: persisted?.access,
  session_dir: process.env.PI_CODING_AGENT_SESSION_DIR,
  auth_target: (() => { try { return readlinkSync(authPath); } catch { return null; } })(),
}));
'''.replace("__AUTH_STORAGE_URI__", json.dumps(auth_storage_uri)).replace(
            "__RESOLVER_URI__", json.dumps(resolver_uri)
        ).replace("__EXPECTED_AUTH_PATH__", json.dumps(str(self.auth_store)))
        node_script.write_text(node_source)

        def run_lane(scratch: Path, role: str) -> dict:
            fake_pi = self.root / f"fake-pi-{role}"
            fake_pi.write_text(
                "#!/usr/bin/python3\n"
                "import os, subprocess, sys\n"
                "if sys.argv[1:] == ['--version']:\n"
                "    print('0.99.1')\n"
                "    raise SystemExit(0)\n"
                "env = dict(os.environ, ZHOOR_TEST_SERVER=" + repr(str(server_state)) + ")\n"
                "result = subprocess.run(['node', " + repr(str(node_script)) + "], "
                "text=True, capture_output=True, pass_fds=(3,), env=env, check=False)\n"
                "sys.stdout.write(result.stdout)\n"
                "if result.returncode:\n"
                "    print('fake OAuth lane failed safely', file=sys.stderr)\n"
                "raise SystemExit(result.returncode)\n"
            )
            fake_pi.chmod(0o700)
            args = spawn_args(self.worktree, scratch)
            with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
                zhoor_herdr._prepare_pi_containment(
                    args.agent_args[1:],
                    route=zhoor_herdr.resolve_audit_route(args.audit_tier),
                    scratch=scratch,
                )
            result = subprocess.run(
                [str(scratch / "bin" / "pi")],
                text=True,
                capture_output=True,
                env={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"},
                check=False,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            return json.loads(result.stdout)

        scratch_b = self.root / "scratch-b"
        scratch_b.mkdir(mode=0o700)
        lane_a = run_lane(self.scratch, "a")
        lane_b = run_lane(scratch_b, "b")

        server = json.loads(server_state.read_text())
        source = json.loads(self.auth_store.read_text())["openai"]
        self.assertEqual(lane_a["status"], "ready")
        self.assertTrue(lane_a["refresh_attempted"])
        self.assertEqual(lane_a["persisted_refresh"], "synthetic-refresh-R2")
        self.assertEqual(lane_a["persisted_access"], "synthetic-access-R2")
        self.assertEqual(lane_b["status"], "ready", "lane B attempted stale R1 and got invalid_grant")
        self.assertEqual(lane_b["persisted_refresh"], "synthetic-refresh-R2")
        self.assertEqual(lane_b["persisted_access"], "synthetic-access-R2")
        self.assertFalse(lane_b["refresh_attempted"], "fresh lane B must reuse R2 without refreshing R1")
        self.assertNotEqual(lane_a["session_dir"], lane_b["session_dir"])
        self.assertEqual(lane_a["auth_target"], str(self.auth_store))
        self.assertEqual(lane_b["auth_target"], str(self.auth_store))
        self.assertEqual(server["accepted_refresh"], "synthetic-refresh-R2")
        self.assertEqual(server["refresh_attempts"], ["synthetic-refresh-R1"])
        self.assertEqual(source["refresh"], "synthetic-refresh-R2")
        self.assertEqual(source["access"], "synthetic-access-R2")

        scratch_c = self.root / "scratch-c"
        scratch_c.mkdir(mode=0o700)
        expired_r2 = json.loads(self.auth_store.read_text())["openai"]
        expired_r2["expires"] = 1
        self.auth_store.write_text(json.dumps({"openai": expired_r2}))
        lane_c = run_lane(scratch_c, "c")
        server = json.loads(server_state.read_text())
        source = json.loads(self.auth_store.read_text())["openai"]
        self.assertEqual(lane_c["status"], "ready")
        self.assertTrue(lane_c["refresh_attempted"])
        self.assertEqual(lane_c["persisted_refresh"], "synthetic-refresh-R3")
        self.assertEqual(lane_c["persisted_access"], "synthetic-access-R3")
        self.assertEqual(server["accepted_refresh"], "synthetic-refresh-R3")
        self.assertEqual(server["refresh_attempts"], ["synthetic-refresh-R1", "synthetic-refresh-R2"])
        self.assertEqual(source["refresh"], "synthetic-refresh-R3")
        self.assertEqual(source["access"], "synthetic-access-R3")

        # Reproduce the pre-fix design: both private stores begin with R1.
        # Lane A rotates the server's credential to R2 but can persist only to
        # its copy. Lane B's stale R1 is then rejected with invalid_grant.
        server_state.write_text(json.dumps({"accepted_refresh": "synthetic-refresh-R1", "refresh_attempts": []}))
        stale_r1 = {
            "type": "oauth", "access": "synthetic-access-R1",
            "refresh": "synthetic-refresh-R1", "expires": 1,
            "clientId": "synthetic-client",
            "scopes": ["openid", "chatgpt.tokens.use.direct"],
        }
        self.auth_store.write_text(json.dumps({"openai": stale_r1}))

        def run_private_copy(config_dir: Path, role: str) -> dict:
            config_dir.mkdir(mode=0o700)
            copied_auth = config_dir / "auth.json"
            copied_auth.write_text(json.dumps({"openai": stale_r1}))
            os.chmod(copied_auth, 0o600)
            env = {
                "PATH": "/usr/bin:/bin",
                "PI_CODING_AGENT_DIR": str(config_dir),
                "PI_CODING_AGENT_SESSION_DIR": str(self.root / f"copy-session-{role}"),
                "ZHOOR_TEST_SERVER": str(server_state),
            }
            result = subprocess.run(
                [shutil.which("node"), str(node_script)],
                env=env, text=True, capture_output=True, check=False,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            return json.loads(result.stdout)

        copy_a = run_private_copy(self.root / "private-copy-a", "a")
        copy_b = run_private_copy(self.root / "private-copy-b", "b")
        server = json.loads(server_state.read_text())
        source = json.loads(self.auth_store.read_text())["openai"]
        self.assertEqual(copy_a["status"], "ready")
        self.assertEqual(copy_a["persisted_refresh"], "synthetic-refresh-R2")
        self.assertEqual(copy_b["status"], "error")
        self.assertEqual(copy_b["error_code"], "invalid_grant")
        self.assertTrue(copy_b["refresh_attempted"])
        self.assertEqual(copy_b["persisted_refresh"], "synthetic-refresh-R1")
        self.assertEqual(server["accepted_refresh"], "synthetic-refresh-R2")
        self.assertEqual(server["refresh_attempts"], ["synthetic-refresh-R1", "synthetic-refresh-R1"])
        self.assertEqual(source["refresh"], "synthetic-refresh-R1")

    def test_installed_pi_auth_storage_refreshes_concurrently_at_authority(self):
        report = self.run_installed_auth_resolution()
        self.assertEqual(report["refresh_calls"], 1)
        self.assertTrue(report["oauth_results"])
        self.assertTrue(report["persisted_refreshed_credential"])
        self.assertTrue(report["complete_oauth_structure_preserved"])
        self.assertTrue(report["auth_path_targets_authority"])
        self.assertTrue(report["persisted_refreshed_at_authority"])
        self.assertTrue(report["lock_removed"])

    def test_authoritative_profile_file_is_private(self):
        self.run_installed_auth_resolution()
        self.assertTrue(self.auth_store.is_file())
        self.assertEqual(stat.S_IMODE(self.auth_store.stat().st_mode), 0o600)


class ZhoorReadonlyGuardTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.worktree = self.root / "worktree"
        self.worktree.mkdir()
        (self.worktree / "inside.txt").write_text("inside\n")
        self.outside = self.root / "outside.txt"
        self.outside.write_text("outside\n")
        (self.worktree / "escape.txt").symlink_to(self.outside)

    def tearDown(self):
        self.temp.cleanup()

    def run_guard(self, event_name: str, event: dict[str, object]) -> dict[str, object]:
        guard_url = (Path(__file__).parents[1] / "scripts/zhoor_pi_readonly_guard.ts").as_uri()
        script = f'''import guard from {json.dumps(guard_url)};
import {{ readFileSync }} from "node:fs";
const handlers = new Map();
guard({{ on: (name, handler) => handlers.set(name, handler) }});
const eventName = process.argv[1];
const event = JSON.parse(readFileSync(0, "utf8"));
const handler = handlers.get(eventName);
const result = handler ? await handler(event) : undefined;
console.log(JSON.stringify({{ registered: [...handlers.keys()], result: result ?? null }}));
'''
        result = subprocess.run(
            ["node", "--experimental-strip-types", "--input-type=module", "-e", script, event_name],
            cwd=self.worktree,
            input=json.dumps(event),
            text=True,
            capture_output=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        return json.loads(result.stdout)

    def test_user_bash_and_double_bang_return_exit_126_without_execution(self):
        for exclude_from_context in (False, True):
            with self.subTest(exclude_from_context=exclude_from_context):
                report = self.run_guard("user_bash", {
                    "type": "user_bash",
                    "command": "printf ZHOOR_UNIT_USER_BASH_SENTINEL",
                    "excludeFromContext": exclude_from_context,
                    "cwd": str(self.worktree),
                })
                self.assertIn("user_bash", report["registered"])
                self.assertEqual(report["result"], {
                    "result": {
                        "output": "ZHOOR read-only containment denied user shell execution",
                        "exitCode": 126,
                        "cancelled": False,
                        "truncated": False,
                    },
                })

    def test_read_grep_find_and_ls_inside_worktree_are_allowed(self):
        for tool_name in ("read", "grep", "find", "ls"):
            with self.subTest(tool_name=tool_name):
                report = self.run_guard("tool_call", {
                    "type": "tool_call",
                    "toolName": tool_name,
                    "input": {"path": "inside.txt"},
                })
                self.assertIsNone(report["result"])

    def test_write_edit_and_model_bash_tools_are_blocked(self):
        for tool_name in ("write", "edit", "bash"):
            with self.subTest(tool_name=tool_name):
                report = self.run_guard("tool_call", {
                    "type": "tool_call",
                    "toolName": tool_name,
                    "input": {"path": "inside.txt"},
                })
                self.assertTrue(report["result"]["block"])

    def test_read_outside_worktree_is_blocked(self):
        report = self.run_guard("tool_call", {
            "type": "tool_call",
            "toolName": "read",
            "input": {"path": str(self.outside)},
        })
        self.assertTrue(report["result"]["block"])

    def test_symlink_escape_read_is_blocked(self):
        report = self.run_guard("tool_call", {
            "type": "tool_call",
            "toolName": "read",
            "input": {"path": "escape.txt"},
        })
        self.assertTrue(report["result"]["block"])


class ZhoorInstalledFileToolIntegrationTests(unittest.TestCase):
    """Exercise the guard with the actual installed Pi 0.99.1 file tools."""

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.worktree = self.root / "worktree"
        self.home = self.root / "home"
        self.config = self.root / "pi-config"
        self.reviewer_profile = self.root / "reviewer-profile"
        self.worktree.mkdir()
        self.home.mkdir()
        self.config.mkdir()
        self.reviewer_profile.mkdir(mode=0o700)
        self.reviewer_auth = self.reviewer_profile / "auth.json"
        self.reviewer_auth.write_text("reviewer-profile-secret-sentinel\n")
        os.chmod(self.reviewer_auth, 0o600)
        node = shutil.which("node")
        pi = shutil.which("pi")
        if not node or not pi:
            self.skipTest("installed Node.js and Pi are required for Pi file-tool integration")
        self.node = node
        package_root = Path(pi).resolve().parents[2]
        tools_module = package_root / "dist/core/tools/index.js"
        if not tools_module.is_file():
            self.skipTest("installed Pi file-tool module is unavailable")
        self.tools_module = tools_module.as_uri()
        self.guard_module = (Path(__file__).parents[1] / "scripts/zhoor_pi_readonly_guard.ts").as_uri()

    def tearDown(self):
        self.temp.cleanup()

    def run_installed_tools(self, cases):
        script = f'''import guard from {json.dumps(self.guard_module)};
import {{ createReadToolDefinition, createGrepToolDefinition, createFindToolDefinition, createLsToolDefinition }} from {json.dumps(self.tools_module)};
import {{ readFileSync }} from "node:fs";
const input = JSON.parse(readFileSync(0, "utf8"));
const handlers = new Map();
guard({{ on: (name, handler) => handlers.set(name, handler) }});
const definitions = new Map([
  ["read", createReadToolDefinition(input.cwd)],
  ["grep", createGrepToolDefinition(input.cwd)],
  ["find", createFindToolDefinition(input.cwd)],
  ["ls", createLsToolDefinition(input.cwd)],
]);
const guardToolCall = handlers.get("tool_call");
const reports = [];
for (const item of input.cases) {{
  const decision = await guardToolCall({{ type: "tool_call", toolName: item.tool, input: item.input }});
  if (decision?.block) {{
    reports.push({{ blocked: true, expectedFound: false, forbiddenFound: false }});
    continue;
  }}
  try {{
    const result = await definitions.get(item.tool).execute("zhoor-integration", item.input, undefined, undefined, {{ cwd: input.cwd }});
    const text = (result?.content ?? []).filter((part) => typeof part.text === "string").map((part) => part.text).join("\\n");
    reports.push({{
      blocked: false,
      expectedFound: item.expectedText === undefined ? null : text.includes(item.expectedText),
      forbiddenFound: item.forbiddenText === undefined ? false : text.includes(item.forbiddenText),
    }});
  }} catch (error) {{
    reports.push({{ blocked: false, toolError: error instanceof Error ? error.name : "unknown" }});
  }}
}}
console.log(JSON.stringify(reports));
'''
        tool_directories = []
        for tool in ("rg", "fd"):
            tool_path = shutil.which(tool)
            if tool_path:
                tool_directories.append(str(Path(tool_path).parent))
        path = os.pathsep.join(dict.fromkeys([*tool_directories, "/usr/bin", "/bin"]))
        env = {
            "PATH": path,
            "HOME": str(self.home),
            "TMPDIR": str(self.root),
            "PI_CODING_AGENT_DIR": str(self.config),
            "PI_OFFLINE": "1",
            "LANG": "C.UTF-8",
        }
        result = subprocess.run(
            [self.node, "--experimental-strip-types", "--input-type=module", "-e", script],
            cwd=self.worktree,
            input=json.dumps({"cwd": str(self.worktree), "cases": cases}),
            text=True,
            capture_output=True,
            env=env,
            timeout=20,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        return json.loads(result.stdout)

    def test_installed_read_tool_cannot_escape_via_pi_path_normalization(self):
        at_outside = self.root / "outside-at.txt"
        at_outside.write_text("ZHOOR_AT_PREFIX_OUTSIDE_SENTINEL\n")
        at_path = "@" + str(at_outside)
        at_decoy = self.worktree / at_path
        at_decoy.parent.mkdir(parents=True)
        at_decoy.write_text("inside decoy\n")

        tilde_outside = self.home / "outside-tilde.txt"
        tilde_outside.write_text("ZHOOR_TILDE_OUTSIDE_SENTINEL\n")
        tilde_decoy = self.worktree / "~" / "outside-tilde.txt"
        tilde_decoy.parent.mkdir()
        tilde_decoy.write_text("inside decoy\n")

        url_outside = self.root / "outside-url.txt"
        url_outside.write_text("ZHOOR_FILE_URL_OUTSIDE_SENTINEL\n")
        url_path = url_outside.as_uri()
        url_decoy = self.worktree / ("file:" + str(url_outside))
        url_decoy.parent.mkdir(parents=True)
        url_decoy.write_text("inside decoy\n")

        cases = [
            {"tool": "read", "input": {"path": at_path}, "forbiddenText": "ZHOOR_AT_PREFIX_OUTSIDE_SENTINEL"},
            {"tool": "read", "input": {"path": "~/outside-tilde.txt"}, "forbiddenText": "ZHOOR_TILDE_OUTSIDE_SENTINEL"},
            {"tool": "read", "input": {"path": url_path}, "forbiddenText": "ZHOOR_FILE_URL_OUTSIDE_SENTINEL"},
        ]
        reports = self.run_installed_tools(cases)
        self.assertEqual(
            [item["blocked"] for item in reports],
            [True, True, True],
            "installed Pi path normalization must not turn an in-worktree-looking input into an outside read",
        )
        self.assertFalse(any(item.get("forbiddenFound", False) for item in reports))

    def test_installed_readonly_tools_allow_inside_and_block_outside_symlink_traversal(self):
        inside = self.worktree / "inside.txt"
        inside.write_text("installed-tool-inside-marker\n")
        outside_dir = self.root / "outside-dir"
        outside_dir.mkdir()
        outside_file = outside_dir / "outside.txt"
        outside_file.write_text("ZHOOR_DIRECT_OUTSIDE_SENTINEL\n")
        nested_file = outside_dir / "nested-marker.txt"
        nested_file.write_text("ZHOOR_RECURSIVE_SYMLINK_SENTINEL\n")
        (self.worktree / "escape.txt").symlink_to(outside_file)
        (self.worktree / "escape-dir").symlink_to(outside_dir, target_is_directory=True)

        if not shutil.which("rg") or not shutil.which("fd"):
            self.skipTest("installed ripgrep and fd are required for recursive Pi file-tool integration")

        cases = [
            {"tool": "read", "input": {"path": "inside.txt"}, "expectedText": "installed-tool-inside-marker"},
            {"tool": "grep", "input": {"path": "inside.txt", "pattern": "installed-tool-inside-marker"}, "expectedText": "installed-tool-inside-marker"},
            {"tool": "find", "input": {"path": ".", "pattern": "inside.txt"}, "expectedText": "inside.txt"},
            {"tool": "ls", "input": {"path": "."}, "expectedText": "inside.txt"},
            {"tool": "read", "input": {"path": str(outside_file)}, "forbiddenText": "ZHOOR_DIRECT_OUTSIDE_SENTINEL"},
            {"tool": "read", "input": {"path": str(self.reviewer_auth)}, "forbiddenText": "reviewer-profile-secret-sentinel"},
            {"tool": "read", "input": {"path": "escape.txt"}, "forbiddenText": "ZHOOR_DIRECT_OUTSIDE_SENTINEL"},
            {"tool": "grep", "input": {"path": ".", "pattern": "ZHOOR_RECURSIVE_SYMLINK_SENTINEL"}, "forbiddenText": "ZHOOR_RECURSIVE_SYMLINK_SENTINEL"},
            {"tool": "find", "input": {"path": ".", "pattern": "nested-marker.txt"}, "forbiddenText": "nested-marker.txt"},
        ]
        reports = self.run_installed_tools(cases)
        self.assertEqual([item["blocked"] for item in reports[:4]], [False] * 4)
        self.assertTrue(all(item.get("expectedFound") for item in reports[:4]))
        self.assertEqual([item["blocked"] for item in reports[4:7]], [True, True, True])
        self.assertEqual([item["blocked"] for item in reports[7:]], [False, False])
        self.assertFalse(any(item.get("forbiddenFound", False) for item in reports[4:]))


class ZhoorInstalledPiCliTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.worktree = self.root / "worktree"
        self.worktree.mkdir()
        self.scratch_runs = 0
        self.profile_dir = self.root / "reviewer-profile"
        self.profile_dir.mkdir(mode=0o700, parents=True)
        os.chmod(self.profile_dir, 0o700)
        self.auth_store = self.profile_dir / "auth.json"
        credential = {
            "type": "oauth",
            "access": "synthetic-cli-access",
            "refresh": "synthetic-cli-refresh",
            "expires": 2_000_000_000_000,
            "clientId": "synthetic-cli-client",
            "scopes": ["openid", "chatgpt.tokens.use.direct"],
        }
        self.auth_store.write_text(json.dumps({"openai": credential}))
        os.chmod(self.auth_store, 0o600)
        self.auth_patch = patch.multiple(
            zhoor_herdr,
            REVIEWER_PROFILE_DIR=self.profile_dir,
            REVIEWER_AUTH_STORE=self.auth_store,
        )
        self.auth_patch.start()

    def tearDown(self):
        self.auth_patch.stop()
        self.temp.cleanup()

    @staticmethod
    def strip_terminal_control(data: bytes) -> str:
        clean = re.sub(rb"\x1b(?:\[[0-?]*[ -/]*[@-~]|\][^\x07]*(?:\x07|\x1b\\))", b"", data)
        return clean.decode("utf-8", errors="replace").replace("\r", "")

    def run_interactive_pi(self, commands, *, guard_source=None, expect_block=True):
        self.scratch_runs += 1
        self.scratch = self.root / f"scratch-{self.scratch_runs}"
        self.scratch.mkdir(mode=0o700)
        args = [
            "--provider", "openai",
            "--model", "gpt-6.1-sol",
            "--thinking", "high",
            "--no-extensions",
            "--no-skills",
            "--no-prompt-templates",
            "--no-themes",
            "--no-context-files",
            "--no-approve",
            "--tools", "read,grep,find,ls",
            "--name", f"zhoor-user-bash-regression-{self.scratch_runs}",
        ]
        sandbox_args, _ = zhoor_herdr._prepare_pi_containment(
            args,
            route=zhoor_herdr.resolve_audit_route("primary"),
            scratch=self.scratch,
        )
        guard_path = Path(sandbox_args[sandbox_args.index("--extension") + 1])
        if guard_source is not None:
            guard_path.chmod(0o600)
            guard_path.write_text(guard_source)
            guard_path.chmod(0o400)

        master, slave = pty.openpty()
        fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 42, 140, 0, 0))
        process = subprocess.Popen(
            [str(self.scratch / "bin" / "pi"), *args, *sandbox_args],
            cwd=self.worktree,
            stdin=slave,
            stdout=slave,
            stderr=slave,
            env={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8", "TERM": "xterm-256color"},
            start_new_session=True,
            close_fds=True,
        )
        os.close(slave)
        output = bytearray()

        def submit_and_receive(command, marker, side_effect, timeout=30):
            deadline = time.monotonic() + timeout
            next_submit = time.monotonic()
            command_submitted = False
            while time.monotonic() < deadline:
                now = time.monotonic()
                if not command_submitted:
                    os.write(master, (command + "\r").encode())
                    command_submitted = True
                    next_submit = now + 2
                ready, _, _ = select.select([master], [], [], 0.1)
                if ready:
                    try:
                        chunk = os.read(master, 65536)
                    except OSError:
                        break
                    if not chunk:
                        break
                    output.extend(chunk)
                    text = self.strip_terminal_control(output)
                    if expect_block and marker in text and "(exit 126)" in text:
                        return
                    if not expect_block and side_effect.exists():
                        return
                # Pi deliberately retains submitted input while startup hooks load.
                # Re-submit the still-visible line until the normal handler is active.
                if time.monotonic() >= next_submit:
                    os.write(master, b"\r")
                    next_submit = time.monotonic() + 2
                if process.poll() is not None:
                    break
            text = self.strip_terminal_control(output)
            safe_tail = re.sub(r"synthetic-[A-Za-z0-9_-]+", "<redacted>", text[-1200:])
            raise AssertionError(
                "installed Pi did not reach the expected interactive result; "
                f"process_exit={process.poll()}, output_tail={safe_tail!r}"
            )

        try:
            # Give the real TUI time to mount before sending key input.
            ready, _, _ = select.select([master], [], [], 2)
            if ready:
                try:
                    output.extend(os.read(master, 65536))
                except OSError:
                    pass
            for command, marker, side_effect in commands:
                submit_and_receive(command, marker, side_effect)
            return self.strip_terminal_control(output)
        finally:
            try:
                os.killpg(process.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            try:
                process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.wait(timeout=3)
            os.close(master)

    def test_installed_pi_cli_blocks_bang_and_double_bang_shell_dispatch(self):
        base_guard = subprocess.run(
            [
                "git", "show",
                "9a9a6ee3d9b1f3bfeb22b9124153226671d6a91b:scripts/zhoor_pi_readonly_guard.ts",
            ],
            cwd=Path(__file__).parents[1],
            text=True,
            capture_output=True,
            check=False,
        )
        self.assertEqual(base_guard.returncode, 0, base_guard.stderr)
        legacy_side_effect = self.root / "legacy-shell-ran"
        denial = "ZHOOR read-only containment denied user shell execution"
        legacy_command = (
            "! /usr/bin/printf ZHOOR_LEGACY_USER_BASH_SENTINEL; "
            f"/usr/bin/touch {legacy_side_effect}"
        )
        legacy_output = self.run_interactive_pi(
            [(legacy_command, denial, legacy_side_effect)],
            guard_source=base_guard.stdout,
            expect_block=False,
        )
        self.assertTrue(legacy_side_effect.exists())
        self.assertNotIn(denial, legacy_output)

        cases = [
            (
                "! /usr/bin/printf ZHOOR_PRIMARY_USER_BASH_SENTINEL; "
                f"/usr/bin/touch {self.root / 'primary-shell-ran'}",
                denial,
                self.root / "primary-shell-ran",
            ),
            (
                "!! /usr/bin/printf ZHOOR_SECONDARY_USER_BASH_SENTINEL; "
                f"/usr/bin/touch {self.root / 'secondary-shell-ran'}",
                denial,
                self.root / "secondary-shell-ran",
            ),
        ]
        for command, denial, side_effect in cases:
            with self.subTest(command=command[:2]):
                output = self.run_interactive_pi([(command, denial, side_effect)])
                self.assertIn(denial, output)
                self.assertGreaterEqual(output.count("(exit 126)"), 1)
                self.assertFalse(side_effect.exists())


class ZhoorAuditRoutingTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.scratch = self.root / "scratch"
        self.scratch.mkdir(mode=0o700)
        self.worktree = self.root / "worktree"
        self.worktree.mkdir()
        self.profile_dir = self.root / "reviewer-profile"
        self.profile_dir.mkdir(mode=0o700, parents=True)
        os.chmod(self.profile_dir, 0o700)
        self.auth_store = self.profile_dir / "auth.json"
        self.auth_store.write_text(json.dumps({"openai": {
            "type": "oauth", "access": "synthetic-routing-access",
            "refresh": "synthetic-routing-refresh", "expires": 2_000_000_000_000,
            "clientId": "synthetic-routing-client",
            "scopes": ["chatgpt.tokens.use.direct"],
        }}))
        os.chmod(self.auth_store, 0o600)
        self.auth_patch = patch.multiple(
            zhoor_herdr,
            REVIEWER_PROFILE_DIR=self.profile_dir,
            REVIEWER_AUTH_STORE=self.auth_store,
        )
        self.auth_patch.start()

    def tearDown(self):
        self.auth_patch.stop()
        self.temp.cleanup()

    @staticmethod
    def native_args(provider="openai", model="gpt-6.1-sol", thinking="high", extra=()):
        return [
            "--provider", provider,
            "--model", model,
            "--thinking", thinking,
            "--no-extensions", "--no-skills", "--no-prompt-templates",
            "--no-themes", "--no-context-files", "--no-approve",
            "--tools", "read,grep,find,ls",
            "--name", "typed-audit-review",
            *extra,
        ]

    def test_primary_and_secondary_routes_are_fixed(self):
        primary = zhoor_herdr.resolve_audit_route("primary")
        secondary = zhoor_herdr.resolve_audit_route("secondary")
        self.assertEqual(
            (primary.audit_tier, primary.provider, primary.model, primary.reasoning),
            ("primary", "openai", "gpt-6.1-sol", "high"),
        )
        self.assertEqual(
            (secondary.audit_tier, secondary.provider, secondary.model, secondary.reasoning),
            ("secondary", "openai", "gpt-6.1-sol", "xhigh"),
        )

    def test_role_name_does_not_select_or_change_audit_tier(self):
        args = spawn_args(self.worktree, self.scratch)
        args.role = "secondary-critical-reviewer"
        lane = zhoor_herdr.spawn_agent(
            args,
            runner=FakeHerdr(self.worktree, panes=1),
            registry_path=self.root / "registry.json",
        )
        self.assertEqual(lane["audit_tier"], "primary")
        self.assertEqual((lane["model"], lane["reasoning"]), ("gpt-6.1-sol", "high"))

    def test_unknown_audit_tier_is_rejected(self):
        with self.assertRaises(zhoor_herdr.LaneError):
            zhoor_herdr.resolve_audit_route("tertiary")

    def test_spawn_parser_accepts_tier_without_route_overrides(self):
        argv = [
            "spawn", "--role", "arbitrary-role", "--name", "reviewer",
            "--kind", "pi", "--workspace", "wH",
            "--cwd", str(self.worktree), "--worktree", str(self.worktree),
            "--scratch", str(self.scratch), "--starting-sha", SHA,
            "--label", "review lane", "--audit-tier", "primary", "--",
            *self.native_args(),
        ]
        args = zhoor_herdr.parser().parse_args(argv)
        self.assertEqual(args.audit_tier, "primary")
        self.assertFalse(hasattr(args, "provider"))
        self.assertFalse(hasattr(args, "model"))
        self.assertFalse(hasattr(args, "reasoning"))

    def test_tier_rejects_mismatched_provider_model_or_reasoning(self):
        combinations = (
            ("primary", "openai", "gpt-6.1-sol", "xhigh"),
            ("secondary", "openai", "gpt-6.1-sol", "high"),
            ("primary", "openai", "gpt-6-luna", "high"),
            ("primary", "openai-codex", "gpt-6.1-sol", "high"),
        )
        for tier, provider, model, thinking in combinations:
            with self.subTest(tier=tier, provider=provider, model=model, thinking=thinking):
                route = zhoor_herdr.resolve_audit_route(tier)
                with self.assertRaisesRegex(zhoor_herdr.LaneError, "does not match audit tier"):
                    zhoor_herdr._prepare_pi_containment(
                        self.native_args(provider, model, thinking),
                        route=route,
                        scratch=self.scratch,
                    )

    def test_tier_rejects_extensions_mcp_and_model_switch_surfaces(self):
        route = zhoor_herdr.resolve_audit_route("primary")
        for extra in (("-e", "unapproved.ts"), ("--extension", "unapproved.ts"),
                      ("--models", "openai/*"), ("--mcp-config", "mcp.json"),
                      ("--skill", "unapproved-skill"), ("--prompt-template", "template"),
                      ("--theme", "theme")):
            with self.subTest(extra=extra):
                with self.assertRaisesRegex(zhoor_herdr.LaneError, "unapproved Pi launch option"):
                    zhoor_herdr._prepare_pi_containment(
                        self.native_args(extra=extra), route=route, scratch=self.scratch,
                    )


if __name__ == "__main__":
    unittest.main()
