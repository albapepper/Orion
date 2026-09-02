-- Property screen (1003 §4 — subject property). 1:1 with loans.
-- See docs/1003-fields.md.

CREATE TABLE loan_subject_property (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id         UUID NOT NULL UNIQUE REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),

    address_line    TEXT,
    -- MISMO: COLLATERALS/COLLATERAL/PROPERTY/ADDRESS/AddressLineText ⚠

    city            TEXT,
    state           TEXT,
    postal_code     TEXT,
    -- MISMO: CityName / StateCode / PostalCode

    property_type   TEXT NOT NULL DEFAULT 'detached',
    -- MISMO: PROPERTY/PROPERTY_DETAIL/PropertyType? (PropertyClassType) ⚠
    -- values: detached | condo | pud | manufactured | multi_family | other

    occupancy       TEXT NOT NULL DEFAULT 'primary',
    -- MISMO: PROPERTY_DETAIL/PropertyUsageType ⚠
    -- values: primary | secondary | investment

    year_built      INTEGER,
    -- MISMO: PropertyStructureBuiltYear ⚠

    units           INTEGER NOT NULL DEFAULT 1,
    -- Number of units on the subject property

    estimated_value NUMERIC(12, 2),
    -- MISMO: PROPERTY_VALUATIONS/.../ValueAmount ⚠
    -- (loan_terms.property_value mirrors this for LTV math)

    notes           TEXT,

    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_subject_property(company_id);
