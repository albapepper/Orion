-- P5-03: team management. Deactivation over deletion — audit history,
-- locks, and past submissions all reference users, so leaving the team
-- means flipping a flag, not breaking foreign keys.
ALTER TABLE users ADD COLUMN active BOOLEAN NOT NULL DEFAULT true;

COMMENT ON COLUMN users.active IS
    'false = deactivated: hidden from sign-in, kept for audit history';
