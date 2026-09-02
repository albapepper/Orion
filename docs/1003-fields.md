# The 1003 in MISMO 3.4 — Field Reference

**Purpose:** the checklist that every Phase 2 migration copies from. One
section per screen: the Orion DB tables it produces, its anchor in the
MISMO 3.4 Reference Model, and the field inventory to capture.

**Sources & the verification rule:**

- MISMO v3.4 Reference Model (`MISMO_3.4.0_B324.xsd` and imports) — the
  schema itself, via MISMO membership (PT-04)
- **ULAD Mapping Document** (Fannie/Freddie UMDP) — the authoritative
  URLA-field → MISMO 3.4 cross-reference. Any path marked ⚠ below is
  written from the reference model and must be confirmed against ULAD
  before Phase 4 serialization; exact element names in nested containers
  vary between model versions.
- Paths below are abbreviated from the root:
  `MESSAGE/DEAL_SETS/DEAL_SET/DEALS/DEAL/…`

**MISMO 3.4 top-level shape:**

```
DEAL
├── LOANS → LOAN                    (the loan: terms, purpose, amounts)
│   └── APPLICATION                 (the URLA itself)
├── PARTIES → PARTY → ROLES → ROLE  (people: borrower, co-borrower,
│                                    loan originator; BORROWER hangs here)
└── COLLATERALS → COLLATERAL        (subject property, valuations)
```

Person-owned data (income, assets, liabilities, REO, declarations,
demographics) hangs off `PARTY/ROLES/ROLE/BORROWER`, linked to the loan
with xlinks. Orion stores these in loan-scoped tables with a
`borrower_id` FK — the xlink relationship becomes a foreign key.

---

## 1. Loan Terms — screen 8

**DB tables:** `loan_terms` (1:1 with loans)

**MISMO anchor:** `LOANS/LOAN/TERMS_OF_LOAN` ⚠ (also `LOAN/APPLICATION`
header fields: application date, purpose)

| Field | MISMO 3.4 |
| --- | --- |
| Loan purpose | `LoanPurposeType` (Purchase, Refinance, CashOut…) |
| Loan type | `MortgageType` (Conventional, FHA, VA, USDA) |
| Loan amount | `NoteAmount` / `NoteAmountType` |
| Interest rate | `NoteRatePercent` |
| Term | `LoanAmortizationTermMonths` |
| Amortization type | `AmortizationType` (Fixed, ARM…) |
| LTV / CLTV | `LTVValue` / `CLTVValue` ⚠ (calc fields — store inputs, compute) |
| Lien position | `LienPriorityType` |
| Application date | `ApplicationDate` |

Capture the **entered** values (amount, rate, value) and compute
ratios on read; DU rejects pre-rounded ratios.

## 2. Borrower Info — screen 1

**DB tables:** extend `borrowers`; add `borrower_dependents`

**MISMO anchor:** `PARTIES/PARTY/ROLES/ROLE/BORROWER`
→ `BORROWER_DETAIL`, `DEPENDENTS`; contact under the owning `PARTY`
(`CONTACT_POINTS`, `ADDRESSES`) ⚠ exact contact-point nesting

| Field | MISMO 3.4 |
| --- | --- |
| Name | `FirstName` `MiddleName` `LastName` `SuffixName` |
| SSN | `SSN` (encrypt at rest; audit must not log it) |
| DOB | `BirthDate` |
| Marital status | `MaritalStatusType` |
| Citizenship | `CitizenshipStatusType` (+ residency `ResidencyType`) |
| Dependents | `DEPENDENTS` → `DependentCount`, `DependentAges` |
| Phone / email | `PARTY` contact containers ⚠ |
| Mailing address | `PARTY/ADDRESSES` (`AddressType`: Current/Mailing) |
| Co-borrower relationship | `RelationshipType` (unmarried addendum lives here later) |

## 3. Employment — screen 2

**DB tables:** `borrower_employments` (N:1 borrowers; history rows flagged
current/previous)

**MISMO anchor:** `BORROWER/EMPLOYERS/EMPLOYER`
→ `CURRENT_EMPLOYMENT` / `EMPLOYMENT_STATUS` ⚠

| Field | MISMO 3.4 |
| --- | --- |
| Employer name | `EmployerName` |
| Self-employed | `EmploymentClassificationType` / self-employed flags ⚠ |
| Position / title | `PositionDescription` |
| Monthly base income | `MonthlyIncomeAmount` (also surfaces in `CURRENT_INCOME` as employment income) |
| Start / end dates | `EmploymentStartDate` / `EmploymentEndDate` |
| Months in line of work | `TimeInLineOfWorkMonths` |
| Business phone | employer contact fields |

2-year history = ordered rows; AI context flags gaps from date math.

## 4. Income — screen 3 (expand existing)

**DB tables:** extend `loan_income` — add averaging + DTI-relevant columns

**MISMO anchor:** `BORROWER/CURRENT_INCOME` (per source; employment
income cross-references `EMPLOYER` via xlink ⚠)

| Field | MISMO 3.4 |
| --- | --- |
| Income source type | `CurrentIncomeType` (Base, Overtime, Bonus, Commissions, SelfEmployed, OtherIncome) |
| Monthly amount | `CurrentIncomeMonthlyTotalAmount` |
| Employment income item | `CURRENT_INCOME/EmploymentIncomeItem` |
| 2-year average (computed) | keep as Orion-computed column; MISMO carries monthly |
| Descriptor / notes | `CurrentIncomeDescription` |

