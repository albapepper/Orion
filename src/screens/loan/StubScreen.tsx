// Placeholder for loan screens not yet built (assets, credit, property).
// Fetches real loan context so the header, lock state, and tasks are live.

import { createResource, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanDetail } from '../../types';
import { TaskPanel } from '../../components/TaskPanel';
import { LoanHeader } from '../../components/LoanHeader';
import { StageStepper } from '../../components/StageStepper';
import { nextStage, SCREEN_LABELS, type LoanScreen } from './stages';

interface Props {
  loanId: string;
  screen: LoanScreen;
  onNavigate: (screen: string) => void;
}

export function StubScreen(props: Props) {
  const [detail] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId }),
  );

  const upcoming = () => nextStage(props.screen);

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <StageStepper current={props.screen} onNavigate={props.onNavigate} />

      <div class="stub-panel">
        <div class="stub-title">{SCREEN_LABELS[props.screen]}</div>
        <p>
          This screen is on the roadmap. The assistant already answers in the
          context of the {props.screen} screen, and tasks filed here are tagged
          to it.
        </p>
        <Show when={upcoming()}>
          <button class="stub-next" onClick={() => props.onNavigate(upcoming()!)}>
            <span>
              Up next: {SCREEN_LABELS[upcoming()!]}
            </span>
            <kbd class="keycap">⌃]</kbd>
          </button>
        </Show>
      </div>

      <TaskPanel loanId={props.loanId} screen={props.screen} />
    </div>
  );
}
