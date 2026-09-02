-- Demographic screen (HMDA government monitoring, URLA §8). 1:1 with
-- borrowers. Sensitive: values are never written to the audit log.
-- See docs/1003-fields.md.

CREATE TABLE borrower_demographics (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    borrower_id     UUID NOT NULL UNIQUE REFERENCES borrowers(id) ON DELETE CASCADE,
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),

    ethnicity       TEXT,
    -- MISMO: BORROWER/GOVERNMENT_MONITORING/ETHNICITY ⚠
    -- values: hispanic_or_latino | not_hispanic_or_latino | not_provided

    race            TEXT,
    -- MISMO: GOVERNMENT_MONITORING/RACE ⚠
    -- values: american_indian_or_alaska_native | asian | black_or_african_american
    --         | native_hawaiian_or_pacific_islander | white | not_provided

    sex             TEXT,
    -- MISMO: SexOfApplicantOrBorrowerType
    -- values: male | female | not_provided

    collection_basis TEXT,
    -- MISMO: monitoring questions ⚠
    -- values: face_to_face | not_face_to_face

    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON borrower_demographics(loan_id);
