-- REO screen (1003 §3 — other real estate owned). Person-owned rows.
-- See docs/1003-fields.md.

CREATE TABLE loan_properties_reo (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    borrower_id     UUID NOT NULL REFERENCES borrowers(id) ON DELETE CASCADE,
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),

    address_line    TEXT,
    -- MISMO: PROPERTY/ADDRESS (linked to owner via xlink) ⚠

    city            TEXT,
    state           TEXT,
    postal_code     TEXT,

    property_type   TEXT NOT NULL DEFAULT 'single_family',
    -- MISMO: PropertyType ⚠
    -- values: single_family | condo | multi_family | land | commercial | other

    market_value    NUMERIC(12, 2),
    -- MISMO: valuation ValueAmount ⚠

    mortgage_payment NUMERIC(12, 2),
    -- MISMO: housing expense / payment ⚠

    rental_income   NUMERIC(12, 2),
    -- MISMO: gross rent ⚠ (netting happens downstream — keep gross)

    disposition     TEXT NOT NULL DEFAULT 'retain',
    -- MISMO: DispositionStatusType ⚠
    -- values: retain | pending_sale | sold | pending_removal

    notes           TEXT,

    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loan_properties_reo(loan_id);
CREATE INDEX ON loan_properties_reo(borrower_id);
