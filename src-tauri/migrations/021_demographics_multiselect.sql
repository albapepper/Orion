-- P3-15: Multi-select race/ethnicity (MISMO/HMDA). The URLA lets a
-- borrower select all that apply — single-value columns can't carry
-- that. Choices move to JSONB arrays; "not_provided" is the exclusive
-- decline-to-answer marker. Existing single values are backfilled so
-- nothing on file is lost, then the old columns are dropped.
ALTER TABLE borrower_demographics
    ADD COLUMN ethnicity_choices JSONB NOT NULL DEFAULT '[]',
    ADD COLUMN race_choices JSONB NOT NULL DEFAULT '[]';

-- MISMO: BORROWER/GOVERNMENT_MONITORING → ETHNICITY containers
-- (hispanic_or_latino | not_hispanic_or_latino | not_provided)
COMMENT ON COLUMN borrower_demographics.ethnicity_choices IS
    'MISMO: GOVERNMENT_MONITORING/ETHNICITY — all that apply, or [not_provided] to decline';

-- MISMO: BORROWER/GOVERNMENT_MONITORING → RACE containers
-- (american_indian_or_alaska_native | asian | black_or_african_american |
--  native_hawaiian_or_pacific_islander | white | not_provided)
COMMENT ON COLUMN borrower_demographics.race_choices IS
    'MISMO: GOVERNMENT_MONITORING/RACE — all that apply, or [not_provided] to decline';

UPDATE borrower_demographics
SET ethnicity_choices = CASE WHEN ethnicity IS NULL THEN '[]'::jsonb
                             ELSE jsonb_build_array(ethnicity) END,
    race_choices = CASE WHEN race IS NULL THEN '[]'::jsonb
                        ELSE jsonb_build_array(race) END;

ALTER TABLE borrower_demographics
    DROP COLUMN ethnicity,
    DROP COLUMN race;
