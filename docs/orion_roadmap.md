# Orion — Build Roadmap

## Status: Phase 1 complete. Working Rust/Tauri app with local AI embedded.

## Date: September 2, 2026

---

## Where You Are

- ✓ Tauri shell running
- ✓ Config system
- ✓ AI abstraction layer (local Ollama working)
- ✓ Postgres connection + migrations
- ✓ Basic app state

Everything from here is screens on top of that foundation. Every screen
follows the same pattern: form → save → audit log → AI sidebar with
context injection → lock/presence.

---

## The Complete Screen Inventory

Every screen Orion needs to be a real LOS. Organized by what it's for.

### The 1003 / URLA (the loan application itself)

These are the data entry screens. They feed everything downstream —
DU/LPA, pricing, underwriting. Build these first.

| #   | Screen       | What it captures                                          | AI context                            |
| --- | ------------ | --------------------------------------------------------- | ------------------------------------- |
| 1   | Borrower Info | Name, SSN, DOB, contact, marital, dependents, citizenship | Borrower profile                      |
| 2   | Employment   | 2-year history, employer, position, dates, self-employed  | Employment gaps, stability            |
| 3   | Income       | All sources, monthly amounts, 2-yr averages, DTI calc     | Income qualification, usable income   |
| 4   | Assets       | Checking, savings, retirement, gifts, verification        | Reserves, sourcing, seasoning         |
| 5   | Liabilities  | All debt, payments, balances, payoff at close             | DTI impact, what to pay off           |
| 6   | REO          | Other properties owned, value, payment, rental income     | Rental income calc, disposition       |
| 7   | Property     | Subject property address, type, occupancy, appraised value | Property eligibility, occupancy      |
| 8   | Loan Terms   | Purpose, type, amount, rate, term, LTV, CLTV              | Product fit, structure                |
| 9   | Declarations | The 13 yes/no questions on the 1003                       | Flags and explanations                |
| 10  | Demographic  | HMDA government monitoring info                           | Compliance capture only               |

### Pipeline & Workflow (how work gets done)

| #  | Screen        | What it does                                                 |
| -- | ------------- | ------------------------------------------------------------ |
| 11 | Dashboard     | AI-summarized pipeline: what needs attention today, by priority |
| 12 | Loan List     | All loans, filterable by status/assignee/stage                |
| 13 | Loan Overview | Summary card for one loan: key numbers, status, who's on it   |
| 14 | Tasks         | Unified task list for a loan, shared by all users             |
| 15 | Documents     | Document checklist, upload metadata, status per doc           |
| 16 | Conditions    | Underwriting conditions, who's clearing them, status          |

### Pricing & AUS (the money and the approval)

| #  | Screen      | What it does                                                |
| -- | ----------- | ----------------------------------------------------------- |
| 17 | Pricing     | Rate/point options, lock status, AI builds options from goals |
| 18 | AUS         | Submit to DU/LPA, view findings, conditions returned          |
| 19 | Rate Sheets | Admin: manage current rates by product/term                   |

### Closing & Funding

| #  | Screen  | What it does                                  |
| -- | ------- | --------------------------------------------- |
| 20 | Closing | Closing date, settlement agent, final numbers |
| 21 | Funding | Funding date, wire confirmation, loan complete |

### Admin & Setup

| #  | Screen           | What it does                                                     |
| -- | ---------------- | ---------------------------------------------------------------- |
| 22 | Setup Wizard     | First-run: AI choice, DB connection, schema deploy, admin account |
| 23 | Users            | Add/edit users, roles (LO, processor, underwriter, admin)         |
| 24 | Company Settings | Company name, defaults, AI settings, integrations                 |

### Shared Components (present on every loan screen)

| Component      | What it does                                              |
| -------------- | --------------------------------------------------------- |
| AI Sidebar     | Persistent right panel, screen-aware context injection    |
| Lock Badge     | Top bar: who has edit access, which screen they're on     |
| Task Panel     | Collapsible: outstanding tasks for this loan              |
| Contacts Panel | People relevant to this screen, one-click email/Slack     |
| Loan Header    | Borrower name, loan number, status — always visible       |

---

## The Roadmap

### Phase 2 — The 1003 Screens

**Sept 2 → Oct 14 (6 weeks)**

Build screens 1–10. Each one gets the full pattern: form, save, audit,
AI sidebar. The schema for all of these needs to be MISMO 3.4 complete
from the first migration — this is what DU/LPA will consume.

Build order (dependencies flow downward):

```
Week 1    Borrower Info + Loan Terms
          (everything else references these)
Week 2    Employment + Income
          (income already partially built — expand it)
Week 3    Assets + Liabilities
Week 4    Property + REO
Week 5    Declarations + Demographic
Week 6    Loan Overview screen + review all 10 for consistency
```

**Definition of done:** A loan officer can enter a complete 1003 with a
mouse, every field saves to Postgres with an audit entry, and the AI
sidebar answers screen-specific questions accurately on all 10 screens.

---

### Phase 3 — Pipeline & Workflow

**Oct 15 → Nov 11 (4 weeks)**

Build screens 11–16. This is what makes it a team product, not a form.

