-- Tasks 2.0: the tasks table already carries assignee, status and
-- per-screen linkage; this adds scheduling and urgency so the unified
-- task view can triage: what's overdue, what's mine, what's urgent.
ALTER TABLE tasks
    ADD COLUMN due_date TIMESTAMPTZ,
    ADD COLUMN priority TEXT NOT NULL DEFAULT 'normal'; -- low | normal | high

CREATE INDEX ON tasks(company_id, assigned_to);
CREATE INDEX ON tasks(company_id, due_date);
