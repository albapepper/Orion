// The loan flow, domino-style: one round junction per stage, ringed in
// gray (empty), amber (in progress), red (problem) or green (complete).
//
// Keyboard flow: ← → walk the junctions, Enter opens one. Home from inside
// a field lands back here (see App.tsx), and Home again leaves the loan
// for the dashboard.

import { For, createSignal, onCleanup, onMount } from 'solid-js';
import type { LoanDetail } from '../types';
import {
  TRACKER_ITEMS,
  TRACKER_LABELS,
  computeStageStatuses,
  type StageStatus,
  type TrackerKey,
} from '../screens/loan/stageStatus';

interface Props {
  current: TrackerKey;
  detail?: LoanDetail;
  onNavigate: (screen: string) => void;
}

const STATUS_TEXT: Record<StageStatus, string> = {
  empty: 'Not started',
  progress: 'In progress',
  problem: 'Needs attention',
  done: 'Complete',
};

export function PizzaTracker(props: Props) {
  const [focused, setFocused] = createSignal<TrackerKey>(props.current);
  let listEl: HTMLUListElement | undefined;

  const statuses = () => computeStageStatuses(props.detail);

  const focusKey = (k: TrackerKey) => {
    setFocused(k);
    listEl?.querySelector<HTMLButtonElement>(`[data-key="${k}"]`)?.focus();
  };

  const focusCurrent = () => focusKey(props.current);

  onMount(() => {
    // Landing on a screen puts you at the tracker so ← → work right away —
    // unless you arrived by clicking into a control.
    requestAnimationFrame(() => {
      const ae = document.activeElement;
      if (
        ae === document.body ||
        (ae instanceof HTMLElement && ae.classList.contains('tracker-node'))
      ) {
        focusCurrent();
      }
    });
    window.addEventListener('orion:focus-tracker', focusCurrent);
  });
  onCleanup(() => window.removeEventListener('orion:focus-tracker', focusCurrent));

  const onKeyDown = (e: KeyboardEvent) => {
    const idx = TRACKER_ITEMS.indexOf(focused());
    if (e.key === 'ArrowRight') {
      e.preventDefault();
      focusKey(TRACKER_ITEMS[Math.min(idx + 1, TRACKER_ITEMS.length - 1)]);
    } else if (e.key === 'ArrowLeft') {
      e.preventDefault();
      focusKey(TRACKER_ITEMS[Math.max(idx - 1, 0)]);
    } else if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      props.onNavigate(focused());
    }
  };

  return (
    <nav class="tracker" aria-label="Loan flow">
      <ul class="tracker-track" role="tablist" onKeyDown={onKeyDown} ref={listEl}>
        <For each={TRACKER_ITEMS}>
          {(k) => (
            <li
              role="presentation"
              class="tracker-step"
              classList={{ current: k === props.current }}
              data-status={statuses()[k]}
            >
              <button
                type="button"
                role="tab"
                data-key={k}
                class="tracker-node"
                tabIndex={focused() === k ? 0 : -1}
                aria-selected={k === props.current}
                title={`${TRACKER_LABELS[k]} — ${STATUS_TEXT[statuses()[k]]}`}
                onClick={() => props.onNavigate(k)}
                onFocus={() => setFocused(k)}
              >
                <span class="tracker-ring">
                  <span class="tracker-core" />
                </span>
                <span class="tracker-label">{TRACKER_LABELS[k]}</span>
              </button>
            </li>
          )}
        </For>
      </ul>
    </nav>
  );
}
