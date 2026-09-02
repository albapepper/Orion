import type { ReoProperty } from '../../types';
import { LoanListScreen } from './LoanListScreen';

const REO_TYPES = ['single_family', 'condo', 'multi_family', 'land', 'commercial', 'other'];
const DISPOSITIONS = ['retain', 'pending_sale', 'sold', 'pending_removal'];

interface Props {
  loanId: string;
  demo: boolean;
  onNavigate: (screen: string) => void;
}

export function ReoScreen(props: Props) {
  return (
    <LoanListScreen<ReoProperty>
      loanId={props.loanId}
      demo={props.demo}
      onNavigate={props.onNavigate}
      screen="reo"
      title="Real estate owned"
      addLabel="+ Add property"
      emptyText="No other properties owned."
      columns={[
        { key: 'address_line', label: 'Address', type: 'text', placeholder: 'Street address' },
        { key: 'city', label: 'City', type: 'text' },
        { key: 'state', label: 'State', type: 'text' },
        { key: 'property_type', label: 'Type', type: 'select', options: [...REO_TYPES] },
        { key: 'market_value', label: 'Market value', type: 'number' },
        { key: 'mortgage_payment', label: 'Payment/mo', type: 'number' },
        { key: 'rental_income', label: 'Rent/mo', type: 'number' },
        { key: 'disposition', label: 'Disposition', type: 'select', options: [...DISPOSITIONS] },
        { key: 'notes', label: 'Notes', type: 'text' },
      ]}
      commands={{ save: 'save_reo', remove: 'delete_reo' }}
      saveArg="record"
      removeArg="reoId"
      toPayload={(row) => ({
        id: row.id,
        borrower_id: row.borrower_id,
        address_line: row.address_line,
        city: row.city,
        state: row.state,
        postal_code: row.postal_code,
        property_type: row.property_type,
        market_value: row.market_value,
        mortgage_payment: row.mortgage_payment,
        rental_income: row.rental_income,
        disposition: row.disposition,
        notes: row.notes,
      })}
      newRow={(b) => ({
        id: crypto.randomUUID(),
        borrower_id: b.id,
        loan_id: b.loan_id,
        company_id: b.company_id,
        address_line: null,
        city: null,
        state: null,
        postal_code: null,
        property_type: 'single_family',
        market_value: null,
        mortgage_payment: null,
        rental_income: null,
        disposition: 'retain',
        notes: null,
      })}
    />
  );
}
