import { For, Show } from 'solid-js';
import type { LoanSummary } from '../types';
import { LOAN_STATUSES } from '../types';

interface Props {
  loans: LoanSummary[];
  onOpenLoan: (loan: LoanSummary) => void;
}

// How urgently each stage needs a person's attention. Higher = sooner.
const STATUS_URGENCY: Record<string, number> = {
  closing: 6,
  approval: 5,
  underwriting: 4,
  processing: 3,
  application: 2,
  funded: 0,
};

const daysSince = (iso: string) =>
  Math.floor((Date.now() - new Date(iso).getTime()) / 86_400_000);

const idleLabel = (iso: string) => {
  const d = daysSince(iso);
  if (d === 0) return 'touched today';
  if (d === 1) return 'idle 1 day';
  return `idle ${d} days`;
};

const nextStep = (loan: LoanSummary) => {
  if (loan.flagged_reviews > 0) return 'Resolve the flagged underwriting review';
  if (loan.status === 'closing') return 'Confirm the closing package and final figures';
  if (loan.status === 'approval') return 'Review approval terms and outstanding conditions';
  if (loan.status === 'underwriting') return 'Check the file for a new underwriting decision';
  if (loan.status === 'processing') return 'Complete the next open application item';
  return 'Continue the borrower application';
};

const priorityLabel = (loan: LoanSummary) =>
  loan.flagged_reviews > 0 || ['closing', 'approval'].includes(loan.status) ? 'Priority' : 'Next';

export function Dashboard(props: Props) {
  // The work queue is deterministic — flags first, then stage urgency,
  // then stalest first — so it agrees with the AI's priority order.
  const workQueue = () =>
    props.loans
      .filter((l) => l.status !== 'funded')
      .slice()
      .sort((a, b) => {
        if (a.flagged_reviews !== b.flagged_reviews)
          return b.flagged_reviews - a.flagged_reviews;
        const urgency =
          (STATUS_URGENCY[b.status] ?? 1) - (STATUS_URGENCY[a.status] ?? 1);
        if (urgency !== 0) return urgency;
        return new Date(a.updated_at).getTime() - new Date(b.updated_at).getTime();
      });

  const countFor = (status: string) =>
    props.loans.filter((l) => l.status === status).length;

  return (
    <div class="screen dashboard">
      <header class="home-header">
        <div>
          <span class="home-eyebrow">Your workspace</span>
          <h1>Good {new Date().getHours() < 12 ? 'morning' : new Date().getHours() < 18 ? 'afternoon' : 'evening'}.</h1>
          <p>A clear view of every file on the path to homeownership.</p>
        </div>
        <div class="home-date">
          <span>{new Date().toLocaleDateString(undefined, { weekday: 'long' })}</span>
          <strong>{new Date().toLocaleDateString(undefined, { month: 'short', day: 'numeric' })}</strong>
        </div>
      </header>

      <div class="route-heading">
        <span>Path to homeownership</span>
        <span class="route-direction">Application <span aria-hidden="true">→</span> Funded</span>
      </div>
      <div class="pipeline-strip" aria-label="Loan journey from application to funded">
        <For each={[...LOAN_STATUSES]}>
          {(status) => (
            <div class="pipeline-stat" classList={{ empty: countFor(status) === 0 }} data-stage={status}>
              <span class="pipeline-count">{countFor(status)}</span>
              <span class="pipeline-label">{status}</span>
              <Show when={status === 'funded'}>
                <span class="summit-label">Home</span>
              </Show>
            </div>
          )}
        </For>
      </div>

      <div class="home-section-heading">
        <div>
          <span class="home-eyebrow">Today</span>
          <h2>Suggested next steps</h2>
        </div>
        <span class="suggested-count">{workQueue().length} moves ahead</span>
      </div>
      <Show
        when={workQueue().length > 0}
        fallback={
          <div class="screen-loading">Nothing needs attention. Pipeline is clear.</div>
        }
      >
        <div class="queue">
          <For each={workQueue()}>
            {(loan, i) => (
              <button class="queue-card" onClick={() => props.onOpenLoan(loan)}>
                <span class="queue-check" aria-hidden="true"><span>{i() + 1}</span></span>
                <span class="queue-who">
                  <span class="queue-action">{nextStep(loan)}</span>
                  <span class="queue-borrower">{loan.borrower_name ?? 'No borrower yet'} <span>·</span> <span class="queue-loan mono">{loan.loan_number}</span></span>
                </span>
                <span class="queue-meta">
                  <span class="queue-priority" classList={{ urgent: priorityLabel(loan) === 'Priority' }}>{priorityLabel(loan)}</span>
                  <span class={`status-pill ${loan.status}`}>{loan.status}</span>
                  <span class="queue-detail"><span>{idleLabel(loan.updated_at)}</span><span>{loan.assigned_to_name ?? 'Unassigned'}</span></span>
                  <span class="queue-arrow" aria-hidden="true">→</span>
                </span>
              </button>
            )}
          </For>
        </div>
      </Show>
    </div>
  );
}
