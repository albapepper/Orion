import { For, Show } from 'solid-js';

interface Props {
  open: boolean;
  onClose: () => void;
}

const BINDINGS: Array<[string, string]> = [
  ['d', 'Dashboard'],
  ['i', 'Income screen'],
  ['a', 'Assets screen'],
  ['c', 'Credit screen'],
  ['p', 'Property screen'],
  ['1–9', 'Open nth recent loan'],
  ['/', 'Focus the assistant'],
  ['[', 'Collapse / expand nav tray'],
  ['Home', 'Back'],
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
            Orion is designed to be fully operable without a mouse.
          </div>
        </div>
      </div>
    </Show>
  );
}
