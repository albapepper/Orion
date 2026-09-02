-- Loan Terms screen (1003 §4 — Loan and Property Information).
-- 1:1 with loans. First migration of Phase 2: every captured column
-- carries its MISMO 3.4 name so the Phase 4 serializer is a rename,
-- not a re-discovery. See docs/1003-fields.md.

CREATE TABLE loan_terms (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id         UUID NOT NULL UNIQUE REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),

    loan_purpose    TEXT NOT NULL DEFAULT 'purchase',
    -- MISMO: TERMS_OF_LOAN/LoanPurposeType
    -- values: purchase | refinance | cash_out | construction | other

    mortgage_type   TEXT NOT NULL DEFAULT 'conventional',
    -- MISMO: TERMS_OF_LOAN/MortgageType
    -- values: conventional | fha | va | usda | other

    amortization_type TEXT NOT NULL DEFAULT 'fixed',
    -- MISMO: TERMS_OF_LOAN/AmortizationType
    -- values: fixed | arm | interest_only | other

    loan_amount     NUMERIC(12, 2),
    -- MISMO: TERMS_OF_LOAN/NoteAmount

    interest_rate   NUMERIC(5, 3),
    -- MISMO: TERMS_OF_LOAN/NoteRatePercent

    term_months     INTEGER,
    -- MISMO: TERMS_OF_LOAN/LoanAmortizationTermMonths

    lien_priority   TEXT NOT NULL DEFAULT 'first',
    -- MISMO: TERMS_OF_LOAN/LienPriorityType
    -- values: first | second

    -- Entered values, not stored ratios: LTV/CLTV are computed on read
    -- from these inputs (MISMO carries LTVValue/CLTVValue).
    property_value  NUMERIC(12, 2),
    -- MISMO: COLLATERALS/COLLATERAL/PROPERTY/PROPERTY_VALUATIONS/...MarketValueAmount

    application_date DATE,
    -- MISMO: LOAN/APPLICATION/ApplicationDate

    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_terms(company_id);
