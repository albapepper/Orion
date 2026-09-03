-- P5-09: The deal's people beyond the borrower — realtor, title,
-- escrow, insurance. Loan-scoped, free-form roles, no borrower FK:
-- vendors serve the file, not one applicant.
CREATE TABLE loan_contacts (
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id    UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id UUID NOT NULL REFERENCES companies(id),
    role       TEXT NOT NULL DEFAULT 'other', -- realtor | title | escrow | insurance | other
    name       TEXT NOT NULL,
    company    TEXT,
    email      TEXT,
    phone      TEXT,
    notes      TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_contacts(loan_id);
