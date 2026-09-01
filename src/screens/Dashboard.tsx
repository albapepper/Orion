import { createResource, For } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanSummary } from '../types';
import { LOAN_STATUSES } from '../types';

interface Props {
  onOpenLoans: () => void;
}

export function Dashboard(props: Props) {
  const [loans] = createResource(() => invoke<LoanSummary[]>('list_loans'));

  const countFor = (status: string) =>
    (loans() ?? []).filter((l) => l.status === status).length;

  return (
    <div class="screen">
      <header class="screen-header">
        <h1>Pipeline</h1>
        <button class="primary" onClick={props.onOpenLoans}>
          View all loans
        </button>
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
    </div>
  );
}
