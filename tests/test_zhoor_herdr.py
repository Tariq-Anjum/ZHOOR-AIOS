from __future__ import annotations

import fcntl
import json
import os
import pty
import re
import select
import shutil
import signal
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
        audit_tier="primary",
        workspace="workspace",
        cwd=str(cwd),
        worktree=str(cwd),
        scratch=str(scratch),
        starting_sha=SHA,
        label="ARCH · Pi · Sol-high",
        agent_args=["--", "--provider", "openai", "--model", "gpt-6.1-sol", "--thinking", "high", "--no-extensions", "--no-skills", "--no-prompt-templates", "--no-themes", "--no-context-files", "--no-approve", "--tools", "read,grep,find,ls", "--name", "pro01-arch-review", "--system-prompt", "Role-only handshake; inspect no source."],
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
        guard_index = start_args.index("--extension")
        guard_path = Path(start_args[guard_index + 1])
        wrapper = self.scratch / "bin" / "pi"
        self.assertTrue(guard_path.is_file())
        self.assertTrue(wrapper.is_file())
        wrapper_text = wrapper.read_text()
        self.assertIn("os.memfd_create", wrapper_text)
        self.assertIn("F_SEAL_SEAL", wrapper_text)
        self.assertNotIn("F_SEAL_WRITE", wrapper_text)
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
        self.assertTrue(report["fd_is_memfd"])
        self.assertFalse(report["argv_has_synthetic_token"])
        self.assertFalse(report["environment_has_synthetic_token"])
        self.assertFalse(report["openai_api_key_env_present"])

    def test_launcher_handoff_allows_pi_to_refresh_oauth_in_anonymous_memfd(self):
        fake_pi = self.root / "fake-pi-refresh"
        fake_pi.write_text('''#!/usr/bin/python3
import json
import os
from pathlib import Path
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
    "fd_remains_memfd": "memfd:pi-provider-auth" in os.readlink("/proc/self/fd/3"),
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
        self.assertTrue(report["fd_remains_memfd"])
        self.assertEqual(json.loads(self.auth_store.read_text())["openai"]["access"], "synthetic-test-access")

    def test_launcher_rechecks_oauth_structure_before_exec(self):
        fake_pi = self.root / "fake-pi"
        executed = self.root / "fake-pi-executed"
        fake_pi.write_text(f"#!/usr/bin/python3\nfrom pathlib import Path\nPath({str(executed)!r}).write_text('executed')\n")
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
        self.assertEqual(result.stderr.strip(), "ZHOOR isolated provider credential handoff failed")
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
        zhoor_herdr.send_prompt("ARCH", "adjudication packet", runner=fake, registry_path=self.registry)
        self.assertEqual(fake.calls[-1][:3], ["herdr", "agent", "prompt"])


class ZhoorInstalledPiAuthTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.worktree = self.root / "worktree"
        self.worktree.mkdir()
        self.scratch = self.root / "scratch"
        self.scratch.mkdir(mode=0o700)
        self.auth_store = self.root / "controller-auth.json"
        self.auth_store.write_text("{}")
        self.auth_patch = patch.object(zhoor_herdr, "PI_AUTH_STORE", self.auth_store)
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

    def run_installed_auth_resolution(self, *, fail_persistence=False):
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
const descriptorTarget = readlinkSync("/proc/self/fd/3");
console.log(JSON.stringify({
  refresh_calls: refreshCalls,
  oauth_results: passed,
  persistence_failed_safely: failedSafely,
  persisted_refreshed_credential: persisted?.access === "synthetic-refreshed-access",
  complete_oauth_structure_preserved: persisted?.clientId === "synthetic-test-client" &&
    persisted?.scopes?.includes("chatgpt.tokens.use.direct") === true &&
    persisted?.accountMetadata?.tenant === "synthetic-test-tenant",
  persisted_original_after_failure: persisted?.access === "synthetic-test-access",
  auth_path_is_fd_symlink: readlinkSync(authPath) === "/proc/self/fd/3",
  descriptor_is_anonymous_memfd: descriptorTarget.includes("memfd:pi-provider-auth"),
  lock_removed: !existsSync(authPath + ".lock"),
}));
'''.replace("__AUTH_STORAGE_URI__", json.dumps(auth_storage_uri)).replace(
            "__RESOLVER_URI__", json.dumps(resolver_uri)
        )
        node_script.write_text(node_source)
        fake_pi = self.root / ("fake-pi-readonly" if fail_persistence else "fake-pi")
        chmod_fd = "os.fchmod(3, 0o400)" if fail_persistence else "pass"
        fake_pi.write_text(
            "#!/usr/bin/python3\n"
            "import os, subprocess, sys\n"
            f"{chmod_fd}\n"
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
        source_bytes_before = self.auth_store.read_bytes()
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
        self.assertEqual(self.auth_store.read_bytes(), source_bytes_before)
        report = json.loads(result.stdout)
        self.assertNotIn("synthetic-test-access", result.stdout)
        self.assertNotIn("synthetic-test-refresh", result.stdout)
        auth_path = self.scratch / "pi-config" / "auth.json"
        self.assertTrue(auth_path.is_symlink())
        self.assertFalse(auth_path.with_name("auth.json.lock").exists())
        return report

    def test_installed_pi_auth_storage_refreshes_concurrently_inside_memfd(self):
        report = self.run_installed_auth_resolution()
        self.assertEqual(report["refresh_calls"], 1)
        self.assertTrue(report["oauth_results"])
        self.assertTrue(report["persisted_refreshed_credential"])
        self.assertTrue(report["complete_oauth_structure_preserved"])
        self.assertTrue(report["auth_path_is_fd_symlink"])
        self.assertTrue(report["descriptor_is_anonymous_memfd"])
        self.assertTrue(report["lock_removed"])

    def test_installed_pi_auth_storage_fails_closed_when_memfd_cannot_be_updated(self):
        report = self.run_installed_auth_resolution(fail_persistence=True)
        self.assertGreaterEqual(report["refresh_calls"], 1)
        self.assertFalse(report["oauth_results"])
        self.assertTrue(report["persistence_failed_safely"])
        self.assertTrue(report["persisted_original_after_failure"])
        self.assertTrue(report["auth_path_is_fd_symlink"])
        self.assertTrue(report["descriptor_is_anonymous_memfd"])
        self.assertTrue(report["lock_removed"])


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


class ZhoorInstalledPiCliTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.worktree = self.root / "worktree"
        self.worktree.mkdir()
        self.scratch_runs = 0
        self.auth_store = self.root / "controller-auth.json"
        credential = {
            "type": "oauth",
            "access": "synthetic-cli-access",
            "refresh": "synthetic-cli-refresh",
            "expires": 2_000_000_000_000,
            "clientId": "synthetic-cli-client",
            "scopes": ["openid", "chatgpt.tokens.use.direct"],
        }
        self.auth_store.write_text(json.dumps({"openai": credential}))
        self.auth_patch = patch.object(zhoor_herdr, "PI_AUTH_STORE", self.auth_store)
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
            "--system-prompt", "Installed Pi user shell containment regression.",
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
        self.auth_store = self.root / "auth.json"
        self.auth_store.write_text(json.dumps({"openai": {
            "type": "oauth", "access": "synthetic-routing-access",
            "refresh": "synthetic-routing-refresh", "expires": 2_000_000_000_000,
            "clientId": "synthetic-routing-client",
            "scopes": ["chatgpt.tokens.use.direct"],
        }}))
        self.auth_patch = patch.object(zhoor_herdr, "PI_AUTH_STORE", self.auth_store)
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
            "--system-prompt", "Read-only independent review.",
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
