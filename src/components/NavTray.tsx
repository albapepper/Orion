import { For, Show } from 'solid-js';
import type { AppStatus, LoanSummary } from '../types';

export interface NavItem {
  id: string;
  label: string;
  key: string; // keyboard shortcut, shown as a keycap
  needsLoan: boolean;
}

export const NAV_ITEMS: NavItem[] = [
  { id: 'dashboard', label: 'Dashboard', key: 'd', needsLoan: false },
  { id: 'income', label: 'Income', key: 'i', needsLoan: true },
  { id: 'assets', label: 'Assets', key: 'a', needsLoan: true },
  { id: 'credit', label: 'Credit', key: 'c', needsLoan: true },
  { id: 'property', label: 'Property', key: 'p', needsLoan: true },
];

interface Props {
  collapsed: boolean;
  onToggle: () => void;
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
    <nav class="nav-tray" classList={{ collapsed: props.collapsed }}>
      <div class="tray-brand">
        <span class="brand-mark">◆</span>
        <Show when={!props.collapsed}>
          <span class="brand-name">Orion</span>
        </Show>
        <button
          class="tray-toggle"
          title={props.collapsed ? 'Expand ( [ )' : 'Collapse ( [ )'}
          onClick={props.onToggle}
        >
          {props.collapsed ? '»' : '«'}
        </button>
      </div>

      <div class="tray-section">
        <For each={NAV_ITEMS}>
          {(item) => (
            <button
              class="tray-item"
              classList={{ active: props.activeNav === item.id }}
              title={`${item.label} (${item.key})`}
              onClick={() => props.onNavigate(item.id)}
            >
              <kbd class="keycap">{item.key}</kbd>
              <Show when={!props.collapsed}>
                <span class="tray-label">{item.label}</span>
              </Show>
            </button>
          )}
        </For>
      </div>

      <Show when={!props.collapsed}>
        <div class="tray-section recent">
          <div class="tray-heading">Recent loans</div>
          <For each={props.recentLoans.slice(0, 9)}>
            {(loan, i) => (
              <button
                class="tray-item loan"
                classList={{ active: props.activeLoanId === loan.id }}
                onClick={() => props.onOpenLoan(loan)}
              >
                <kbd class="keycap">{i() + 1}</kbd>
                <span class="tray-label">
                  <span class="loan-number">{loan.loan_number}</span>
                  <span class="loan-borrower">{loan.borrower_name ?? '—'}</span>
                </span>
              </button>
            )}
          </For>
        </div>
      </Show>

      <div class="tray-footer">
        <Show when={props.status && !props.status.db_connected}>
          <div class="tray-badge warn" title="No database — showing demo data">
            <Show when={!props.collapsed} fallback="◦">
              Demo mode
            </Show>
          </div>
        </Show>
        <button class="tray-item" title="Keyboard legend (?)" onClick={props.onShowLegend}>
          <kbd class="keycap">?</kbd>
          <Show when={!props.collapsed}>
            <span class="tray-label">Shortcuts</span>
          </Show>
        </button>
      </div>
    </nav>
  );
}
