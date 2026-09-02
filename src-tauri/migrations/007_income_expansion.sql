-- Income expansion (1003 §2): 2-year averaging support and wider
-- income-type coverage. Extends loan_income (existing table).

ALTER TABLE loan_income
    ADD COLUMN prior_year_monthly NUMERIC(12, 2),
    -- MISMO: CURRENT_INCOME (prior-year occurrence, linked by
    -- CurrentIncomeType + xlink ⚠) — enables the 2-yr average the AUS
    -- expects for variable income types.

    ADD COLUMN verified BOOLEAN NOT NULL DEFAULT false;
    -- Verification marker moved forward from the pipeline phase; kept
    -- NULL-safe so income sources can be marked as documented.

-- Widened income types (MISMO CurrentIncomeType ⚠). Existing values
-- unchanged: base | overtime | bonus | commission | self_employed | other
-- New: retirement | social_security | rental | military_benefits
