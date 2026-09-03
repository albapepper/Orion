# Orion — Execution Plan

**Rolling plan for docs/orion_roadmap.md. One stage active at a time.**

## The Convention

1. **Pick the top unchecked item** in the active stage below.
2. **Build it.** Scope is whatever the item says — no further.
3. **Check the box and commit in the same commit.** The plan file travels
   with the code, so history reads as a changelog.
4. **Commit message:** `<ITEM-ID>: <what landed>` — e.g.
   `P2-03: borrower info queries, audit hook, IPC commands`
5. **If work surfaces a new task,** add it to the stage with the next ID
   before moving on. Plan drifts forward, never silently.
6. **When the last box in a stage checks,** expand the next stage's
   outline into IDs, then start it. That's the "rolling" part.

Rules of engagement:

- An item is **done when it's committed**, not when it compiles.
- A half-finished item stays unchecked — park it with a note in the
  item text if you stop mid-way.
- Phase gates (definition of done) get reviewed before rolling to the
  next phase.

---

## Status

| Phase                        | State                                    |
| ---------------------------- | ---------------------------------------- |
| 1 — Foundation               | ✅ done (pre-plan)                       |
| UX comfort pass              | ✅ done (P2-00)                          |
| 2 — The 1003 Screens         | ✅ done (incl. P2-45 dual-column pass)   |
| 3 — Pipeline & Workflow      | ✅ done — **DEMO-READY at code level**; live two-user check pending dev check |
| **4 — Pricing & AUS**        | 🔄 **ACTIVE (expanded below)**           |
| 5 — Team, Admin & Polish     | ⏸ outline only                          |
| 6 — Certification & Pilot    | ⏸ outline only                          |

**Definition of done for this stage** (from roadmap): a loan officer can
enter a complete 1003, every field saves to Postgres with an audit
entry, and the AI sidebar answers screen-specific questions accurately
on all 10 screens.

---

## Stage: Phase 2 — The 1003 Screens (ACTIVE)

Every screen lands as four commits, following the eight-step pattern
(migration / backend / context / UI). MISMO 3.4 completeness is a
property of every migration from the first one — this is what DU/LPA
consumes later.

### Week 0 — Staging

- [x] **P2-00** Commit the UX comfort pass (stage stepper, autosave, focus
      visibility, legend/README) as its own commit before new work starts.
- [x] **P2-01** MISMO 3.4 field reference: write `docs/1003-fields.md`
      mapping each of the 10 screens to its MISMO 3.4 fields/containers,
      before any migration is written. First migration copies from this.

### Week 1 — Borrower Info + Loan Terms

- [x] **P2-02** Loan Terms migration (purpose, type, amount, rate, term,
      LTV/CLTV — MISMO-complete)
- [x] **P2-03** Loan Terms backend: queries + audit hook + IPC commands
- [x] **P2-04** Loan Terms AI context builder (`build_context`)
- [x] **P2-05** Loan Terms screen UI + sidebar wire + lock check (lock
      badge shown via header; acquisition deferred to Phase 3, consistent
      with all screens)
- [x] **P2-06** Borrower Info migration (name, SSN, DOB, contact, marital,
      dependents, citizenship — MISMO-complete)
- [x] **P2-07** Borrower Info backend: queries + audit hook + IPC commands
- [x] **P2-08** Borrower Info AI context builder
- [x] **P2-09** Borrower Info screen UI + sidebar wire + lock check

### Week 2 — Employment + Income expansion

- [x] **P2-10** Employment migration (2-year history, employer, position,
      dates, self-employed flags)
- [x] **P2-11** Employment backend: queries + audit hook + IPC commands
- [x] **P2-12** Employment AI context builder (gaps, stability)
- [x] **P2-13** Employment screen UI + sidebar wire + lock check
- [x] **P2-14** Income migration expansion (2-yr averages, DTI-relevant
      fields; extend existing table, MISMO-complete)
- [x] **P2-15** Income backend expansion (queries, audit, commands)
- [x] **P2-16** Income AI context expansion (usable income calc)
- [x] **P2-17** Income screen expansion + replace `IncomeScreen` placeholder
      patterns with shared screen scaffolding

### Week 3 — Assets + Liabilities

- [x] **P2-18** Assets migration (checking, savings, retirement, gifts,
      verification fields)
- [x] **P2-19** Assets backend: queries + audit hook + IPC commands
- [x] **P2-20** Assets AI context builder (reserves, sourcing, seasoning)
- [x] **P2-21** Assets screen UI + sidebar wire + lock check (replaces stub;
      built on new shared LoanListScreen scaffolding)
- [x] **P2-22** Liabilities migration (debt, payments, balances, payoff)
- [x] **P2-23** Liabilities backend: queries + audit hook + IPC commands
- [x] **P2-24** Liabilities AI context builder (DTI impact)
- [x] **P2-25** Liabilities screen UI + sidebar wire + lock check (new
      cascade stage after assets, ⌃l)

### Week 4 — Property + REO

