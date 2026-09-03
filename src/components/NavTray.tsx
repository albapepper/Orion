import { For, Show } from 'solid-js';
import type { AppStatus, LoanSummary, UserAccount } from '../types';
import { ROLE_LABELS } from '../lib/session';

export interface NavItem {
  id: string;
  label: string;
  key: string; // ctrl+key shortcut, shown as a keycap
}

// App navigation only. Everything about a specific loan — its flow, its
// stages — lives in the tracker at the top of the loan page.
const APP_ITEMS: NavItem[] = [
  { id: 'dashboard', label: 'Dashboard', key: 'd' },
  { id: 'loans', label: 'Loans', key: 'n' },
  { id: 'tasks', label: 'Tasks', key: 'k' },
  { id: 'rates', label: 'Rate sheets', key: 'y' },
  { id: 'settings', label: 'Settings', key: 's' },
];

interface Props {
  activeNav: string;
  onNavigate: (id: string) => void;
  recentLoans: LoanSummary[];
  activeLoanId: string | null;
  onOpenLoan: (loan: LoanSummary) => void;
  onShowLegend: () => void;
  status?: AppStatus;
  user?: UserAccount | null;
}

export function NavTray(props: Props) {
  return (
    <nav class="nav-tray">
      <div class="tray-brand">
        <span class="brand-mark">◆</span>
        <span class="brand-name">Orion</span>
      </div>

      <div class="tray-section">
        <Show when={props.user}>
          {(u) => (
            <button
              class="tray-item identity"
              title="Switch persona — Settings"
              onClick={() => props.onNavigate('settings')}
            >
              <span class="identity-avatar">{u().name.charAt(0)}</span>
              <span class="tray-label">
                <span class="identity-name">{u().name}</span>
                <span class="loan-borrower">{ROLE_LABELS[u().role] ?? u().role}</span>
              </span>
            </button>
          )}
        </Show>
        <For each={APP_ITEMS}>
          {(item) => (
            <button
              class="tray-item"
              classList={{ active: props.activeNav === item.id }}
              title={`${item.label} (Ctrl+${item.key.toUpperCase()})`}
              onClick={() => props.onNavigate(item.id)}
            >
              <kbd class="keycap">⌃{item.key}</kbd>
              <span class="tray-label">{item.label}</span>
            </button>
          )}
        </For>
      </div>

      <div class="tray-section recent">
        <div class="tray-heading">Recent loans</div>
        <For each={props.recentLoans.slice(0, 9)}>
          {(loan, i) => (
            <button
              class="tray-item loan"
              classList={{ active: props.activeLoanId === loan.id }}
              onClick={() => props.onOpenLoan(loan)}
              title={`Open ${loan.loan_number} (Ctrl+${i() + 1})`}
            >
              <kbd class="keycap">⌃{i() + 1}</kbd>
              <span class="tray-label">
                <span class="loan-number">{loan.loan_number}</span>
                <span class="loan-borrower">{loan.borrower_name ?? '—'}</span>
              </span>
            </button>
          )}
        </For>
      </div>

      <div class="tray-footer">
        <Show when={props.status && !props.status.db_connected}>
          <div class="tray-badge warn" title="No database — showing demo data">
            Demo mode
          </div>
        </Show>
        <button class="tray-item" title="Keyboard legend (?)" onClick={props.onShowLegend}>
          <kbd class="keycap">?</kbd>
          <span class="tray-label">Shortcuts</span>
        </button>
      </div>
    </nav>
  );
}
