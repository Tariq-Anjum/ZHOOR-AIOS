# Review handoffs

Write one immutable review report per reviewer as `<JOB-ID>-<REVIEWER>.md`. Reviewers must not modify implementation or `../<JOB-ID>.md`. The orchestrator resolves conflicting reviews using tests, deterministic evidence, architecture authority, and human judgment—not model voting.

## Naming

`<REVIEWER>` is a reviewer token, never a role: `herdr-primary`, `herdr-challenger`, `herdr-review`, `sol-final`, or `owner`. The role (`primary`, `independent-challenger`, `interim-primary`) is recorded inside the report using `../../templates/REVIEW_TEMPLATE.md`, because a two-report card cannot have two `-primary` files.

## Independence

- Primary and challenger reports come from different reviewers in independent sessions. A different model/provider family is optional, not an independence requirement.
- An independent challenger writes its report before reading the primary one.
- An agent/model that authored the reviewed tranche cannot act as its independent challenger. There is no waiver for self-review; an author-conflicted reviewer may only provide supplemental, non-independent evidence.
- If the assigned lane is unavailable, use a different reviewer/session with no authorship conflict and record the substitution in `../../DECISIONS.md`. The card stays in `REVIEW` until the owed independent report exists.
