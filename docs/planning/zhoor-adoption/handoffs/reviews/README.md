# Review handoffs

Write one immutable review report per reviewer as `<JOB-ID>-<REVIEWER>.md`. Reviewers must not modify implementation or `../<JOB-ID>.md`. The orchestrator resolves conflicting reviews using tests, deterministic evidence, architecture authority, and human judgment—not model voting.

## Naming

`<REVIEWER>` is a reviewer token, never a role: `sol`, `luna`, `qwen-max`, `qwen-flash`, `glm`, `muse`, `deepseek`, `owner`. The role (`primary`, `independent-challenger`, `interim-primary`) is recorded inside the report using `../../templates/REVIEW_TEMPLATE.md`, because a two-report card cannot have two `-primary` files.

## Independence

- Primary and challenger reports for one card come from different model families.
- An independent challenger writes its report before reading the primary one.
- An agent that helped author the artefact under review is never its only reviewer.
- If the assigned lane is unavailable, follow `../../ORCHESTRATOR_RUNBOOK.md` §Lane unavailability and record the substitution in `../../DECISIONS.md`. The card stays in `REVIEW` until the owed independent report exists.
