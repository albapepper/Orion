CREATE TABLE tasks (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),
    title           TEXT NOT NULL,
    description     TEXT,
    assigned_to     UUID REFERENCES users(id),
    screen          TEXT,  -- which screen this task belongs to
    status          TEXT NOT NULL DEFAULT 'open',
    -- statuses: open | in_progress | done
    created_by      UUID NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at    TIMESTAMPTZ
);

CREATE INDEX ON tasks(loan_id);
CREATE INDEX ON tasks(company_id, status);

-- Loan locking: first user in gets the lock, others are read-only
CREATE TABLE loan_locks (
    loan_id     UUID PRIMARY KEY REFERENCES loans(id) ON DELETE CASCADE,
    locked_by   UUID NOT NULL REFERENCES users(id),
    locked_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    screen      TEXT  -- which screen they're currently on
);

-- Immutable audit log: every data change goes here
CREATE TABLE audit_log (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id  UUID NOT NULL REFERENCES companies(id),
    loan_id     UUID REFERENCES loans(id),
    user_id     UUID NOT NULL REFERENCES users(id),
    action      TEXT NOT NULL,     -- e.g. "income.update"
    table_name  TEXT NOT NULL,
    record_id   UUID NOT NULL,
    old_value   JSONB,
    new_value   JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON audit_log(loan_id);
CREATE INDEX ON audit_log(company_id, created_at);
