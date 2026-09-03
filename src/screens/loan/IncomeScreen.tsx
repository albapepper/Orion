import type { LoanIncome } from '../../types';
import { INCOME_TYPES } from '../../types';
import { LoanListScreen } from './LoanListScreen';

interface Props {
  loanId: string;
  demo: boolean;
  onNavigate: (screen: string) => void;
}

const money = (n: number) => `$${Math.round(n).toLocaleString()}`;

const twoYearAvg = (r: LoanIncome): number | null =>
  r.prior_year_monthly == null ? null : (r.prior_year_monthly + r.monthly_amount) / 2;

// Income Analysis (P3-16): now rides the shared LoanListScreen like
// assets, liabilities, and REO. The qualifying math stays in the
// footer — total monthly sources and the 2-year averages DU looks at.
export function IncomeScreen(props: Props) {
  return (
    <LoanListScreen<LoanIncome>
      loanId={props.loanId}
      demo={props.demo}
      onNavigate={props.onNavigate}
      screen="income"
      title="Income Analysis"
      addLabel="+ Add income source"
      emptyText="No income sources yet. Start with base employment income."
      columns={[
        { key: 'income_type', label: 'Type', type: 'select', options: [...INCOME_TYPES] },
        { key: 'monthly_amount', label: 'Monthly amount', type: 'number' },
        { key: 'prior_year_monthly', label: 'Prior year (mo)', type: 'number' },
        { key: 'employer_name', label: 'Employer', type: 'text', placeholder: 'If employment income' },
        { key: 'years_on_job', label: 'Years on job', type: 'number', step: '0.5' },
        { key: 'verified', label: 'Verified', type: 'check' },
        { key: 'notes', label: 'Notes', type: 'text' },
      ]}
      commands={{ save: 'save_income', remove: 'delete_income' }}
      saveArg="income"
      removeArg="incomeId"
      toPayload={(row) => ({
        id: row.id,
        borrower_id: row.borrower_id,
        income_type: row.income_type,
        monthly_amount: Number(row.monthly_amount) || 0,
        prior_year_monthly: row.prior_year_monthly,
        verified: row.verified,
        employer_name: row.employer_name,
        years_on_job: row.years_on_job,
        notes: row.notes,
      })}
      newRow={(b) => ({
        id: crypto.randomUUID(),
        borrower_id: b.id,
        loan_id: b.loan_id,
        company_id: b.company_id,
        income_type: 'base',
        monthly_amount: 0,
        prior_year_monthly: null,
        verified: false,
        employer_name: null,
        years_on_job: null,
        notes: null,
      })}
      summary={(rows) => (
        <tr>
          <td>Total monthly</td>
          <td class="mono">{money(rows.reduce((s, r) => s + (Number(r.monthly_amount) || 0), 0))}</td>
          <td>Total 2-yr avg</td>
          <td class="mono">
            {money(
              rows.reduce(
                (s, r) => s + (twoYearAvg(r) ?? (Number(r.monthly_amount) || 0)),
                0,
              ),
            )}
          </td>
          <td colspan="4" />
        </tr>
      )}
    />
  );
}