- [x] **P2-26** Property migration (subject address, type, occupancy,
      appraised value) — replaces demo property stub
- [x] **P2-27** Property backend: queries + audit hook + IPC commands
- [x] **P2-28** Property AI context builder (eligibility, occupancy)
- [x] **P2-29** Property screen UI + sidebar wire + lock check (replaces stub)
- [x] **P2-30** REO migration (other properties, value, payment, rental income)
- [x] **P2-31** REO backend: queries + audit hook + IPC commands
- [x] **P2-32** REO AI context builder (rental income, disposition)
- [x] **P2-33** REO screen UI + sidebar wire + lock check (⌃r, after liabilities)

### Week 5 — Declarations + Demographic

- [x] **P2-34** Declarations migration (the 13 URLA yes/no questions + text)
- [x] **P2-35** Declarations backend: queries + audit hook + IPC commands
- [x] **P2-36** Declarations AI context builder (flags + explanations)
- [x] **P2-37** Declarations screen UI + sidebar wire + lock check (own
      stage, ⌃q; Yes/No toggles + explanations)
- [x] **P2-38** Demographic migration (HMDA monitoring info)
- [x] **P2-39** Demographic backend: queries + audit hook + IPC commands
- [x] **P2-40** Demographic screen UI + sidebar wire (⌃g; no AI context —
      compliance capture only)
- [x] **P2-41** Demographic AI context builder — resolved: intentionally
      no AI path for HMDA data; `ai_chat` falls through to the generic
      screen context, per cross-cutting rule 4

### Week 6 — Loan Overview + consistency review

- [x] **P2-42** Loan Overview screen (summary card: key numbers, status,
      who's on it) + nav integration (⌃o; ⌃] walks from overview into the
      cascade at borrower)
- [x] **P2-43** Cross-screen consistency review — verified against the
      pattern checklist: MISMO column comments on all 6 new migrations;
      audit hooks on all 15 save/delete paths; context builders on 9 of
      10 screens (demographic intentionally none); StageStepper on every
      loan screen; keybindings assigned for every stage + overview.
      Known deviations (accepted): IncomeScreen predates the shared
      LoanListScreen and keeps its own table; `credit` stage remains a
      stub (credit pull is V2); lock acquisition deferred to Phase 3.

- [x] **P2-44** Week-6 definition-of-done walkthrough — code-level pass
      done; live GUI walkthrough deferred to dev check. Gaps recorded
      for Phase 3: MISMO allows multi-select race/ethnicity (Orion UI is
      single-select — revisit before HMDA export); income screen could
      move to LoanListScreen; Contact/LO contact info not yet captured
      on the 1003. Real-mode (Postgres) entry test pending dev check.

### Follow-on — banker/underwriter dual-column pass

- [x] **P2-45** Two screens per category, side-by-side: the banker's
      work column (left) and the underwriter's review column (right).
      Everyone touching the loan sees both — transparency is the bet —
      but only one is ever editable per signed-in user: loan officers/
      processors/admins own the banking column, underwriters/admins the
      review column (uw_reviews migration 014; `save_uw_review` enforces
      the role server-side, UI locks the other column). Includes persona
      sign-in + Settings picker (⌃s), PizzaTracker with per-stage
      statuses replacing StageStepper, and per-screen UW review wiring.

---

## Stage: Phase 3 — Pipeline & Workflow (ACTIVE)

Build screens 11–16 — the team layer. The dual-column rule from P2-45
extends here: everyone sees the work, roles decide who edits what.

**Definition of done** (from roadmap): two users can open the same
loan, one has edit access, the other sees who's editing and on which
screen. Both see the same task list. The dashboard tells each user
what to work on next. **Milestone: DEMO-READY.**

### Week 7 — Dashboard + Loan List

