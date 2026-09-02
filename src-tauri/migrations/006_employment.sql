-- Employment screen (1003 §1 continued — 2-year work history).
-- Ordered rows per borrower; the AI context flags gaps from date math.
-- See docs/1003-fields.md.

CREATE TABLE borrower_employments (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    borrower_id     UUID NOT NULL REFERENCES borrowers(id) ON DELETE CASCADE,
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),

    employer_name   TEXT,
    -- MISMO: EMPLOYER/EmployerName

    position        TEXT,
    -- MISMO: EMPLOYER/PositionDescription

    self_employed   BOOLEAN NOT NULL DEFAULT false,
    -- MISMO: EMPLOYER/EMPLOYMENT_STATUS/SelfEmployedIndicator ⚠

    monthly_income  NUMERIC(12, 2),
    -- MISMO: EMPLOYER/CURRENT_EMPLOYMENT/MonthlyIncomeAmount
    -- (income screen holds per-source breakdowns; this is the VOE figure)

    start_date      DATE,
    -- MISMO: EmploymentStartDate

    end_date        DATE,
    -- MISMO: EmploymentEndDate (NULL = current)

    is_current      BOOLEAN NOT NULL DEFAULT true,
    -- Drives ordering + the "current employer" pick for AI context

    months_in_line_of_work INTEGER,
    -- MISMO: TimeInLineOfWorkMonths

    business_phone  TEXT,
    -- MISMO: EMPLOYER contact fields ⚠

    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON borrower_employments(borrower_id);
CREATE INDEX ON borrower_employments(loan_id);
