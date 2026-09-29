from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace

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
        provider="openai-codex",
        model="GPT-6 Luna",
        reasoning="xhigh",
        agent_args=["--", "--model", "provider/model", "--thinking", "xhigh", "--tools", "read,grep,find,ls"],
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

    def tearDown(self):
        self.temp.cleanup()

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
        self.assertFalse(any(call[:2] == ["herdr", "pane"] and "split" in call for call in fake.calls))

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


if __name__ == "__main__":
    unittest.main()
