-- Conditions: underwriting's checklist for clearing the loan. One row
-- per condition; outstanding ones are what block approval and drive the
-- flags the underwriter column shows.
CREATE TABLE loan_conditions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id     UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id  UUID NOT NULL REFERENCES companies(id),
    title       TEXT NOT NULL,
    detail      TEXT,
    source      TEXT NOT NULL DEFAULT 'underwriter',
    -- underwriter | processor | system
    status      TEXT NOT NULL DEFAULT 'open',
    -- open | submitted | cleared | waived
    priority    TEXT NOT NULL DEFAULT 'normal', -- low | normal | high
    clear_by    TIMESTAMPTZ,
    cleared_at  TIMESTAMPTZ,
    cleared_by  UUID REFERENCES users(id),
    created_by  UUID REFERENCES users(id),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_conditions(loan_id);
CREATE INDEX ON loan_conditions(company_id, status);