- [x] **P3-01** Dashboard screen: pipeline summary (loans by stage,
      counts, attention flags) + AI pipeline summary ("what should I
      work on next") from existing context builders
- [x] **P3-02** Loan List screen: filters (stage, status, owner),
      sort, open-loan navigation

### Week 8 — Tasks 2.0

- [x] **P3-03** Tasks migration expansion (assignee, due date,
      priority, status; per-screen linkage already exists)
- [x] **P3-04** Tasks backend: create/assign/complete, unified
      cross-loan task list, audit hook, IPC commands
- [x] **P3-05** Tasks UI: unified Tasks view + TaskPanel upgrade +
      AI context (overdue, unassigned)

### Week 9 — Documents + Conditions

- [x] **P3-06** Documents migration (per-loan doc records, type,
      stage linkage, uploaded-by)
- [x] **P3-07** Documents backend: attach/list/delete + audit + IPC
- [x] **P3-08** Documents screen UI + sidebar wire
- [x] **P3-09** Conditions migration (approval conditions, status,
      source, clear-by)
- [x] **P3-10** Conditions backend: CRUD + audit + IPC
- [x] **P3-11** Conditions screen UI + sidebar wire (outstanding
      conditions drive the UW column flags)

### Week 10 — Lock system + presence

- [x] **P3-12** Lock backend: acquire/renew/release per loan+screen;
      server-side write enforcement on ALL save paths (extends the
      P2-45 role checks to lock state). Locks carry a 10-min TTL —
      stale locks read as free and get stolen on next acquire.
      Acquisition + renewal + presence update ride `get_loan_detail`
      (each screen load declares who and where); release fires on
      leaving the loan.
- [x] **P3-13** Presence: who is on which screen of which loan,
      visible to other sessions (lock row + screen; LockBadge in the
      loan header; demo mode fakes Dana holding loan 2 so both sides
      of the lock are demonstrable via persona switching)
- [x] **P3-14** Lock UI on every screen: live LockBadge, read-only
      work column when another user holds the lock (LoanColumns), with
      `ensure_can_write` refusing every backend save path as the
      enforcement backstop

### Carry-over from Phase 2 (P2-44 deferrals)

- [x] **P3-15** Multi-select race/ethnicity (MISMO/HMDA) — required
      before HMDA export
- [x] **P3-16** Move IncomeScreen onto shared LoanListScreen
- [x] **P3-17** Contact/LO contact info section on the 1003

---

## Stage: Phase 4 — Pricing & AUS (COMPLETE)

Screens 17–19 plus the MISMO serialization layer. Definition of done:
a complete loan can be priced, serialized to MISMO 3.4, submitted to
DU and LPA, and findings display in Orion. **Done in mock mode —
GSE sandbox credentials flip the same commands live** (config:
`aus.du_endpoint` / `aus.lpa_endpoint`).

---

## Stage: Phase 5 — Team, Admin & Polish (ACTIVE)

Dec 17 → Jan 27. Setup wizard · Users · Company settings · Closing ·
Funding · Contacts panel · new-user walkthrough. **Gate: PILOT-READY.**

### Week 15 — Setup wizard & company

- [ ] **P5-01** Setup wizard: the backend already has the commands
      (connect, deploy schema, save config, test AI) — build the
      first-run screen that drives them: gate on `configured`,
      walk DB URL → test → deploy migrations → create company →
      AI check, with a "continue in demo mode" escape hatch
- [ ] **P5-02** Company settings: companies row CRUD (name), settings
      screen becomes a real admin surface (DB, AI, AUS endpoints
      with the test buttons wired), company name in the nav tray

### Week 16 — Team

- [ ] **P5-03** Users admin: create/edit/deactivate users, role
      assignment (loan_officer | processor | underwriter | admin),
      migration for an active flag; Users screen (⌃m) absorbing the
      persona picker
- [ ] **P5-04** Loans & lifecycle: create_loan command (real mode has
      no way to make a loan), set_loan_status with role gating +
      audit, "+ New loan" on the loans screen, status stepper on
      Overview

### Week 17 — Closing & Funding

- [ ] **P5-05** Closing migration + backend: per-loan closing record
      (closing date, location, settlement agent, cost summary) with
      audit + IPC
- [ ] **P5-06** Closing screen UI + stage in the tracker (after AUS);
      scheduling closing moves the loan to 'closing'
- [ ] **P5-07** Funding migration + backend: funding record (funded
      date, disbursement method/reference, amount) with audit + IPC
- [ ] **P5-08** Funding screen UI + stage; funding completes the
      pipeline (status 'funded')

### Week 18 — Contacts & walkthrough

- [ ] **P5-09** Contacts migration + backend: loan_contacts (role:
      realtor | title | escrow | insurance | other) with audit + IPC
- [ ] **P5-10** Contacts panel: bottom panel on every loan screen,
      sibling of the task panel
- [ ] **P5-11** New-user walkthrough: first-run tour card (demo-mode
      explainer, key legend, first-loan CTA), remembered locally

### Week 19 — Pilot polish

- [ ] **P5-12** Pilot polish pass: empty states, error copy, demo→real
      handoff (first real loan from a fresh install), pilot-readiness
      checklist. **Gate: PILOT-READY.**

---

## Queue: Future Stages (outlines — expanded on arrival)

### Phase 5 — Team, Admin & Polish (Dec 17 → Jan 27)

Setup wizard · Users · Company settings · Closing · Funding · Contacts
panel · new-user walkthrough. **Gate: PILOT-READY.**

### Phase 6 — Certification & Pilot (Jan 28 → Mar 31)

GSE certification (DU + LPA parallel) · first pilot shop. **Gate: LAUNCH.**

---

## Parallel Track (non-code, checked here so it isn't forgotten)

- [ ] **PT-01** Read Rocket employment agreement (IP / non-compete / non-solicit)
- [ ] **PT-02** Apply: Fannie Mae Technology Partner program
- [ ] **PT-03** Apply: Freddie Mac Technology Partner program
- [ ] **PT-04** Join MISMO (member tier, full 3.4 spec)
- [ ] **PT-05** Software/IP attorney engaged
- [ ] **PT-06** EULA / Enterprise License Agreement drafted
- [ ] **PT-07** IP assignment → Scoracle LLC
- [ ] **PT-08** Trademark filing, Class 42
