import { For, Show } from 'solid-js';
import type { AppStatus, LoanSummary } from '../types';

export interface NavItem {
  id: string;
  label: string;
  key: string; // ctrl+key shortcut, shown as a keycap
}

// Loan screens follow the cascade: property → terms → credit → income → assets
export const NAV_ITEMS: NavItem[] = [
  { id: 'dashboard', label: 'Dashboard', key: 'd' },
  { id: 'property', label: 'Property', key: 'p' },
  { id: 'terms', label: 'Terms', key: 't' },
  { id: 'credit', label: 'Credit', key: 'c' },
  { id: 'income', label: 'Income', key: 'i' },
  { id: 'assets', label: 'Assets', key: 'a' },
];

interface Props {
  activeNav: string;
  onNavigate: (id: string) => void;
  recentLoans: LoanSummary[];
  activeLoanId: string | null;
  onOpenLoan: (loan: LoanSummary) => void;
  onShowLegend: () => void;
  status?: AppStatus;
}

export function NavTray(props: Props) {
  return (
    <nav class="nav-tray">
      <div class="tray-brand">
        <span class="brand-mark">◆</span>
        <span class="brand-name">Orion</span>
      </div>

      <div class="tray-section">
        <For each={NAV_ITEMS}>
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
