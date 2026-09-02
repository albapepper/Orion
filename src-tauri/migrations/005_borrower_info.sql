-- Borrower Info screen (1003 §1). Extends the borrowers table;
-- dependents get their own table. SSN is sensitive: masked in the
-- audit log and never sent to the AI context. See docs/1003-fields.md.

ALTER TABLE borrowers
    ADD COLUMN middle_name        TEXT,
    -- MISMO: PARTY/NAME/MiddleName
    ADD COLUMN suffix_name        TEXT,
    -- MISMO: PARTY/NAME/SuffixName
    ADD COLUMN ssn                TEXT,
    -- MISMO: BORROWER_DETAIL/SSN — masked in audit + AI context
    ADD COLUMN birth_date         DATE,
    -- MISMO: BORROWER_DETAIL/BirthDate
    ADD COLUMN marital_status     TEXT,
    -- MISMO: BORROWER_DETAIL/MaritalStatusType
    -- values: married | unmarried | separated
    ADD COLUMN citizenship_status TEXT;
    -- MISMO: BORROWER_DETAIL/CitizenshipStatusType
    -- values: us_citizen | permanent_resident | non_permanent_resident

CREATE TABLE borrower_dependents (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    borrower_id UUID NOT NULL REFERENCES borrowers(id) ON DELETE CASCADE,
    loan_id     UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id  UUID NOT NULL REFERENCES companies(id),
    age         INTEGER,
    -- MISMO: BORROWER/DEPENDENTS/DependentAges (one occurrence per dependent)
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON borrower_dependents(borrower_id);
CREATE INDEX ON borrower_dependents(loan_id);
