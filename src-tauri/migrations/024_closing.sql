-- P5-05: The closing record. One per loan — the who/where/when of
-- signing plus the cost summary the CD drives. The loan's status move
-- to 'closing' is pipeline state (P5-04's stepper); this is the data
-- the appointment needs.
CREATE TABLE loan_closing (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id          UUID NOT NULL UNIQUE REFERENCES loans(id) ON DELETE CASCADE,
    company_id       UUID NOT NULL REFERENCES companies(id),
    closing_date     DATE,
    closing_location TEXT,
    settlement_agent TEXT,
    closing_costs    NUMERIC(12,2),
    notes            TEXT,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_closing(loan_id);
