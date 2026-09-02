import type { Liability } from '../../types';
import { LoanListScreen } from './LoanListScreen';

const LIABILITY_TYPES = ['mortgage', 'installment', 'revolving', 'open', 'other'];

interface Props {
  loanId: string;
  demo: boolean;
  onNavigate: (screen: string) => void;
}

export function LiabilitiesScreen(props: Props) {
  return (
    <LoanListScreen<Liability>
      loanId={props.loanId}
      demo={props.demo}
      onNavigate={props.onNavigate}
      screen="liabilities"
      title="Liabilities"
      addLabel="+ Add liability"
      emptyText="No liabilities yet. Add every debt the borrower is obligated on."
      columns={[
        { key: 'liability_type', label: 'Type', type: 'select', options: [...LIABILITY_TYPES] },
        { key: 'creditor_name', label: 'Creditor', type: 'text', placeholder: 'Creditor name' },
        { key: 'monthly_payment', label: 'Monthly payment', type: 'number' },
        { key: 'unpaid_balance', label: 'Unpaid balance', type: 'number' },
        { key: 'payoff_at_close', label: 'Pay off at close', type: 'check' },
        { key: 'notes', label: 'Notes', type: 'text' },
      ]}
      commands={{ save: 'save_liability', remove: 'delete_liability' }}
      saveArg="liability"
      removeArg="liabilityId"
      toPayload={(row) => ({
        id: row.id,
        borrower_id: row.borrower_id,
        liability_type: row.liability_type,
        creditor_name: row.creditor_name,
        monthly_payment: row.monthly_payment,
        unpaid_balance: row.unpaid_balance,
        payoff_at_close: row.payoff_at_close,
        notes: row.notes,
      })}
      newRow={(b) => ({
        id: crypto.randomUUID(),
        borrower_id: b.id,
        loan_id: b.loan_id,
        company_id: b.company_id,
        liability_type: 'installment',
        creditor_name: null,
        monthly_payment: null,
        unpaid_balance: null,
        payoff_at_close: false,
        notes: null,
      })}
      summary={(rows) => (
        <tr>
          <td>Total monthly payments</td>
          <td class="mono">
            $
            {rows
              .filter((r) => !r.payoff_at_close)
              .reduce((s, r) => s + Number(r.monthly_payment ?? 0), 0)
              .toLocaleString()}
            /mo
          </td>
          <td colspan="4">Items marked “pay off at close” are excluded from DTI.</td>
        </tr>
      )}
    />
  );
}
