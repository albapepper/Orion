-- Underwriter review state per loan screen: notes + verification status.
-- Visible to everyone touching the loan; editable by underwriters only
-- (enforced in save_uw_review, not just in the UI).
CREATE TABLE loan_uw_reviews (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id     UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id  UUID NOT NULL REFERENCES companies(id),
    screen      TEXT NOT NULL,
    status      TEXT NOT NULL DEFAULT 'pending', -- pending | verified | flagged
    note        TEXT,
    updated_by  UUID REFERENCES users(id),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (loan_id, screen)
);
