import path from "node:path";
import { realpathSync } from "node:fs";
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";

export default function (pi: ExtensionAPI) {
  const root = realpathSync(process.cwd());
  const allowed = new Set(["read", "grep", "find", "ls"]);
  const isInsideWorktree = (value: unknown): boolean => {
    if (typeof value !== "string") return false;
    try {
      const resolved = realpathSync(path.resolve(process.cwd(), value));
      return resolved === root || resolved.startsWith(root + path.sep);
    } catch {
      return false;
    }
  };

  pi.on("tool_call", async (event) => {
    if (!allowed.has(event.toolName) || !isInsideWorktree(event.input.path)) {
      return {
        block: true,
        reason: "ARCH review access is limited to read-only files inside the assigned worktree",
      };
    }
    return undefined;
  });
}
