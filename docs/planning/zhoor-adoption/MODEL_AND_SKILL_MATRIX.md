# Model and skill allocation

This is a development-agent allocation and orchestration routing policy, not a ZHOOR product runtime model policy. Herdr Runtime Server is the
continuity surface for every session. Model availability is checked at launch; a missing route does not authorize a containment downgrade.

## Mandatory stack for every agent

1. `skill-router` — inspect the compact index and load only what the job requires.
2. `karpathy-guidelines` — mandatory low-cost quality guardrail for every card:
   - state assumptions;
   - do not overbuild;
   - stay in scope;
   - optimize against the declared success condition.
3. `file-based-planning` — only for jobs likely to cross sessions or produce extensive findings; persist plan/findings/errors.
4. Job-specific capability listed below. Do not load overlapping workflows speculatively.

`grill-me` and `grill-with-docs` are explicit/manual skills. Use them only when the owner directly requests a grilling session. Reviews do not implicitly activate them.

## Runtime roles

| Role | Best use here | Avoid |
|---|---|---|
| `architecture-primary` | Architecture/governance review | Author-conflicted work or final acceptance without owner review |
| `independent-challenger` | Independent challenge in a separate session | Reading the primary report before writing its own; reviewing authored work |
| `implementation-low-cost` | Bounded implementation and reproducible evidence | Unreviewed final acceptance or scope expansion |
| `evaluation-reviewer` | Deterministic evaluation, benchmark evidence, and review | Treating model output as ground truth |
| `security-reviewer` | Security contracts, adversarial fixtures, and boundary review | Authorization or execution authority |
| Human owner | Acceptance, adjudication, scope changes, and promotion decisions | Delegating authority to model output |

## Permanent controller and reviewer routing

These are the owner-approved permanent routes for the controller and independent audit lanes. Verify provider auth, exact model, and reasoning support at session start.

| Stable role | Harness | Provider / authentication | Model | Reasoning |
|---|---|---|---|---|
| Controller / orchestrator / integrator | Codex | Codex session | GPT-6 Luna | xhigh |
| Primary independent auditor | Pi | `openai` / ChatGPT subscription OAuth | GPT-6.1 Sol | high |
| Secondary critical auditor | Pi | `openai` / ChatGPT subscription OAuth | GPT-6.1 Sol | xhigh |

The Herdr helper requires an explicit `--audit-tier primary` or `--audit-tier secondary` and fixes provider, model, and reasoning for that tier. It rejects route overrides, `openai-codex`, and unsupported Pi launch surfaces. `openai-codex` is legacy and is not a permanent ZHOOR reviewer route. Reviewer authentication uses Pi's private `openai` OAuth credential; API-key auth is rejected. The route and model catalog must be live-verified before launch. If unavailable, stop and resolve availability without weakening containment.

The primary auditor is the normal independent reviewer for important implementation cards. Use the secondary critical auditor for security/trust boundaries, authentication/authorization, grant/trust-root, execution/enforcement, privilege, identity, daemon/service lifecycle, transport, filesystem enforcement, architecture/interface changes, important milestone closure, material primary `CHANGES_REQUIRED`, unresolved reviewer/controller disagreement, and new containment/orchestration security changes. This maintenance milestone requires both tiers. Keep the secondary review blind to the primary report until it has formed its own verdict. Workers remain on their existing card-specific routes.

## Job-specific routing

| Jobs | Required additions beyond mandatory stack |
|---|---|
| ARC-02, ARC-03, AUD-01, AUD-02, CANARY-01 | `file-based-planning`; `wayfinder` only for orchestrator dependency changes. ARC-01 uses file planning only if review crosses sessions. |
| ARC-01 (both reports) | No extra skill. Use the `architecture-primary` and `independent-challenger` roles in separate sessions; see current concrete mappings above. |
| PRO-01, SEC-01 | OpenSpec workflow if already available; otherwise use repository schemas/tests directly. Do not install it just for the card. |
| BASE-01, REG-01, POL-01, WRK-01, WRK-02, TRC-01, GRANT-01, AIOSD-01, AIOSD-02 | No extra skill unless the router finds one narrow engineering match |
| EVAL-00 through EVAL-04, SHADOW-01, BENCH-01, BENCH-02 | `file-based-planning` for durable evaluation/benchmark findings; no model-judge skill |

BMAD and OpenSpec are capability pointers, not automatically installed dependencies. BMAD is unnecessary for these already-decomposed cards. OpenSpec is optional only where a schema/spec change benefits from its existing project setup.

## Runtime cost ladder

Development models must not be confused with production routing:

1. Level 0: schema/rule validation—no model.
2. Level 1: registry/index lookup—no model.
3. Level 2: tiny local semantic backend—selected only after `BENCH-02` evidence.
4. Level 3: approved stronger local/remote backend.
5. Level 4: reasoning model with a minimal evidence packet.
6. Level 5: human review.

## Needle and Jev

- **Jev/TypeSafe:** do not install or depend on it. It is a methodology reference and introduces external service/data/credential concerns.
- **Needle:** do not install during architecture, protocol, deterministic worker, trace, or dataset jobs.
- `BENCH-02` may run an isolated Needle benchmark only after the owner approves network/package activity. Pin and inspect the then-current package/version; disable telemetry; use sanitized data; avoid training extras and hosted services.
- Current discussions mention a possible `cactus-needle` package, but this plan intentionally does not freeze an installation command months before the benchmark.

## Optional OpenRouter additions

Do not add another hosted model solely because it is popular or newly ranked. Add a candidate only when an existing job has a measured deficit and the candidate offers a distinct role (for example, independent security review or lower-cost fixture generation). Record provider, exact version, price, privacy/retention terms, rate limits, and benchmark results. Hosted models never become ground truth or authorization authorities.
