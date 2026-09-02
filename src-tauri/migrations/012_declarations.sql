-- Declarations screen (1003 §5). One row per question per borrower.
-- The question text lives in code (URLA_QUESTIONS); answers are data,
-- so new questions never need a schema change.
-- See docs/1003-fields.md.

CREATE TABLE loan_declarations (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    borrower_id   UUID NOT NULL REFERENCES borrowers(id) ON DELETE CASCADE,
    loan_id       UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id    UUID NOT NULL REFERENCES companies(id),

    question_key  TEXT NOT NULL,
    -- Stable key matching URLA §5 question IDs; text maintained in code.
    -- MISMO: BORROWER/DECLARATION (one occurrence per question) —
    -- question_key → DeclarationType enum map built from ULAD at
    -- serialization time.

    answer        BOOLEAN,
    -- NULL = not yet answered

    explanation   TEXT,
    -- Required when answer is true (underwriting will ask)

    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (borrower_id, question_key)
);

CREATE INDEX ON loan_declarations(loan_id);
