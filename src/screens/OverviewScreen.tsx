import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanDetail, MismoExport } from '../types';
import { LoanHeader } from '../components/LoanHeader';
import { PizzaTracker } from '../components/PizzaTracker';
import { LoanColumns } from '../components/LoanColumns';
import { UnderwriterPanel } from '../components/UnderwriterPanel';

interface Props {
  loanId: string;
  demo?: boolean;
  onNavigate: (screen: string) => void;
}

const money = (n: number) => `$${Math.round(n).toLocaleString()}`;

export function OverviewScreen(props: Props) {
  const [detail] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId, screen: 'overview' }),
  );

  const terms = () => detail()?.terms;
  const income = () => detail()?.income ?? [];
  const totalIncome = () => income().reduce((s, r) => s + r.monthly_amount, 0);
  const totalAssets = () =>
    (detail()?.assets ?? []).reduce((s, r) => s + Number(r.balance), 0);
  const activeDebt = () =>
    (detail()?.liabilities ?? [])
      .filter((l) => !l.payoff_at_close)
      .reduce((s, l) => s + Number(l.monthly_payment ?? 0), 0);

  const ltv = () => {
    const t = terms();
    if (!t?.loan_amount || !t?.property_value) return null;
    return (t.loan_amount / t.property_value) * 100;
  };

  const monthlyPI = () => {
    const t = terms();
    if (!t?.loan_amount || !t?.term_months) return null;
    const r = (t.interest_rate ?? 0) / 100 / 12;
    if (r <= 0) return t.loan_amount / t.term_months;
    return (
      (t.loan_amount * r * Math.pow(1 + r, t.term_months)) /
      (Math.pow(1 + r, t.term_months) - 1)
    );
  };

  const primary = () =>
    detail()?.borrowers.find((b) => b.is_primary) ?? detail()?.borrowers[0];

  // ── MISMO 3.4 export (P4-04) ───────────────────────────────────────
  const [exportResult, setExportResult] = createSignal<MismoExport | null>(null);
  const [exporting, setExporting] = createSignal(false);
  const [exportError, setExportError] = createSignal<string | null>(null);
  const [savedPath, setSavedPath] = createSignal<string | null>(null);
  const [copied, setCopied] = createSignal(false);

  const runExport = async () => {
    setExporting(true);
    setExportError(null);
    setSavedPath(null);
    try {
      const res = await invoke<MismoExport>('export_loan_mismo', { loanId: props.loanId });
      setExportResult(res);
    } catch (e) {
      setExportError(String(e));
    } finally {
      setExporting(false);
    }
  };

  const fileName = () => `${detail()?.summary.loan_number ?? 'loan'}-1003.xml`;

  const saveExport = async () => {
    try {
      const p = await invoke<string>('save_mismo_file', {
        xml: exportResult()!.xml,
        fileName: fileName(),
      });
      setSavedPath(p);
    } catch (e) {
      setExportError(String(e));
    }
  };

  const copyExport = async () => {
    try {
      await navigator.clipboard.writeText(exportResult()!.xml);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (e) {
      setExportError(String(e));
    }
  };

  const stats = () => {
    const t = terms();
    return [
      { label: 'Loan amount', value: t?.loan_amount != null ? money(t.loan_amount) : '—' },
      {
        label: 'Note rate',
        value: t?.interest_rate != null ? `${t.interest_rate.toFixed(3)}%` : '—',
      },
      { label: 'LTV', value: ltv() !== null ? `${ltv()!.toFixed(1)}%` : '—' },
      { label: 'Est. monthly P&I', value: monthlyPI() !== null ? money(monthlyPI()!) : '—' },
      {
        label: 'Qualifying income',
        value: totalIncome() > 0 ? `${money(totalIncome())}/mo` : '—',
      },
      { label: 'Debt payments', value: activeDebt() > 0 ? `${money(activeDebt())}/mo` : '—' },
      { label: 'Total assets', value: totalAssets() > 0 ? money(totalAssets()) : '—' },
    ];
  };

  const sections = () => {
    const p = detail()?.property;
    const emp = detail()?.employments ?? [];
    const decl = detail()?.declarations ?? [];
    return [
      {
        label: 'Borrower',
        value: () => {
          const b = primary();
          return b ? `${b.first_name} ${b.last_name}` : '—';
        },
        screen: 'borrower',
      },
      {
        label: 'Property',
        value: () =>
          p?.address_line ? `${p.address_line}, ${p.city ?? ''}` : 'Not entered',
        screen: 'property',
      },
      {
        label: 'Employment',
        value: () =>
          emp.length ? `${emp.length} entr${emp.length === 1 ? 'y' : 'ies'}` : 'Not entered',
        screen: 'employment',
      },
      {
        label: 'Declarations',
        value: () =>
          decl.length
            ? `${decl.filter((x) => x.answer !== null).length} answered`
            : 'Not started',
        screen: 'declarations',
      },
      {
        label: 'Income',
        value: () => (income().length ? `${income().length} sources` : 'Not entered'),
        screen: 'income',
      },
      {
        label: 'Assets',
        value: () =>
          (detail()?.assets ?? []).length
            ? `${detail()!.assets.length} accounts`
            : 'Not entered',
        screen: 'assets',
      },
      {
        label: 'Liabilities',
        value: () =>
          (detail()?.liabilities ?? []).length
            ? `${detail()!.liabilities.length} debts`
            : 'Not entered',
        screen: 'liabilities',
      },
      {
        label: 'REO',
        value: () =>
          (detail()?.reo ?? []).length ? `${detail()!.reo.length} properties` : 'None',
        screen: 'reo',
      },
    ];
  };

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="overview" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        lock={detail()?.lock ?? null}
        work={
          <Show when={detail()} fallback={<div class="screen-loading">Loading…</div>}>
        <div class="overview-meta">
          <span class={`status-pill ${detail()!.summary.status}`}>
            {detail()!.summary.status}
          </span>
          <span>Assigned to {detail()!.summary.assigned_to_name ?? 'nobody yet'}</span>
          <Show when={terms()?.application_date}>
            <span class="mono">applied {terms()!.application_date}</span>
          </Show>
        </div>

        <div class="terms-math" style="margin-top: 20px">
          <For each={stats()}>
            {(s) => (
              <div class="math-stat">
                <span class="math-value mono">{s.value}</span>
                <span class="math-label">{s.label}</span>
              </div>
            )}
          </For>
        </div>

        <h2 class="section-title">The file at a glance</h2>
        <div class="overview-grid">
          <For each={sections()}>
            {(s) => (
              <button class="overview-cell" onClick={() => props.onNavigate(s.screen)}>
                <span class="overview-label">{s.label}</span>
                <span class="overview-value">{s.value()}</span>
              </button>
            )}
          </For>
        </div>

        <h2 class="section-title">Export 1003</h2>
        <div class="export-row">
          <button class="ghost" disabled={exporting()} onClick={runExport}>
            {exporting() ? 'Serializing…' : 'Build MISMO 3.4 XML'}
          </button>
        </div>
        <Show when={exportError()}>
          <div class="task-error">{exportError()}</div>
        </Show>
        <Show when={exportResult()}>
          <Show when={exportResult()!.missing.length}>
            <div class="mismo-missing">
              <span class="mismo-missing-title">Not submission-ready:</span>
              <For each={exportResult()!.missing}>
                {(m) => <span class="mismo-missing-item">{m}</span>}
              </For>
            </div>
          </Show>
          <div class="export-row">
            <button class="ghost small" onClick={saveExport}>
              Save to Downloads
            </button>
            <button class="ghost small" onClick={copyExport}>
              {copied() ? 'Copied' : 'Copy XML'}
            </button>
            <Show when={savedPath()}>
              <span class="mono mismo-path">{savedPath()}</span>
            </Show>
          </div>
          <details class="mismo-preview-wrap">
            <summary>Preview XML</summary>
            <pre class="mismo-preview mono">{exportResult()!.xml}</pre>
          </details>
        </Show>

        <p class="screen-hint">
          Press <kbd class="keycap inline">?</kbd> for keyboard navigation — every
          screen is reachable without the mouse.
        </p>
          </Show>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="overview"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'overview')}
            demo={props.demo}
          />
        }
      />
    </div>
  );
}
