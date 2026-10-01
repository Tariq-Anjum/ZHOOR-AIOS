import path from "node:path";
import { realpathSync } from "node:fs";
import { homedir } from "node:os";
import { fileURLToPath } from "node:url";
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";

export default function (pi: ExtensionAPI) {
  const root = realpathSync(process.cwd());
  const allowed = new Set(["read", "grep", "find", "ls"]);
  const unicodeSpaces = /[\u00A0\u2000-\u200A\u202F\u205F\u3000]/g;
  const resolvePiToolPath = (value: string): string => {
    let normalized = value.replace(unicodeSpaces, " ");
    if (normalized.startsWith("@")) normalized = normalized.slice(1);
    if (normalized === "~") normalized = homedir();
    else if (normalized.startsWith("~/")) normalized = path.join(homedir(), normalized.slice(2));
    if (/^file:\/\//.test(normalized)) normalized = fileURLToPath(normalized);
    return path.isAbsolute(normalized) ? path.resolve(normalized) : path.resolve(root, normalized);
  };
  const isInsideWorktree = (value: unknown): boolean => {
    if (typeof value !== "string") return false;
    try {
      const resolved = realpathSync(resolvePiToolPath(value));
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

  pi.on("user_bash", () => ({
    result: {
      output: "ZHOOR read-only containment denied user shell execution",
      exitCode: 126,
      cancelled: false,
      truncated: false,
    },
  }));
}
