import { Show } from 'solid-js';
import type { LoanDetail } from '../types';
import { LockBadge } from './LockBadge';

interface Props {
  detail?: LoanDetail;
  screenLabel: string;
  onBack: () => void;
}

export function LoanHeader(props: Props) {
  const primaryBorrower = () =>
    props.detail?.borrowers.find((b) => b.is_primary) ?? props.detail?.borrowers[0];

  return (
    <header class="screen-header">
      <div>
        <button class="back-link" onClick={props.onBack} title="Back (Home)">
          ← Back
        </button>
        <h1>
          {props.detail?.summary.loan_number ?? '…'}
          <span class="screen-label"> · {props.screenLabel}</span>
        </h1>
        <div class="context-line">
          Borrower:{' '}
          <Show when={primaryBorrower()} fallback="—">
            {primaryBorrower()!.first_name} {primaryBorrower()!.last_name}
          </Show>
        </div>
      </div>
      <LockBadge lock={props.detail?.lock ?? null} currentUserId={null} />
    </header>
  );
}
