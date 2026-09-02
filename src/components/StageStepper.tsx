import { For } from 'solid-js';
import { LOAN_SCREENS, SCREEN_LABELS, STAGE_KEYS, type LoanScreen } from '../screens/loan/stages';

interface Props {
  current: LoanScreen;
  onNavigate: (screen: LoanScreen) => void;
}

// Makes the cascade tangible: where you are, what came before, what's next.
export function StageStepper(props: Props) {
  const stageIndex = () => LOAN_SCREENS.indexOf(props.current);

  return (
    <div class="stage-stepper" role="tablist" aria-label="Loan stages">
      <For each={LOAN_SCREENS}>
        {(stage, i) => (
          <>
            <button
              role="tab"
              aria-selected={stage === props.current}
              class="stage-step"
              classList={{ active: stage === props.current, done: i() < stageIndex() }}
              onClick={() => props.onNavigate(stage)}
            >
              <kbd class="keycap">⌃{STAGE_KEYS[stage]}</kbd>
              <span class="stage-label">{SCREEN_LABELS[stage]}</span>
            </button>
            {i() < LOAN_SCREENS.length - 1 && <span class="stage-arrow">→</span>}
          </>
        )}
      </For>
    </div>
  );
}
