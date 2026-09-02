import { Show } from 'solid-js';
import type { LoanDetail } from '../types';
import { LockBadge } from './LockBadge';

interface Props {
  detail?: LoanDetail;
}

// Inside a loan it should feel like you're in just that one loan:
// loan number + applicant, centered. Nothing else competes.
export function LoanHeader(props: Props) {
  const primaryBorrower = () =>
    props.detail?.borrowers.find((b) => b.is_primary) ?? props.detail?.borrowers[0];

  return (
    <header class="loan-header">
      <div class="loan-header-lock">
        <LockBadge lock={props.detail?.lock ?? null} currentUserId={null} />
      </div>
      <div class="loan-header-number">{props.detail?.summary.loan_number ?? '…'}</div>
      <div class="loan-header-name">
        <Show when={primaryBorrower()} fallback="—">
          {primaryBorrower()!.first_name} {primaryBorrower()!.last_name}
        </Show>
      </div>
    </header>
  );
}
