from __future__ import annotations

import json
import os
import subprocess
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch

from scripts import zhoor_herdr


SHA = "480d4e801e53480beced1a609622a027d3806828"


class FakeHerdr:
    def __init__(self, cwd: Path, *, panes: int = 2, extra_kind: str = "sidebar", focus_changes: bool = False, delayed_focus_change: bool = False, restore_fails: bool = False, fail_create: bool = False):
        self.cwd = cwd
        self.pane_count = panes
        self.extra_kind = extra_kind
        self.focus_changes = focus_changes
        self.delayed_focus_change = delayed_focus_change
        self.restore_fails = restore_fails
        self.fail_create = fail_create
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
            if self.created and self.delayed_focus_change:
                self.focused_tab = "new-tab"
                self.delayed_focus_change = False
            panes = []
            if self.created:
                panes = [{"tab_id": "new-tab", "pane_id": "root-pane", "cwd": str(self.cwd)}]
                for idx in range(1, self.pane_count):
                    extra = {"tab_id": "new-tab", "pane_id": f"extra-pane-{idx}", "cwd": str(self.cwd)}
                    if self.extra_kind == "sidebar":
                        extra.update({"label": "Sidebar", "tokens": {"herdr-sidebar-explorer": "test"}})
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
        workspace="workspace",
        cwd=str(cwd),
        worktree=str(cwd),
        scratch=str(scratch),
        starting_sha=SHA,
        label="ARCH · Pi · Luna-XH",
        provider="openai",
        model="GPT-6 Luna",
        reasoning="xhigh",
        agent_args=["--", "--provider", "openai", "--model", "gpt-6-luna", "--thinking", "xhigh", "--no-extensions", "--no-skills", "--no-prompt-templates", "--no-themes", "--no-context-files", "--no-approve", "--tools", "read,grep,find,ls", "--name", "pro01-arch-review", "--system-prompt", "Role-only handshake; inspect no source."],
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
        self.auth_store = self.root / "controller-auth.json"
        self.oauth_credential = {
            "access": "synthetic-test-access", "refresh": "synthetic-test-refresh",
            "expires": 2_000_000_000_000, "type": "oauth",
            "clientId": "synthetic-test-client",
            "scopes": ["openid", "chatgpt.tokens.use.direct"],
            "accountMetadata": {"tenant": "synthetic-test-tenant"},
        }
        self.auth_store.write_text(json.dumps({"openai": self.oauth_credential}))
        self.auth_patch = patch.object(zhoor_herdr, "PI_AUTH_STORE", self.auth_store)
        self.auth_patch.start()

    def tearDown(self):
        self.auth_patch.stop()
        self.temp.cleanup()

    def assert_invalid_openai_credential_is_rejected(self, credential):
        self.auth_store.write_text(json.dumps({"openai": credential}))
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "OpenAI ChatGPT OAuth credential is missing or invalid"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))

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
        self.assertEqual(lane["interactive_pane_ids"], ["root-pane"])
        self.assertEqual(lane["managed_ui_pane_ids"], ["extra-pane-1"])
        self.assertFalse(lane["focus_changed_on_create"])
        self.assertTrue(lane["focus_restored"])
        self.assertIn(f"HOME={self.scratch}/home", create)
        self.assertIn(f"TMPDIR={self.scratch}/tmp", create)
        self.assertIn(f"PI_CODING_AGENT_DIR={self.scratch}/pi-config", create)
        self.assertIn(f"PI_CODING_AGENT_SESSION_DIR={self.scratch}/pi-sessions", create)
        start_args = start[start.index("--") + 1:]
        guard_index = start_args.index("--extension")
        guard_path = Path(start_args[guard_index + 1])
        wrapper = self.scratch / "bin" / "pi"
        self.assertTrue(guard_path.is_file())
        self.assertTrue(wrapper.is_file())
        wrapper_text = wrapper.read_text()
        self.assertIn("os.memfd_create", wrapper_text)
        self.assertIn("F_SEAL_WRITE", wrapper_text)
        self.assertIn("os.execve", wrapper_text)
        self.assertTrue((self.scratch / "pi-config" / "auth.json").is_symlink())
        self.assertEqual(os.readlink(self.scratch / "pi-config" / "auth.json"), "/proc/self/fd/3")
        self.assertNotIn("synthetic-test-access", wrapper_text)
        self.assertNotIn("synthetic-test-refresh", wrapper_text)
        self.assertNotIn("AGENT_BROWSER_ENCRYPTION_KEY", wrapper_text)
        self.assertIn(f"PATH={self.scratch}/bin:/usr/bin:/bin", create)
        version = subprocess.run([str(wrapper), "--version"], text=True, capture_output=True, check=False)
        self.assertEqual(version.returncode, 0)
        self.assertIn("0.99.1", version.stdout)
        self.assertFalse(any(call[:2] == ["herdr", "pane"] and "split" in call for call in fake.calls))

    def test_missing_approved_provider_auth_fails_before_tab_creation(self):
        self.auth_store.unlink()
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "OpenAI ChatGPT OAuth credential is missing or invalid"):
            zhoor_herdr.spawn_agent(spawn_args(self.cwd, self.scratch), runner=fake, registry_path=self.registry)
        self.assertFalse(any(call[:3] == ["herdr", "tab", "create"] for call in fake.calls))

    def test_legacy_openai_codex_provider_is_rejected(self):
        auth = json.loads(self.auth_store.read_text())
        auth["openai-codex"] = {
            "type": "oauth", "access": "synthetic-legacy-access",
            "refresh": "synthetic-legacy-refresh", "expires": 2_000_000_000_000,
        }
        self.auth_store.write_text(json.dumps(auth))
        args = spawn_args(self.cwd, self.scratch)
        args.provider = "openai-codex"
        args.agent_args[args.agent_args.index("--provider") + 1] = "openai-codex"
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "only provider=openai"):
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

    def test_launcher_handoff_preserves_full_oauth_object_in_memfd(self):
        fake_pi = self.root / "fake-pi"
        fake_pi.write_text('''#!/usr/bin/python3
import json
import os
import sys
from pathlib import Path
auth_path = Path(os.environ["PI_CODING_AGENT_DIR"]) / "auth.json"
credential = json.loads(auth_path.read_text())["openai"]
print(json.dumps({
    "type": credential.get("type"),
    "access_present": isinstance(credential.get("access"), str),
    "refresh_present": isinstance(credential.get("refresh"), str),
    "expires_integer": type(credential.get("expires")) is int,
    "client_id_present": bool(credential.get("clientId")),
    "direct_scope_present": "chatgpt.tokens.use.direct" in credential.get("scopes", []),
    "extra_metadata_preserved": credential.get("accountMetadata") == {"tenant": "synthetic-test-tenant"},
    "auth_path_is_symlink": auth_path.is_symlink(),
    "fd_is_memfd": "memfd:pi-provider-auth" in os.readlink("/proc/self/fd/3"),
    "argv_has_synthetic_token": any("synthetic-test-" in value for value in sys.argv),
    "environment_has_synthetic_token": any("synthetic-test-" in value for value in os.environ.values()),
    "openai_api_key_env_present": "OPENAI_API_KEY" in os.environ,
}))
''')
        fake_pi.chmod(0o700)
        args = spawn_args(self.cwd, self.scratch)
        with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
            zhoor_herdr._prepare_pi_containment(
                args.agent_args[1:], provider=args.provider, model=args.model,
                reasoning=args.reasoning, scratch=self.scratch,
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
        self.assertTrue(report["fd_is_memfd"])
        self.assertFalse(report["argv_has_synthetic_token"])
        self.assertFalse(report["environment_has_synthetic_token"])
        self.assertFalse(report["openai_api_key_env_present"])

    def test_launcher_rechecks_oauth_structure_before_exec(self):
        fake_pi = self.root / "fake-pi"
        executed = self.root / "fake-pi-executed"
        fake_pi.write_text(f"#!/usr/bin/python3\nfrom pathlib import Path\nPath({str(executed)!r}).write_text('executed')\n")
        fake_pi.chmod(0o700)
        args = spawn_args(self.cwd, self.scratch)
        with patch.object(zhoor_herdr.shutil, "which", return_value=str(fake_pi)):
            zhoor_herdr._prepare_pi_containment(
                args.agent_args[1:], provider=args.provider, model=args.model,
                reasoning=args.reasoning, scratch=self.scratch,
            )
        credential = dict(self.oauth_credential, type="api_key")
        self.auth_store.write_text(json.dumps({"openai": credential}))
        result = subprocess.run(
            [str(self.scratch / "bin" / "pi")], text=True,
            capture_output=True, check=False,
        )
        self.assertEqual(result.returncode, 125)
        self.assertEqual(result.stderr.strip(), "ZHOOR isolated provider credential handoff failed")
        self.assertFalse(executed.exists())

    def test_non_openai_provider_is_rejected(self):
        auth = {"wrong-provider": self.oauth_credential}
        self.auth_store.write_text(json.dumps(auth))
        args = spawn_args(self.cwd, self.scratch)
        args.provider = "wrong-provider"
        args.agent_args[args.agent_args.index("--provider") + 1] = "wrong-provider"
        fake = FakeHerdr(self.cwd)
        with self.assertRaisesRegex(zhoor_herdr.LaneError, "only provider=openai"):
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
        zhoor_herdr.send_prompt("ARCH", "adjudication packet", runner=fake, registry_path=self.registry)
        self.assertEqual(fake.calls[-1][:3], ["herdr", "agent", "prompt"])


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


if __name__ == "__main__":
    unittest.main()
