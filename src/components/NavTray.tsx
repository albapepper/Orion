import { For, Show } from 'solid-js';
import type { AppStatus, LoanSummary, UserAccount } from '../types';
import { ROLE_LABELS } from '../lib/session';

export interface NavItem {
  id: string;
  label: string;
  key: string; // ctrl+key shortcut, shown as a keycap
  icon: 'home' | 'loans' | 'tasks' | 'rates' | 'team' | 'settings';
}

// App navigation only. Everything about a specific loan — its flow, its
// stages — lives in the tracker at the top of the loan page.
const APP_ITEMS: NavItem[] = [
  { id: 'dashboard', label: 'Home', key: 'd', icon: 'home' },
  { id: 'loans', label: 'Loans', key: 'n', icon: 'loans' },
  { id: 'tasks', label: 'Tasks', key: 'k', icon: 'tasks' },
  { id: 'rates', label: 'Rate sheets', key: 'y', icon: 'rates' },
  { id: 'users', label: 'Team', key: 'm', icon: 'team' },
  { id: 'settings', label: 'Settings', key: 's', icon: 'settings' },
];

function NavIcon(props: { name: NavItem['icon'] }) {
  return (
    <svg class="nav-icon" viewBox="0 0 24 24" fill="none" aria-hidden="true">
      <Show when={props.name === 'home'}>
        <path d="M4 10.5 12 4l8 6.5V20h-5v-6H9v6H4v-9.5Z" />
      </Show>
      <Show when={props.name === 'loans'}>
        <path d="M5 3h11l3 3v15H5V3Z" /><path d="M8 9h8M8 13h8M8 17h5" />
      </Show>
      <Show when={props.name === 'tasks'}>
        <path d="M8 5h11M8 12h11M8 19h11M3.5 5l1 1 2-2M3.5 12l1 1 2-2M3.5 19l1 1 2-2" />
      </Show>
      <Show when={props.name === 'rates'}>
        <path d="M4 19V9M10 19V5M16 19v-7M22 19H2" />
      </Show>
      <Show when={props.name === 'team'}>
        <path d="M8.5 11a3.5 3.5 0 1 0 0-7 3.5 3.5 0 0 0 0 7ZM2.5 20c.4-4 2.3-6 6-6s5.6 2 6 6M16 10a3 3 0 1 0 0-6M15.5 14c3.7 0 5.7 2 6 6" />
      </Show>
      <Show when={props.name === 'settings'}>
        <circle cx="12" cy="12" r="3" /><path d="M19 12a7 7 0 0 0-.1-1l2-1.5-2-3.4-2.4 1a8 8 0 0 0-1.7-1L14.5 3h-5L9 6a8 8 0 0 0-1.7 1L5 6.1 3 9.5 5.1 11a7 7 0 0 0 0 2L3 14.5l2 3.4 2.4-1a8 8 0 0 0 1.7 1l.4 3.1h5l.5-3.1a8 8 0 0 0 1.7-1l2.3 1 2-3.4-2.1-1.5a7 7 0 0 0 .1-1Z" />
      </Show>
    </svg>
  );
}

interface Props {
  activeNav: string;
  onNavigate: (id: string) => void;
  recentLoans: LoanSummary[];
  activeLoanId: string | null;
  onOpenLoan: (loan: LoanSummary) => void;
  onShowLegend: () => void;
  status?: AppStatus;
  user?: UserAccount | null;
  theme: 'light' | 'dark';
  onToggleTheme: () => void;
}

export function NavTray(props: Props) {
  return (
    <nav class="nav-tray">
      <div class="tray-brand">
        <span class="brand-mark" aria-hidden="true">
          <svg viewBox="0 0 32 32" fill="none">
            <circle cx="16" cy="16" r="11" />
            <path d="M16 1v7M16 24v7M1 16h7M24 16h7" />
            <path class="brand-star" d="m16 10 1.7 4.3L22 16l-4.3 1.7L16 22l-1.7-4.3L10 16l4.3-1.7L16 10Z" />
          </svg>
        </span>
        <span class="brand-lockup">
          <span class="brand-name">Orion</span>
          <span class="brand-product">Mortgage workspace</span>
        </span>
        <Show when={props.status?.company_name}>
          <span class="brand-company">{props.status!.company_name}</span>
        </Show>
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
              <NavIcon name={item.icon} />
              <span class="tray-label">{item.label}</span>
              <kbd class="nav-shortcut">⌃{item.key}</kbd>
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
        <div class="tray-footer-actions">
          <button class="tray-utility" title="Keyboard shortcuts" onClick={props.onShowLegend}>
            <span class="utility-icon">⌘</span>
            <span>Shortcuts</span>
          </button>
          <button class="theme-toggle" title={`Use ${props.theme === 'dark' ? 'light' : 'dark'} mode`} onClick={props.onToggleTheme}>
            <span class="theme-track"><span class="theme-thumb" /></span>
            <span>{props.theme === 'dark' ? 'Dark' : 'Light'}</span>
          </button>
        </div>
      </div>
    </nav>
  );
}
