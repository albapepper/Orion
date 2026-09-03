-- P5-07: The funding record. Money moved — when, how, and how much.
-- This is the last stop of the pipeline: a funded date here is what
-- turns the tracker's final stage green.
CREATE TABLE loan_funding (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id     UUID NOT NULL UNIQUE REFERENCES loans(id) ON DELETE CASCADE,
    company_id  UUID NOT NULL REFERENCES companies(id),
    funded_date DATE,
    method      TEXT,   -- wire | check | ach
    reference   TEXT,
    amount      NUMERIC(12,2),
    notes       TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_funding(loan_id);
