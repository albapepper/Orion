-- Rate sheets: what each lender is paying this week. The sheet is the
-- container (who, when, active); the options are the priceable rows the
-- pricing screen (P4-02) and the pricing AI (P4-03) build from.
CREATE TABLE rate_sheets (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id     UUID NOT NULL REFERENCES companies(id),
    lender         TEXT NOT NULL,
    effective_date DATE,
    expires_date   DATE,
    is_active      BOOLEAN NOT NULL DEFAULT true,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE rate_sheet_options (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rate_sheet_id   UUID NOT NULL REFERENCES rate_sheets(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),
    product         TEXT NOT NULL DEFAULT '30yr_fixed',
    -- 30yr_fixed | 20yr_fixed | 15yr_fixed | 7_1_arm | 5_1_arm
    rate            NUMERIC(5,3) NOT NULL,  -- e.g. 6.500
    points          NUMERIC(5,3) NOT NULL DEFAULT 0, -- positive = borrower pays, negative = lender credit
    comp_plan       TEXT,                   -- lender credit / comp plan reference
    min_loan_amount NUMERIC(12,2),
    max_loan_amount NUMERIC(12,2),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON rate_sheets(company_id, is_active);
CREATE INDEX ON rate_sheet_options(rate_sheet_id);
