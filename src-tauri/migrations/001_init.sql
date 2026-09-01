CREATE TABLE companies (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id  UUID NOT NULL REFERENCES companies(id),
    email       TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    role        TEXT NOT NULL DEFAULT 'loan_officer',
    -- roles: loan_officer | processor | underwriter | admin
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON users(company_id);
