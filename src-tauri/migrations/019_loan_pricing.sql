-- Loan pricing: the option rows a loan officer builds against the active
-- rate sheet. Monthly P&I and the dollar cost of points are computed from
-- the loan's terms at save time so quotes stay attached to real numbers.
-- One option is the LO's pick — the one that flows toward locking (Phase 4 lock-in comes later).
CREATE TABLE loan_pricing_options (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id       UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id    UUID NOT NULL REFERENCES companies(id),
    product       TEXT NOT NULL DEFAULT '30yr_fixed',
    -- 30yr_fixed | 20yr_fixed | 15yr_fixed | 7_1_arm | 5_1_arm
    rate          NUMERIC(5,3) NOT NULL,
    points        NUMERIC(5,3) NOT NULL DEFAULT 0, -- positive = borrower pays, negative = lender credit
    monthly_pi    NUMERIC(12,2),  -- computed from loan terms at save time
    points_cost   NUMERIC(12,2),  -- points × loan amount
    comp_plan     TEXT,
    selected      BOOLEAN NOT NULL DEFAULT false,
    created_by    UUID REFERENCES users(id),
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_pricing_options(loan_id);