DTI is computed downstream — store sources, compute qualification.

## 5. Assets — screen 4

**DB tables:** `loan_assets` (+ optional `asset_gift_sources`)

**MISMO anchor:** `BORROWER/ASSETS/ACCOUNT`
→ `ACCOUNT_DETAIL`, `ACCOUNT_TRANSACTION`, gift/grant containers ⚠

| Field | MISMO 3.4 |
| --- | --- |
| Account type | `AccountType` (CheckingAsset, SavingsAsset, RetirementFund, MutualFund, GiftFunds…) |
| Institution | `AccountOwnerParty` references + institution name |
| Balance | `BookValue` / `CashBalance` ⚠ |
| Gift source / donor | gift-grant container ⚠ (donor name, relationship, transfer date) |
| Verification metadata | `DepositoryAccountType`, last-verified fields ⚠ |

## 6. Liabilities — screen 5

**DB tables:** `loan_liabilities`

**MISMO anchor:** `BORROWER/LIABILITIES/LIABILITY`

| Field | MISMO 3.4 |
| --- | --- |
| Liability type | `LiabilityType` (MortgageLoan, Installment, Revolving, Open…) |
| Creditor name | `CreditorName` |
| Monthly payment | `MonthlyPaymentAmount` |
| Unpaid balance | `UnpaidBalanceAmount` |
| Payoff at close | `WillBePaidOffIndicator` / `PayoffStatus` ⚠ |
| Months remaining / deferred | `RemainingTermMonths`, deferment flags ⚠ |

## 7. REO — screen 6

**DB tables:** `loan_properties_reo`

**MISMO anchor:** property structures under `DEAL`/`PARTY` with
owner-party xlinks; REO disposition lives with
`PROPERTY/PROPERTY_DETAIL` + income/expense containers ⚠ — confirm
exact path against ULAD (REO in 3.4 is xlink-linked, not nested).

| Field | MISMO 3.4 |
| --- | --- |
| Address / type | property address, `PropertyType` |
| Market value | `PropertyValuation` / `MarketValueAmount` ⚠ |
| Mortgage payment | `HousingExpense`/payment fields ⚠ |
| Rental income | gross rent container ⚠ |
| Disposition intent | `DispositionStatusType` (PendingSale, Retain…) |

## 8. Property (subject) — screen 7

**DB tables:** `loan_subject_property` (1:1 loans)

**MISMO anchor:** `DEAL/COLLATERALS/COLLATERAL/SUBJECT_PROPERTY` ⚠
(nested `PROPERTY_DETAIL`, valuations under `PROPERTY_VALUATIONS`)

| Field | MISMO 3.4 |
| --- | --- |
| Address | `PROPERTY/ADDRESS` (full USPAP-style address fields) |
| Property type | `PropertyType` (Detached, Condominium, PUD, Manufactured…) |
| Occupancy | `PropertyUsageType` (PrimaryResidence, SecondHome, Investment) |
| Estimated value | valuation container ⚠ |
| Year built / units | `PropertyStructureBuiltYear`, unit count |
| Construction / lot | detail containers ⚠ |

## 9. Declarations — screen 9

**DB tables:** `loan_declarations` (1 row per question per borrower)

**MISMO anchor:** `BORROWER/DECLARATION` — one `DECLARATION` occurrence
per URLA §5 question; the URLA question ID ↔ `DeclarationType` enum
mapping must be built from ULAD when this migration is written.

| Field | MISMO 3.4 |
| --- | --- |
| Question answer | per-question boolean indicators ⚠ |
| Explanation text | `DECLARATION_DETAIL` / explanation fields ⚠ |

Store as `(borrower_id, declaration_key, answer, explanation)` so
new questions are data, not schema.

## 10. Demographic (HMDA) — screen 10

**DB tables:** `borrower_demographics`

**MISMO anchor:** `BORROWER/GOVERNMENT_MONITORING` → race/ethnicity/sex
containers + monitoring questions ⚠

| Field | MISMO 3.4 |
| --- | --- |
| Ethnicity | `ETHNICITY` containers (choices + decline-to-answer) |
| Race | `RACE` containers (choices + decline-to-answer) |
| Sex | `SEX` / `SexOfApplicantOrBorrowerType` |
| Basis of collection | face-to-face vs. not indicator |

Compliance capture only: no AI context beyond "recite what was
collected"; never infer or summarize these fields.

---

## Cross-cutting rules for every Phase 2 migration

1. **MISMO-complete from migration one.** Every captured field has its
   MISMO name recorded (column comment: `-- MISMO: X/Y`) so the Phase 4
   serializer is a rename, not a re-discovery.
2. **UUID PKs, `company_id` FK, `created_at`/`updated_at`** — same as
   existing tables.
3. **Person-owned vs. loan-owned:** borrower data (income, assets…) keys
   on `borrower_id`; loan-level data (terms, subject property) keys on
   `loan_id` only.
4. **Nothing sensitive in logs:** SSN and demographic fields are never
   written to the audit log payload or AI context verbatim.
5. **Amounts are `NUMERIC`, not float**; ratios computed on read.
