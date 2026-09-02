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
    <div class="screen">
      <header class="screen-header">
        <h1>Dashboard</h1>
      </header>

      <div class="pipeline-grid">
        <For each={[...LOAN_STATUSES]}>
          {(status) => (
            <div class="pipeline-card">
              <div class="pipeline-count">{countFor(status)}</div>
              <div class="pipeline-label">{status}</div>
            </div>
          )}
        </For>
      </div>

      <h2 class="section-title">Up next</h2>
      <Show
        when={workQueue().length > 0}
        fallback={<div class="screen-loading">Nothing needs attention. Pipeline is clear.</div>}
      >
        <table class="data-table">
          <thead>
            <tr>
              <th class="rank-col">#</th>
              <th>Loan</th>
              <th>Borrower</th>
              <th>Stage</th>
              <th>Idle</th>
              <th>Assigned to</th>
            </tr>
          </thead>
          <tbody>
            <For each={workQueue()}>
              {(loan, i) => (
                <tr class="clickable" onClick={() => props.onOpenLoan(loan)}>
                  <td class="rank-col mono">{i() + 1}</td>
                  <td class="mono">{loan.loan_number}</td>
                  <td>{loan.borrower_name ?? '—'}</td>
                  <td>
                    <span class={`status-pill ${loan.status}`}>{loan.status}</span>
                  </td>
                  <td class="mono">
                    {daysSince(loan.updated_at) === 0
                      ? 'today'
                      : `${daysSince(loan.updated_at)}d`}
                  </td>
                  <td>{loan.assigned_to_name ?? 'Unassigned'}</td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </Show>
    </div>
  );
}