```
Week 7    Dashboard with AI pipeline summary + Loan List
Week 8    Tasks (create, assign, complete, unified view)
Week 9    Documents + Conditions
Week 10   Lock system on every screen + presence indicator
```

**Definition of done:** Two users can open the same loan, one has edit
access, the other sees who's editing and on which screen. Both see the
same task list. The dashboard tells each user what to work on next.

**Milestone: DEMO-READY.** This is what you show a buyer. Enough to
prove the concept, not enough to run a shop on yet.

---

### Phase 4 — Pricing & AUS

**Nov 12 → Dec 16 (5 weeks)**

Build screens 17–19 plus the MISMO serialization layer.

```
Week 11   Rate Sheets admin + Pricing screen (manual option building)
Week 12   Pricing AI: "give me options at 6.5% with under $3K in points"
Week 13   MISMO 3.4 serialization layer (loan → XML)
Week 14   DU submission (sandbox) + findings display
Week 15   LPA submission (sandbox) + findings display
```

**Definition of done:** A complete loan can be priced, serialized to
MISMO 3.4, submitted to DU and LPA sandbox, and findings display in
Orion. This is the last piece a buyer needs to see before they'll
consider a pilot.

**Dependency:** GSE sandbox access must be granted by week 14. Apply NOW.

---

### Phase 5 — Team, Admin & Polish

**Dec 17 → Jan 27 (5 weeks, accounts for holidays)**

Build screens 20–24 and the shared components.

```
Week 16   Setup Wizard (both AI paths, DB connect, schema deploy)
Week 17   User management + Company settings
Week 18   Closing + Funding screens
Week 19   Contacts panel per screen + polish every screen
Week 20   Full walkthrough as a new user — fix everything that's confusing
```

**Definition of done:** IT can install Orion from scratch using only the
wizard. A new loan officer can open a loan and work it start to finish
without asking anyone how the software works.

**Milestone: PILOT-READY.** You can put this in front of a real shop.

---

### Phase 6 — Certification & Pilot

**Jan 28 → Mar 31 (8 weeks)**

```
Weeks 21–24   GSE certification testing (DU and LPA, parallel)
              Fix every scenario that doesn't pass
Weeks 25–28   Pilot with first customer
              Daily bug fixes, weekly feature polish
              Collect the feedback that becomes V1.1
```

**Definition of done:** GSE production credentials in hand. One shop
running new loans through Orion daily.

**Milestone: LAUNCH.** You have a product with a reference customer
and GSE approval. Now you sell it.

---

## Parallel Tracks — Start These Now

These don't block the build but they gate the sale. Do them alongside
the code.

### Legal (Weeks 1–6)

```
This week    Read your Rocket employment agreement.
             IP clause, non-compete, non-solicitation.
             Employment attorney review if anything's unclear ($200–400)

Week 2       Find a software/IP attorney (enterprise software experience)
Week 3–5     EULA / Enterprise License Agreement drafted ($3–7K)
Week 3       IP assignment: you → Scoracle LLC ($500–1K)
Week 2       Trademark filing, Class 42 ($350) — before you go public
```

### GSE Applications (Start this week — 8–18 month lead time)

```
This week    Apply to Fannie Mae Technology Partner program
This week    Apply to Freddie Mac Technology Partner program
This week    Join MISMO (member tier for full 3.4 spec)
Month 2      Sandbox access granted (typical)
Month 4      Start certification testing (Phase 4)
Month 8–14   Production approval
```

### Mobile (After Phase 5)

Swift + SwiftUI. Slim version: Loan List, Loan Overview, Tasks,
lightweight edits on Borrower Info and Income. No AI. Connects to
the same Postgres. Starts after pilot feedback shapes what field
users actually need.

---

## Milestones at a Glance

```
Nov 11, 2026    DEMO-READY      Show buyers. Get letters of intent.
Jan 27, 2027    PILOT-READY     Install at one shop. Real loans.
Mar 31, 2027    LAUNCH          GSE approved. Reference customer. Sell.
Q2 2027         MOBILE          iOS app in TestFlight
```

---

## The Pattern for Every Screen

Don't reinvent this per screen. Every screen is:

```
1. Migration      — tables for this screen's data (MISMO-complete)
2. Rust queries   — get_X(loan_id), save_X(loan_id, data)
3. Audit hook     — every save_X writes to audit_log
4. Tauri command  — exposes get/save to frontend
5. Context fn     — build_context() for the AI sidebar
6. SolidJS screen — form with mouse-driven fields, save button
7. Sidebar wire   — pass screen name to AiSidebar component
8. Lock check     — acquire on enter, release on exit, badge in header
```

Build one screen completely with this pattern. Then every other screen
is the same eight steps with different fields. You already have the
income screen — use it as the template.

---

## What Not to Build Yet

- TRID disclosure generation — V2
- HMDA reporting/LAR file — V2
- Credit pull integration — V2 (requires bureau agreements)
- E-signature — V2
- Batch/overnight jobs — V3
- Web version — not planned
- Multi-language — not planned

Every one of these is a buyer question. The answer is "on the roadmap."
Don't let any of them delay the pilot.
