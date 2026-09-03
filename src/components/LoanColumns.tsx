import { Show, type JSX } from 'solid-js';
import type { LockInfo } from '../types';
import { canEditWork, ROLE_LABELS, user } from '../lib/session';
import { ContactsPanel } from './ContactsPanel';

// Loan screens read in two columns: the banker's work on the left, the
// underwriter's notes and verifications on the right — anyone touching the
// loan sees the state of both. Editing is gated twice, by design: roles
// decide which column is yours, and the loan lock decides whether it's
// your turn at all — if another user holds the lock, the work column goes
// read-only and the badge up top says who's in the file. Below the
// columns sits the deal's contact directory (P5-10).
export function LoanColumns(props: {
  work: JSX.Element;
  review: JSX.Element;
  lock?: LockInfo | null;
  loanId?: string;
  demo?: boolean;
}) {
  const lockHeldByOther = () =>
    props.lock != null && props.lock.locked_by !== user()?.id;
  const roleLocked = () => user() !== null && !canEditWork(user());

  return (
    <>
    <div class="loan-columns">
      <section
        class="loan-col work"
        classList={{ locked: lockHeldByOther() || roleLocked() }}
      >
        <Show when={lockHeldByOther()}>
          <div class="work-lock-hint">
            <strong>{props.lock!.user_name}</strong> is editing this loan
            <Show when={props.lock!.screen}>
              {' '}
              — they're on the {props.lock!.screen} screen
            </Show>
            . This column is read-only until they're done.
          </div>
        </Show>
        <Show when={!lockHeldByOther() && roleLocked()}>
          <div class="work-lock-hint">
            Banker's column is read-only — you're signed in as{' '}
            <strong>{user()!.name}</strong> ({ROLE_LABELS[user()!.role] ?? user()!.role}).
            Switch personas on the Team screen (⌃m).
          </div>
        </Show>
        {props.work}
      </section>
      <aside class="loan-col review">{props.review}</aside>
    </div>
    <Show when={props.loanId}>
      <ContactsPanel loanId={props.loanId!} demo={props.demo ?? false} />
    </Show>
    </>
  );
}
