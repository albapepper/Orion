import { createResource, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanSummary } from '../types';

interface Props {
  onOpenLoan: (loan: LoanSummary) => void;
}

export function LoanList(props: Props) {
  const [loans] = createResource(() => invoke<LoanSummary[]>('list_loans'));

  return (
    <div class="screen">
      <header class="screen-header">
        <h1>Loans</h1>
      </header>

      <Show when={loans()} fallback={<div class="screen-loading">Loading loans…</div>}>
        <table class="data-table">
          <thead>
            <tr>
              <th>Loan #</th>
              <th>Borrower</th>
              <th>Status</th>
              <th>Assigned to</th>
              <th>Updated</th>
            </tr>
          </thead>
          <tbody>
            <For each={loans()}>
              {(loan) => (
                <tr class="clickable" onClick={() => props.onOpenLoan(loan)}>
                  <td class="mono">{loan.loan_number}</td>
                  <td>{loan.borrower_name ?? '—'}</td>
                  <td>
                    <span class={`status-pill ${loan.status}`}>{loan.status}</span>
                  </td>
                  <td>{loan.assigned_to_name ?? 'Unassigned'}</td>
                  <td>{new Date(loan.updated_at).toLocaleDateString()}</td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </Show>
    </div>
  );
}
