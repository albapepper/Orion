-- Assets screen (1003 §2). Person-owned accounts.
-- See docs/1003-fields.md.

CREATE TABLE loan_assets (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    borrower_id     UUID NOT NULL REFERENCES borrowers(id) ON DELETE CASCADE,
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),

    account_type    TEXT NOT NULL DEFAULT 'checking',
    -- MISMO: ASSETS/ACCOUNT/ACCOUNT_DETAIL/AccountType
    -- values: checking | savings | retirement | mutual_fund | gift | other

    institution     TEXT,
    -- MISMO: ACCOUNT_DETAIL/InstitutionName ⚠

    balance         NUMERIC(12, 2) NOT NULL DEFAULT 0,
    -- MISMO: ACCOUNT_DETAIL/CashBalance ⚠

    gift_donor      TEXT,
    -- MISMO: gift-grant container ⚠ (donor name)

    gift_relationship TEXT,
    -- donor relationship to borrower (documentation driver for gifts)

    notes           TEXT,

    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_assets(loan_id);
CREATE INDEX ON loan_assets(borrower_id);
