import { For, Show } from 'solid-js';

interface Props {
  open: boolean;
  onClose: () => void;
}

const BINDINGS: Array<[string, string]> = [
  ['⌃d', 'Dashboard'],
  ['⌃n', 'Loan list'],
  ['⌃k', 'Tasks'],
  ['⌃y', 'Rate sheets'],
  ['⌃s', 'Settings'],
  ['⌃1–9', 'Open nth recent loan'],
  ['⌃o', 'Loan overview'],
  ['⌃b', 'Borrower Info screen'],
  ['⌃e', 'Employment screen'],
  ['⌃p', 'Property screen'],
  ['⌃t', 'Loan Terms screen'],
  ['⌃c', 'Credit screen'],
  ['⌃i', 'Income screen'],
  ['⌃a', 'Assets screen'],
  ['⌃l', 'Liabilities screen'],
  ['⌃r', 'REO screen'],
  ['⌃q', 'Declarations screen'],
  ['⌃g', 'Demographic (HMDA) screen'],
  ['⌃f', 'Documents screen'],
  ['⌃h', 'Conditions screen'],
  ['⌃v', 'Pricing screen'],
  ['⌃]', 'Next loan stage'],
  ['⌃[', 'Previous loan stage'],
  ['← →', 'Walk the loan tracker'],
  ['Enter', 'Open the highlighted tracker stage'],
  ['↑ ↓', 'Move between fields'],
  ['Home', 'Back: field → tracker → dashboard'],
  ['/', 'Focus the assistant'],
  ['?', 'Toggle this legend'],
  ['Esc', 'Close / unfocus'],
];

export function KeyLegend(props: Props) {
  return (
    <Show when={props.open}>
      <div class="legend-overlay" onClick={props.onClose}>
        <div class="legend-panel" onClick={(e) => e.stopPropagation()}>
          <div class="legend-title">Keyboard</div>
          <div class="legend-grid">
            <For each={BINDINGS}>
              {([key, action]) => (
                <>
                  <kbd class="keycap">{key}</kbd>
                  <span class="legend-action">{action}</span>
                </>
              )}
            </For>
          </div>
          <div class="legend-note">
            Loan screens are keyboard-first: land on a loan and the tracker
            has focus — arrows pick a stage, Enter enters it, arrows walk
            the fields down each column. Home backs out one level at a time:
            field → tracker → dashboard. The dashboard, tray, and assistant
            are ordinary mouse-driven surfaces.
          </div>
        </div>
      </div>
    </Show>
  );
}
