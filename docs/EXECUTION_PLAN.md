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
| **3 — Pipeline & Workflow**  | 🔄 **ACTIVE**                            |
| 4 — Pricing & AUS            | ⏸ outline only                          |
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

## Queue: Future Stages (outlines — expanded on arrival)

### Phase 3 — Pipeline & Workflow (Oct 15 → Nov 11)

Dashboard AI summary · Loan List filters · Loan Overview · Tasks 2.0 ·
Documents · Conditions · lock system on every screen + presence.
**Gate: DEMO-READY.**

### Phase 4 — Pricing & AUS (Nov 12 → Dec 16)

Rate sheets admin · Pricing screen · Pricing AI options · MISMO 3.4
serializer · DU sandbox · LPA sandbox. **Gate: pilot-credible.**
⚠ GSE sandbox access must be in hand by week 14.

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
