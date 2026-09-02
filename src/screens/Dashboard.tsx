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

export function Dashboard(props: Props) {
  const workQueue = () =>
    props.loans
      .filter((l) => l.status !== 'funded')
      .slice()
      .sort((a, b) => {
        const urgency =
          (STATUS_URGENCY[b.status] ?? 1) - (STATUS_URGENCY[a.status] ?? 1);
        if (urgency !== 0) return urgency;
        // Same stage: stalest first
        return new Date(a.updated_at).getTime() - new Date(b.updated_at).getTime();
      });

  const countFor = (status: string) =>
    props.loans.filter((l) => l.status === status).length;

  return (
    <div class="screen dashboard">
      <div class="pipeline-strip">
        <For each={[...LOAN_STATUSES]}>
          {(status) => (
            <div class="pipeline-stat" classList={{ empty: countFor(status) === 0 }}>
              <span class="pipeline-count">{countFor(status)}</span>
              <span class="pipeline-label">{status}</span>
            </div>
          )}
        </For>
      </div>

      <h2 class="section-title">Up next</h2>
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
                <span class="queue-rank mono">{i() + 1}</span>
                <span class="queue-who">
                  <span class="queue-borrower">{loan.borrower_name ?? 'No borrower yet'}</span>
                  <span class="queue-loan mono">{loan.loan_number}</span>
                </span>
                <span class="queue-meta">
                  <span class={`status-pill ${loan.status}`}>{loan.status}</span>
                  <span class="queue-idle">{idleLabel(loan.updated_at)}</span>
                  <span class="queue-assigned">
                    {loan.assigned_to_name ?? 'Unassigned'}
                  </span>
                </span>
              </button>
            )}
          </For>
        </div>
      </Show>
    </div>
  );
}
