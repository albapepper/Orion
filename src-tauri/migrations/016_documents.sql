-- Documents: per-loan document records — what the file contains, which
-- stage it supports, who uploaded it. Records only for now: the actual
-- file bytes live outside the database (network share / object storage)
-- and attach in a later phase; `file_name` carries the reference.
CREATE TABLE loan_documents (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id     UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id  UUID NOT NULL REFERENCES companies(id),
    title       TEXT NOT NULL,
    doc_type    TEXT NOT NULL DEFAULT 'other',
    -- application | income_verification | bank_statement | tax_return |
    -- appraisal | title | insurance | id_document | other
    screen      TEXT, -- which loan stage this doc supports
    file_name   TEXT,
    mime_type   TEXT,
    size_bytes  BIGINT,
    uploaded_by UUID REFERENCES users(id),
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    removed_at  TIMESTAMPTZ -- soft delete keeps the audit trail honest
);

CREATE INDEX ON loan_documents(loan_id);
CREATE INDEX ON loan_documents(company_id, doc_type);
