// Per-stage completion state for the loan tracker.
// gray = empty, amber = in progress, red = problem, green = complete.
//
// Statuses are derived from the loan detail itself — no separate tracking
// record to keep in sync. "Problem" means data exists but a row is missing
// facts it needs (a job without an employer, a "yes" without an explanation).

import type { LoanDetail } from '../../types';
import { LOAN_SCREENS, SCREEN_LABELS, type LoanScreen } from './stages';

export type StageStatus = 'empty' | 'progress' | 'problem' | 'done';

export type TrackerKey = 'overview' | LoanScreen;

// The tracker row: the loan's home, then the cascade in working order.
export const TRACKER_ITEMS: TrackerKey[] = ['overview', ...LOAN_SCREENS];

export const TRACKER_LABELS: Record<TrackerKey, string> = {
  overview: 'Overview',
  ...SCREEN_LABELS,
};

const present = (v: unknown): boolean =>
  v !== null && v !== undefined && v !== '' && !(typeof v === 'number' && Number.isNaN(v));

const score = (required: number, filled: number, issues = 0): StageStatus => {
  if (issues > 0) return 'problem';
  if (filled === 0) return 'empty';
  return filled >= required ? 'done' : 'progress';
};

export function computeStageStatuses(
  detail?: LoanDetail,
): Record<TrackerKey, StageStatus> {
  if (!detail) {
    return Object.fromEntries(
      TRACKER_ITEMS.map((k) => [k, 'empty']),
    ) as Record<TrackerKey, StageStatus>;
  }

  const borrowers = detail.borrowers ?? [];
  const primary = borrowers.find((b) => b.is_primary) ?? borrowers[0];
  const borrowerIds = new Set(borrowers.map((b) => b.id));
  const statuses = {} as Record<TrackerKey, StageStatus>;

  // Borrower profile: the identity facts a 1003 cannot go without
  if (primary) {
    const fields = [
      primary.first_name,
      primary.last_name,
      primary.ssn,
      primary.birth_date,
      primary.marital_status,
      primary.citizenship_status,
    ];
    statuses.borrower = score(fields.length, fields.filter(present).length);
  } else {
    statuses.borrower = 'empty';
  }

  // Employment: every job needs an employer, an income, and a start date
  const jobs = (detail.employments ?? []).filter((e) => borrowerIds.has(e.borrower_id));
  statuses.employment = jobs.length
    ? score(
        jobs.length * 3,
        jobs.reduce(
          (s, j) => s + [j.employer_name, j.monthly_income, j.start_date].filter(present).length,
          0,
        ),
        jobs.filter(
          (j) =>
            !present(j.employer_name) || !present(j.monthly_income) || !present(j.start_date),
        ).length,
      )
    : 'empty';

  // Property: the subject address and its value
  const p = detail.property;
  if (p) {
    const fields = [p.address_line, p.city, p.state, p.postal_code, p.estimated_value];
    statuses.property = score(fields.length, fields.filter(present).length);
  } else {
    statuses.property = 'empty';
  }

  // Terms: the deal shape
  const t = detail.terms;
  if (t) {
    const fields = [t.loan_amount, t.interest_rate, t.term_months, t.property_value];
    statuses.terms = score(fields.length, fields.filter(present).length);
  } else {
    statuses.terms = 'empty';
  }

  // Credit is pulled from an external source; nothing to gather yet
  statuses.credit = 'empty';

  // Income: every source needs an amount
  const income = (detail.income ?? []).filter((r) => borrowerIds.has(r.borrower_id));
  statuses.income = income.length
    ? score(
        income.length,
        income.filter((r) => r.monthly_amount > 0).length,
        income.filter((r) => !r.monthly_amount || r.monthly_amount <= 0).length,
      )
    : 'empty';

  // Assets: each account carries an institution and a balance
  const assets = (detail.assets ?? []).filter((r) => borrowerIds.has(r.borrower_id));
  statuses.assets = assets.length
    ? score(
        assets.length * 2,
        assets.reduce((s, a) => s + [a.institution, a.balance].filter(present).length, 0),
      )
    : 'empty';

  // Liabilities: each debt carries a creditor and a payment
  const liabilities = (detail.liabilities ?? []).filter((r) => borrowerIds.has(r.borrower_id));
  statuses.liabilities = liabilities.length
    ? score(
        liabilities.length * 2,
        liabilities.reduce(
          (s, l) => s + [l.creditor_name, l.monthly_payment].filter(present).length,
          0,
        ),
      )
    : 'empty';

  // REO: each property carries an address and a value
  const reo = (detail.reo ?? []).filter((r) => borrowerIds.has(r.borrower_id));
  statuses.reo = reo.length
    ? score(
        reo.length * 2,
        reo.reduce((s, r) => s + [r.address_line, r.market_value].filter(present).length, 0),
      )
    : 'empty';

  // Declarations: every question answered; "yes" needs an explanation
  const decls = detail.declarations ?? [];
  statuses.declarations = decls.length
    ? score(
        decls.length,
        decls.filter((d) => d.answer !== null).length,
        decls.filter((d) => d.answer === true && !present(d.explanation)).length,
      )
    : 'empty';

  // Demographic (HMDA): the primary borrower's collection
  const demo = (detail.demographics ?? []).find((d) => d.borrower_id === primary?.id);
  if (demo) {
    const fields = [demo.ethnicity, demo.race, demo.sex, demo.collection_basis];
    statuses.demographic = score(fields.length, fields.filter(present).length);
  } else {
    statuses.demographic = 'empty';
  }

  // Documents: the file's paperwork. "Some" is the best it ever reads —
  // a file is never really done collecting documents, so it tops out at
  // in-progress and doesn't gate the overview's green.
  statuses.documents = (detail.documents ?? []).length > 0 ? 'progress' : 'empty';

  // Conditions: underwriting's checklist. Outstanding = work remaining;
  // anything past its clear-by date is a problem.
  const conditions = detail.conditions ?? [];
  statuses.conditions = conditions.length
    ? conditions.every((c) => c.status === 'cleared' || c.status === 'waived')
      ? 'done'
      : conditions.some(
            (c) =>
              (c.status === 'open' || c.status === 'submitted') &&
              c.clear_by !== null &&
              new Date(c.clear_by) < new Date(),
          )
        ? 'problem'
        : 'progress'
    : 'empty';

  // Overview summarizes the file — credit is external and documents are
  // never truly "done", so neither blocks green
  const vals = TRACKER_ITEMS.filter(
    (k) => k !== 'overview' && k !== 'credit' && k !== 'documents',
  ).map(
    (k) => statuses[k],
  );
  statuses.overview = vals.includes('problem')
    ? 'problem'
    : vals.every((v) => v === 'done')
      ? 'done'
      : vals.some((v) => v !== 'empty')
        ? 'progress'
        : 'empty';

  return statuses;
}
