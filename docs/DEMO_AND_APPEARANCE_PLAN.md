# Orion — Demo and Appearance Next Steps

**Status:** proposed handoff plan; implementation and runtime verification pending.
**Date:** October 23, 2026.

[NORTH_STAR.md](../NORTH_STAR.md) governs both tracks: focused equipment,
one coherent kit, and only what the user needs. [AESTHETIC_VISION.md](AESTHETIC_VISION.md)
provides the detailed visual direction. This plan prepares a sponsor demonstration,
not a production launch. All steps below remain unchecked.

## Current evidence and boundaries

This is a source review, not a reproduced runtime assessment. Core loan screens,
setup, team management, pricing, AUS, closing, and funding already exist; do not
restart the stale README build checklist. The existing plugin registry and
shared loan components are the foundation, not a reason to build replacements.

| Finding verified in source | Consequence for the next work |
| --- | --- |
| [SetupWizard](../src/screens/SetupWizard.tsx) has no admin step. [create_company](../src-tauri/src/commands/config_commands.rs) creates a company but no user; [create_user](../src-tauri/src/commands/user_commands.rs) requires a signed-in admin. | A clean connected installation lacks a first-admin path. Reproduce and resolve before claiming reproducible setup. |
| [documents.rs](../src-tauri/src/documents.rs) uses `DOC_COLUMNS` with `d`/`u` aliases in INSERT RETURNING; [conditions.rs](../src-tauri/src/conditions.rs) similarly uses `c`/`u`. Neither INSERT binds those aliases. | Suspected real Postgres failures, not runtime reproduced here. Reproduce and fix these paths with database regression tests before demonstrating their persistence. |
| Document commands accept metadata, not file bytes. [ipac.rs](../src-tauri/src/ipac.rs) says credit is not pulled; [stages.ts](../src/screens/loan/stages.ts) has no credit screen. | Present document tracking as metadata only and credit as unintegrated. Check seeded task links: demo data still includes a credit task. |
| [handbooks.rs](../src-tauri/src/handbooks.rs) returns hard-coded summaries. | Label these as illustrative summaries, not authoritative current handbook retrieval or underwriting advice. |
| [mismo.rs](../src-tauri/src/mismo.rs) retains unresolved ULAD markers for originator, REO, and declaration mappings. [aus_commands.rs](../src-tauri/src/commands/aus_commands.rs) uses generic XML POST and [aus.rs](../src-tauri/src/aus.rs) parses responses best-effort. | Endpoint configuration alone does not establish DU/LPA compatibility or certification. Use explicitly mocked findings for the sponsor journey. |
| [LoanListScreen](../src/screens/loan/LoanListScreen.tsx) does not persist built-in demo edits. [locks](../src-tauri/src/locks/mod.rs) use a ten-minute TTL; [AppState](../src-tauri/src/commands/mod.rs) holds one current identity per app state. | Prove saves/reopen against isolated Postgres. Use independent sessions for two-user checks; persona switching in one session is insufficient evidence. |
| [App.tsx](../src/App.tsx) has a routed center, not a general workspace-tab manager. [ChatBox](../src/components/ChatBox.tsx) is AI-only, with screen-based dividers and unconditional local-provider copy. | Tabs and the Slack shell are work to do. Test loan context changes and make provider/status copy truthful. |

## A. Make the sponsor demo repeatable

Work in this order. A later step can be prepared early, but its gate depends
on the preceding evidence.

1. **A1 — Define and isolate the demonstration.**
   - [ ] Write a short presenter script and capability legend: working,
         mocked, metadata-only, disconnected, or deferred.
   - [ ] Establish a dedicated synthetic Postgres database and isolated app
         configuration/profile. Verify the target before any migration or
         reset; never use a live database or real borrower data.
   - [ ] Specify deterministic seed data: company, admin, loan officer,
         underwriter, representative loans, tasks, and an active rate sheet.
   - **Deliverable/gate:** setup and reset recipe with target checks, fixture
     identities, expected starting state, and declared AI/AUS behavior.
     Built-in no-database demo mode remains an explicitly non-persistent fallback.

2. **A2 — Prove clean setup and foundational writes.** Depends on A1.
   - [ ] Reproduce first-admin bootstrap from an empty database and implement
         a narrowly scoped setup path that preserves normal admin restrictions.
   - [ ] Reproduce and correct document/condition INSERT RETURNING behavior;
         verify returned fields, subsequent reads, and corresponding audit rows.
   - [ ] Start from a fresh isolated configuration, run migrations and setup,
         create/sign in users, create a loan, save an Income value, restart,
         and verify the same value on reopen. Capture failures before fixing.
   - **Deliverable/gate:** repeatable clean-install walkthrough plus focused
     Postgres regression tests for bootstrap and the two insert paths. No
     undocumented manual database repair is required for the demonstrated path.

