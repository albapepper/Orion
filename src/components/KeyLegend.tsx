import { For, Show } from 'solid-js';

interface Props {
  open: boolean;
  onClose: () => void;
}

const BINDINGS: Array<[string, string]> = [
  ['⌃d', 'Dashboard'],
  ['⌃p', 'Property screen'],
  ['⌃c', 'Credit screen'],
  ['⌃i', 'Income screen'],
  ['⌃a', 'Assets screen'],
  ['⌃1–9', 'Open nth recent loan'],
  ['⌃]', 'Next loan stage'],
  ['⌃[', 'Previous loan stage'],
  ['/', 'Focus the assistant'],
  ['Home', 'Back to previous page'],
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
            Orion is designed to be fully operable without a mouse. Loan
            screens cascade: property → credit → income → assets.
          </div>
        </div>
      </div>
    </Show>
  );
}
