import { Show } from 'solid-js';

// P5-11: the first-run walkthrough. Shows once, remembered locally —
// three things a new user needs: where the work starts, how demo mode
// differs from the real database, and that every screen is reachable
// from the keyboard.
export function Walkthrough(props: {
  open: boolean;
  demo: boolean;
  onDismiss: () => void;
}) {
  return (
    <Show when={props.open}>
      <div class="tour-overlay" onClick={props.onDismiss}>
        <div class="tour-card" onClick={(e) => e.stopPropagation()}>
          <h2 class="wizard-title">Welcome to Orion</h2>
          <p class="wizard-text">
            An AI-native loan origination platform that runs on your own
            machine. Three things worth knowing before you dive in:
          </p>
          <ul class="tour-list">
            <li>
              <kbd class="keycap">⌃N</kbd> opens the whole book of loans —
              create one with <strong>+ New loan</strong>, and the tracker at
              the top of every file walks the pipeline from borrower to funded.
            </li>
            <li>
              <Show
                when={props.demo}
                fallback={
                  <>
                    You're connected to your real database — every edit
                    saves, every action lands in the audit log.
                  </>
                }
              >
                You're in <strong>demo mode</strong> — a full cast of demo
                loans and teammates to explore with. Connect your own
                Postgres any time in Settings (⌃S).
              </Show>
            </li>
            <li>
              Every screen is reachable without the mouse — press{' '}
              <kbd class="keycap">?</kbd> for the full keyboard map, and{' '}
              <kbd class="keycap">/</kbd> to ask the assistant anything.
            </li>
          </ul>
          <div class="wizard-actions">
            <button class="ghost" onClick={props.onDismiss}>
              Take me in →
            </button>
            <span class="wizard-note">You won't see this again.</span>
          </div>
        </div>
      </div>
    </Show>
  );
}
