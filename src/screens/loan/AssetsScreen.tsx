import type { Asset } from '../../types';
import { LoanListScreen } from './LoanListScreen';

const ACCOUNT_TYPES = ['checking', 'savings', 'retirement', 'mutual_fund', 'gift', 'other'];

interface Props {
  loanId: string;
  demo: boolean;
  onNavigate: (screen: string) => void;
}

export function AssetsScreen(props: Props) {
  return (
    <LoanListScreen<Asset>
      loanId={props.loanId}
      demo={props.demo}
      onNavigate={props.onNavigate}
      screen="assets"
      title="Assets"
      addLabel="+ Add asset"
      emptyText="No assets yet. Start with the largest account."
      columns={[
        { key: 'account_type', label: 'Account type', type: 'select', options: [...ACCOUNT_TYPES] },
        { key: 'institution', label: 'Institution', type: 'text', placeholder: 'Bank name' },
        { key: 'balance', label: 'Balance', type: 'number' },
        { key: 'gift_donor', label: 'Gift donor', type: 'text', placeholder: 'For gift funds' },
        { key: 'gift_relationship', label: 'Relationship', type: 'text', placeholder: 'e.g. parent' },
        { key: 'notes', label: 'Notes', type: 'text' },
      ]}
      commands={{ save: 'save_asset', remove: 'delete_asset' }}
      saveArg="asset"
      removeArg="assetId"
      toPayload={(row) => ({
        id: row.id,
        borrower_id: row.borrower_id,
        account_type: row.account_type,
        institution: row.institution,
        balance: Number(row.balance) || 0,
        gift_donor: row.gift_donor,
        gift_relationship: row.gift_relationship,
        notes: row.notes,
      })}
      newRow={(b) => ({
        id: crypto.randomUUID(),
        borrower_id: b.id,
        loan_id: b.loan_id,
        company_id: b.company_id,
        account_type: 'checking',
        institution: null,
        balance: 0,
        gift_donor: null,
        gift_relationship: null,
        notes: null,
      })}
      summary={(rows) => (
        <tr>
          <td>Total assets</td>
          <td class="mono">
            ${rows.reduce((s, r) => s + Number(r.balance), 0).toLocaleString()}
          </td>
          <td colspan="5" />
        </tr>
      )}
    />
  );
}
