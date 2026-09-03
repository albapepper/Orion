// App-wide signed-in identity plus the field-locking policy.
//
// This is the single place that decides who may edit what. It is dynamic:
// it always answers from the current session's user, and the backend
// enforces the same rule on every write (save_uw_review role check), so
// locking is real, not just visual.

import { createSignal } from 'solid-js';
import type { UserAccount } from '../types';

const [user, setUser] = createSignal<UserAccount | null>(null);

export { user, setUser };

export const isUnderwriter = (u: UserAccount | null): boolean =>
  u?.role === 'underwriter' || u?.role === 'admin';

const BANKER_ROLES = ['loan_officer', 'processor', 'admin'];
const UNDERWRITER_ROLES = ['underwriter', 'admin'];

/** The banker's work column — loan officers, processors, and admins. */
export const canEditWork = (u: UserAccount | null): boolean =>
  u !== null && BANKER_ROLES.includes(u.role);

/** The underwriter column — underwriters (and admins) only. */
export const canEditReview = (u: UserAccount | null): boolean =>
  u !== null && UNDERWRITER_ROLES.includes(u.role);

export const ROLE_LABELS: Record<string, string> = {
  loan_officer: 'Loan officer',
  processor: 'Processor',
  underwriter: 'Underwriter',
  admin: 'Admin',
};
