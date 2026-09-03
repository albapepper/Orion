import { Show, type JSX } from 'solid-js';
import { canEditWork, ROLE_LABELS, user } from '../lib/session';

// Loan screens read in two columns: the banker's work on the left, the
// underwriter's notes and verifications on the right — anyone touching the
// loan sees the state of both. Editing is role-gated per column: bankers
// work the left, underwriters the right, and neither can overwrite the
// other. Fields stack vertically inside each column so arrow keys fly
// straight down the page.
export function LoanColumns(props: { work: JSX.Element; review: JSX.Element }) {
  const lockWork = () => user() !== null && !canEditWork(user());

  return (
    <div class="loan-columns">
      <section class="loan-col work" classList={{ locked: lockWork() }}>
        <Show when={lockWork()}>
          <div class="work-lock-hint">
            Banker's column is read-only — you're signed in as{' '}
            <strong>{user()!.name}</strong> ({ROLE_LABELS[user()!.role] ?? user()!.role}).
            Switch personas in Settings (⌃s).
          </div>
        </Show>
        {props.work}
      </section>
      <aside class="loan-col review">{props.review}</aside>
    </div>
  );
}
