// Placeholder for loan screens not yet built (assets, credit, property).
// Fetches real loan context so the header, lock state, and tasks are live.

import { createResource } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanDetail } from '../../types';
import { TaskPanel } from '../../components/TaskPanel';
import { LoanHeader } from '../../components/LoanHeader';

interface Props {
  loanId: string;
  screen: string; // "assets" | "credit" | "property"
  label: string;
  onBack: () => void;
}

export function StubScreen(props: Props) {
  const [detail] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId }),
  );

  return (
    <div class="screen">
      <LoanHeader detail={detail()} screenLabel={props.label} onBack={props.onBack} />

      <div class="stub-panel">
        <div class="stub-title">{props.label}</div>
        <p>
          This screen is on the roadmap. The assistant already answers in the
          context of the {props.screen} screen, and tasks filed here are tagged
          to it.
        </p>
      </div>

      <TaskPanel loanId={props.loanId} screen={props.screen} />
    </div>
  );
}