3. **A3 — Exercise one representative loan journey.** Depends on A2.
   - [ ] Walk Home/Loans → borrower and core 1003 facts → Income → underwriter
         review → task/condition → document metadata → pricing → mocked AUS
         → closing/funding records. Use synthetic prepared facts where appropriate;
         clearly distinguish prefilled data from actions performed live.
   - [ ] Verify save/reopen and visible error behavior on demonstrated writes.
         Show saved closing/funding records without implying money movement.
   - [ ] In two independent sessions, verify lock ownership, rejected competing
         writes, read-only explanation, review roles, release, and stale-lock
         recovery. Test an idle period beyond the TTL without waiting through it
         during the presentation. Check task links and cross-loan navigation.
   - [ ] Verify AI context when changing loan and screen, and on app-level views.
         Preflight the actual provider; if unavailable, use a clearly labeled
         fixture response or an honest unavailable state, never a simulated live answer.
   - **Deliverable/gate:** recorded pass/fail checklist for the exact journey,
     with no unresolved blocker in the chosen script. Mock recommendations,
     including “Approve/Eligible,” are explicitly simulations, never approvals.

4. **A4 — Rehearse, reset, and freeze the demo.** Depends on A3 and B's gate.
   - [ ] Add focused regressions for defects exposed by the journey, lock
         contention, context changes, and persistence; avoid tests that merely
         reproduce implementation details. Retain the existing plugin-context
         smoke test in [plugins/mod.rs](../src-tauri/src/plugins/mod.rs).
   - [ ] Run the frontend build/type checks and relevant Rust/database tests in
         the isolated environment; record commands and outcomes. Rehearse once
         from reset, then reset and repeat with the presentation configuration.
   - **Deliverable/gate:** stable revision, reset instructions, presenter script,
     fixture set, evidence of repeat runs, and a visibly labeled fallback route.
     A successful build alone does not pass this gate.

**Outside the sponsor-demo gate:** DU and Freddie Mac LPA/LP sandbox access,
partner-specific transport/authentication and response validation, ULAD mapping
confirmation and certification, real credit integration, file-content storage,
authoritative handbook retrieval, and pilot/security hardening. Track these
separately against [PILOT_CHECKLIST.md](PILOT_CHECKLIST.md) and
[EXECUTION_PLAN.md](EXECUTION_PLAN.md); their historical readiness labels do
not replace current evidence. Pilot work includes authentication, authorization
and tenant isolation review, operational recovery, and deployment validation.
These cannot be claimed complete through a synthetic demo. A defect affecting
the demonstrated workflow remains a demo blocker even if related hardening is deferred.

## B. Bring appearance up to date

1. **B1 — Establish the shared visual foundation.**
   - [ ] Consolidate effective tokens in `src/App.css`; define shared gray/yellow
         colors, rail material, type roles, controls, and status/focus treatments.
         Create an original yellow block-built ORION wordmark for `NavTray`.
   - **Deliverable/gate:** reviewable shell composition using existing code and
     fonts, with a consistent opaque fallback. Yellow/gray, soft-gray light,
     equipment character, and matched glass are confirmed; precise swatches,
     fonts, widths, blur, radii, and spacing still need visual validation.

2. **B2 — Make the shell work as one kit.** Depends on B1.
   - [ ] Match navigation/chat materials; add central workspace tabs around
         the existing routing and loan-stage behavior without a second navigation
         implementation. Preserve loan identity, drafts, and intentional focus.
   - [ ] Add one AI / Slack mode selector to the right rail. Slack is a labeled
         disconnected shell; no fake channels, unread counts, or enabled sending.
         AI provider/model copy must reflect actual configuration.
   - [ ] Define responsive rail collapse/drawer behavior with explicit open/close
         controls and focus return. Keep navigation reachable, stack review when
         needed, and prevent inaccessible content at compact widths and 200% zoom.
   - **Deliverable/gate:** working keyboard path through navigation, central tabs,
     loan stages, chat switching, and compact rails, with no lost context or
     unintended messages. Visual selection and keyboard focus remain distinct.

3. **B3 — Prove one loan screen, then extend.** Depends on B2 and A2.
   - [ ] Apply the system to Income Analysis through `LoanListScreen`,
         `LoanHeader`, `LoanColumns`, and `UnderwriterPanel`; use A's same fixture
         and save/reopen journey. Keep roles, locks, amounts, and save states clear.
   - [ ] Review empty, loading, error, read-only, saving, and disconnected states.
         Measure rendered contrast; test keyboard/focus, reduced transparency,
         forced colors, and reduced motion. Do not claim accessibility from swatches alone.
   - [ ] Extend approved shared patterns across the demo journey, then remaining
         screens. Remove obsolete conflicting styles; avoid special-case copies,
         generic SaaS card clutter, and panels competing with the working loan.
   - **Deliverable/gate:** a coherent demo journey at presentation and compact
     sizes, with documented visual/interaction checks and no workflow regression.

## Recommended starting slice and dependency order

Start with **A1 → A2 plus B1**, then a minimal B2 shell around **one persisted
Income journey**. Show a synthetic loan, change and reopen one amount, display
review/lock context, and switch the single chat rail between truthful AI status
and the disconnected Slack shell. Reuse the existing route, plugin registry,
and shared loan scaffold; do not build a separate presentation-only application.

This slice proves the setup, real persistence, and visual identity together.
It is an intermediate gate, not the complete sponsor demo. Continue through
A3 and B3 before A4's full rehearsal. Resolve workflow defects before polishing
their presentation, and use the North Star review questions before expanding scope.
