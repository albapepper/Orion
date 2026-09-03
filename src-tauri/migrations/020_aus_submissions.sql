-- P4-05: AUS submissions — DU (and P4-06's LPA) findings land here.
-- One row per submission of the serialized loan to an automated
-- underwriting engine: the engine's recommendation, its findings
-- messages parsed into rows, and the raw response kept for debugging
-- and GSE certification. Requests aren't stored — the MISMO export is
-- reproducible from the file at any time.
CREATE TABLE aus_submissions (
    id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id       UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id    UUID NOT NULL REFERENCES companies(id),
    system        TEXT NOT NULL,            -- 'du' | 'lpa'
    case_id       TEXT,                     -- engine-issued case file id
    recommendation TEXT,                    -- Approve/Eligible, Refer, ...
    messages      JSONB NOT NULL DEFAULT '[]', -- [{severity, code, text}]
    raw_response  TEXT,
    submitted_by  UUID REFERENCES users(id),
    submitted_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON aus_submissions(loan_id);
