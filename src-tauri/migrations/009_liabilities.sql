-- Liabilities screen (1003 §2). Person-owned debts; feeds DTI.
-- See docs/1003-fields.md.

CREATE TABLE loan_liabilities (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    borrower_id     UUID NOT NULL REFERENCES borrowers(id) ON DELETE CASCADE,
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),

    liability_type  TEXT NOT NULL DEFAULT 'installment',
    -- MISMO: LIABILITY/LiabilityType
    -- values: mortgage | installment | revolving | open | other

    creditor_name   TEXT,
    -- MISMO: LIABILITY/CreditorName

    monthly_payment NUMERIC(12, 2),
    -- MISMO: LIABILITY/MonthlyPaymentAmount

    unpaid_balance  NUMERIC(12, 2),
    -- MISMO: LIABILITY/UnpaidBalanceAmount

    payoff_at_close BOOLEAN NOT NULL DEFAULT false,
    -- MISMO: WillBePaidOffIndicator ⚠

    notes           TEXT,

    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_liabilities(loan_id);
CREATE INDEX ON loan_liabilities(borrower_id);
