import { createMemo, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanSummary } from '../types';
import { LOAN_STATUSES } from '../types';

interface Props {
  loans: LoanSummary[];
  demo: boolean;
  onOpenLoan: (loan: LoanSummary) => void;
  onCreated: (loan: LoanSummary) => void;
}

type SortKey = 'updated' | 'number' | 'borrower';

const dateLabel = (iso: string) =>
  new Date(iso).toLocaleDateString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  });

// Every loan in the shop, filterable by stage and owner. The dashboard
// answers "what's next"; this answers "show me the whole book" — the
// pipeline's table of contents.
export function LoansScreen(props: Props) {
  const [statusFilter, setStatusFilter] = createSignal('all');
  const [ownerFilter, setOwnerFilter] = createSignal('all');
  const [query, setQuery] = createSignal('');
  const [sortKey, setSortKey] = createSignal<SortKey>('updated');

  // ── New loan (P5-04) ───────────────────────────────────────────────
  const [creating, setCreating] = createSignal(false);
  const [firstName, setFirstName] = createSignal('');
  const [lastName, setLastName] = createSignal('');
  const [createError, setCreateError] = createSignal<string | null>(null);
  const [busy, setBusy] = createSignal(false);

  const createLoan = async () => {
    setBusy(true);
    setCreateError(null);
    try {
      const loan = await invoke<LoanSummary>('create_loan', {
        firstName: firstName(),
        lastName: lastName(),
      });
      setCreating(false);
      setFirstName('');
      setLastName('');
      props.onCreated(loan);
    } catch (e) {
      setCreateError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const owners = () => [
    ...new Set(
      props.loans
        .map((l) => l.assigned_to_name)
        .filter((n): n is string => n !== null),
    ),
  ];

  const filtered = createMemo(() => {
    const q = query().trim().toLowerCase();
    const rows = props.loans.filter((l) => {
      if (statusFilter() !== 'all' && l.status !== statusFilter()) return false;
      if (ownerFilter() === 'unassigned' && l.assigned_to_name !== null)
        return false;
      if (
        ownerFilter() !== 'all' &&
        ownerFilter() !== 'unassigned' &&
        l.assigned_to_name !== ownerFilter()
      )
        return false;
      if (q) {
        const hay = `${l.loan_number} ${l.borrower_name ?? ''}`.toLowerCase();
        if (!hay.includes(q)) return false;
      }
      return true;
    });
    return rows.sort((a, b) => {
      switch (sortKey()) {
        case 'number':
          return a.loan_number.localeCompare(b.loan_number);
        case 'borrower':
          return (a.borrower_name ?? '').localeCompare(b.borrower_name ?? '');
        default:
          return new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime();
      }
    });
  });

  const activeFilters = () =>
    statusFilter() !== 'all' || ownerFilter() !== 'all' || query().trim() !== '';

  return (
    <div class="screen">
      <div class="screen-title-row">
        <h2 class="screen-title">Loans</h2>
        <span class="save-status">{filtered().length} of {props.loans.length}</span>
        <Show when={!props.demo}>
          <button
            class="ghost small"
            style="margin-left: auto"
            onClick={() => setCreating(!creating())}
          >
            {creating() ? 'Cancel' : '+ New loan'}
          </button>
        </Show>
      </div>

      <Show when={creating()}>
        <div class="user-create">
          <input
            type="text"
            placeholder="Borrower first name"
            value={firstName()}
            onInput={(e) => setFirstName(e.currentTarget.value)}
          />
          <input
            type="text"
            placeholder="Borrower last name"
            value={lastName()}
            onInput={(e) => setLastName(e.currentTarget.value)}
          />
          <button
            class="ghost"
            disabled={busy() || !firstName().trim() || !lastName().trim()}
            onClick={createLoan}
          >
            Create loan
          </button>
        </div>
        <Show when={createError()}>
          <div class="form-error">{createError()}</div>
        </Show>
      </Show>

      <div class="loans-toolbar">
        <input
          type="text"
          class="loans-search"
          placeholder="Search loan number or borrower…"
          value={query()}
          onInput={(e) => setQuery(e.currentTarget.value)}
        />
        <select
          value={statusFilter()}
          onChange={(e) => setStatusFilter(e.currentTarget.value)}
          title="Filter by stage"
        >
          <option value="all">All stages</option>
          <For each={[...LOAN_STATUSES]}>
            {(s) => <option value={s}>{s}</option>}
          </For>
        </select>
        <select
          value={ownerFilter()}
          onChange={(e) => setOwnerFilter(e.currentTarget.value)}
          title="Filter by owner"
        >
          <option value="all">All owners</option>
          <For each={owners()}>{(name) => <option value={name}>{name}</option>}</For>
          <option value="unassigned">Unassigned</option>
        </select>
        <select
          value={sortKey()}
          onChange={(e) => setSortKey(e.currentTarget.value as SortKey)}
          title="Sort"
        >
          <option value="updated">Recently updated</option>
          <option value="number">Loan number</option>
          <option value="borrower">Borrower name</option>
        </select>
        <Show when={activeFilters()}>
          <button
            class="ghost small"
            onClick={() => {
              setStatusFilter('all');
              setOwnerFilter('all');
              setQuery('');
            }}
          >
            Clear
          </button>
        </Show>
      </div>

      <Show
        when={filtered().length > 0}
        fallback={<div class="screen-loading">No loans match these filters.</div>}
      >
        <table class="data-table loans-table">
          <thead>
            <tr>
              <th>Loan</th>
              <th>Borrower</th>
              <th>Stage</th>
              <th>Owner</th>
              <th>Flags</th>
              <th>Updated</th>
            </tr>
          </thead>
          <tbody>
            <For each={filtered()}>
              {(loan) => (
                <tr class="loan-row" onClick={() => props.onOpenLoan(loan)}>
                  <td class="mono">{loan.loan_number}</td>
                  <td>{loan.borrower_name ?? '—'}</td>
                  <td>
                    <span class={`status-pill ${loan.status}`}>{loan.status}</span>
                  </td>
                  <td>{loan.assigned_to_name ?? <em>Unassigned</em>}</td>
                  <td>
                    <Show when={loan.flagged_reviews > 0} fallback={<span class="loans-none">—</span>}>
                      <span class="queue-flag">⚠ {loan.flagged_reviews}</span>
                    </Show>
                  </td>
                  <td>{dateLabel(loan.updated_at)}</td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </Show>
    </div>
  );
}
