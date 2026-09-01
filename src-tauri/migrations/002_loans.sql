CREATE TABLE loans (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id      UUID NOT NULL REFERENCES companies(id),
    loan_number     TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'application',
    -- statuses: application | processing | underwriting | approval | closing | funded
    assigned_to     UUID REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loans(company_id);
CREATE INDEX ON loans(assigned_to);

CREATE TABLE borrowers (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),
    first_name      TEXT NOT NULL,
    last_name       TEXT NOT NULL,
    email           TEXT,
    phone           TEXT,
    is_primary      BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON borrowers(loan_id);

CREATE TABLE loan_income (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),
    borrower_id     UUID NOT NULL REFERENCES borrowers(id),
    income_type     TEXT NOT NULL,
    -- types: base | overtime | bonus | commission | self_employed | other
    monthly_amount  NUMERIC(12, 2) NOT NULL DEFAULT 0,
    employer_name   TEXT,
    years_on_job    NUMERIC(4, 1),
    notes           TEXT,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_income(loan_id);
