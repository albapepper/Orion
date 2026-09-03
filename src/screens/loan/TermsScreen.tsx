import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanDetail, LoanTerms } from '../../types';
import { TaskPanel } from '../../components/TaskPanel';
import { LoanHeader } from '../../components/LoanHeader';
import { PizzaTracker } from '../../components/PizzaTracker';
import { LoanColumns } from '../../components/LoanColumns';
import { UnderwriterPanel } from '../../components/UnderwriterPanel';

interface Props {
  loanId: string;
  demo: boolean;
  onNavigate: (screen: string) => void;
}

const LOAN_PURPOSES = ['purchase', 'refinance', 'cash_out', 'construction', 'other'];
const MORTGAGE_TYPES = ['conventional', 'fha', 'va', 'usda', 'other'];
const AMORTIZATION_TYPES = ['fixed', 'arm', 'interest_only', 'other'];
const LIEN_POSITIONS = ['first', 'second'];

const label = (s: string) =>
  s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());

const money = (n: number) => `$${Math.round(n).toLocaleString()}`;

export function TermsScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId, screen: 'terms' }),
  );
  const [edits, setEdits] = createSignal<Partial<LoanTerms>>({});
  const [status, setStatus] = createSignal<'saving' | 'saved' | 'error' | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const field = <K extends keyof LoanTerms>(f: K): LoanTerms[K] => {
    const e = edits();
    return (f in e ? e[f] : detail()?.terms?.[f]) as LoanTerms[K];
  };

  const setField = <K extends keyof LoanTerms>(f: K, v: LoanTerms[K]) => {
    setEdits((e) => ({ ...e, [f]: v }));
  };

  // Ratios are computed on read — never stored (see docs/1003-fields.md)
  const ltv = () => {
    const amount = field('loan_amount');
    const value = field('property_value');
    if (!amount || !value || value <= 0) return null;
    return (amount / value) * 100;
  };

  const monthlyPI = () => {
    const amount = field('loan_amount');
    const rate = field('interest_rate');
    const months = field('term_months');
    if (!amount || amount <= 0 || !months || months <= 0) return null;
    const r = (rate ?? 0) / 100 / 12;
    if (r <= 0) return amount / months;
    return (amount * r * Math.pow(1 + r, months)) / (Math.pow(1 + r, months) - 1);
  };

  const hasEdits = () => Object.keys(edits()).length > 0;

  // One record, whole-form commit on blur/Enter — same rhythm as income
  const commit = async () => {
    if (!hasEdits() || props.demo) return;
    setStatus('saving');
    setError(null);
    try {
      await invoke('save_loan_terms', {
        loanId: props.loanId,
        terms: {
          loan_purpose: field('loan_purpose'),
          mortgage_type: field('mortgage_type'),
          amortization_type: field('amortization_type'),
          loan_amount: field('loan_amount'),
          interest_rate: field('interest_rate'),
          term_months: field('term_months'),
          lien_priority: field('lien_priority'),
          property_value: field('property_value'),
          application_date: field('application_date'),
        },
      });
      setEdits({});
      refetch();
      setStatus('saved');
      setTimeout(() => setStatus((s) => (s === 'saved' ? null : s)), 1800);
    } catch (e) {
      setStatus('error');
      setError(String(e));
    }
  };

  const numInput = (f: 'loan_amount' | 'property_value') => (
    <input
      type="number"
      min="0"
      value={field(f) ?? ''}
      onInput={(e) =>
        setField(
          f,
          Number.isNaN(e.currentTarget.valueAsNumber) ? null : e.currentTarget.valueAsNumber,
        )
      }
      onBlur={commit}
      onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
    />
  );

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="terms" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        lock={detail()?.lock ?? null}
        loanId={props.loanId}
        demo={props.demo}
        work={
          <Show when={detail()} fallback={<div class="screen-loading">Loading…</div>}>
        <div class="screen-title-row">
          <h2 class="screen-title">Loan structure</h2>
          <span class={`save-status ${status() ?? ''}`}>
            {status() === 'saving' ? 'Saving…' : status() === 'saved' ? 'Saved ✓' : null}
          </span>
        </div>
        <Show when={props.demo}>
          <div class="demo-note">Demo mode — edits stay on this screen and are not persisted.</div>
        </Show>

        <div class="form-grid">
          <div class="field">
            <label>Purpose</label>
            <select
              value={field('loan_purpose')}
              onChange={(e) => {
                setField('loan_purpose', e.currentTarget.value);
                commit();
              }}
              onBlur={commit}
            >
              <For each={LOAN_PURPOSES}>{(p) => <option value={p}>{label(p)}</option>}</For>
            </select>
          </div>
          <div class="field">
            <label>Loan type</label>
            <select
              value={field('mortgage_type')}
              onChange={(e) => {
                setField('mortgage_type', e.currentTarget.value);
                commit();
              }}
              onBlur={commit}
            >
              <For each={MORTGAGE_TYPES}>{(t) => <option value={t}>{label(t)}</option>}</For>
            </select>
          </div>
          <div class="field">
            <label>Loan amount</label>
            {numInput('loan_amount')}
          </div>
          <div class="field">
            <label>Property value</label>
            {numInput('property_value')}
          </div>
          <div class="field">
            <label>Interest rate (%)</label>
            <input
              type="number"
              step="0.001"
              min="0"
              value={field('interest_rate') ?? ''}
              onInput={(e) =>
                setField(
                  'interest_rate',
                  Number.isNaN(e.currentTarget.valueAsNumber)
                    ? null
                    : e.currentTarget.valueAsNumber,
                )
              }
              onBlur={commit}
              onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
            />
          </div>
          <div class="field">
            <label>Term (months)</label>
            <input
              type="number"
              min="0"
              value={field('term_months') ?? ''}
              onInput={(e) =>
                setField(
                  'term_months',
                  Number.isNaN(e.currentTarget.valueAsNumber)
                    ? null
                    : e.currentTarget.valueAsNumber,
                )
              }
              onBlur={commit}
              onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
            />
          </div>
          <div class="field">
            <label>Amortization</label>
            <select
              value={field('amortization_type')}
              onChange={(e) => {
                setField('amortization_type', e.currentTarget.value);
                commit();
              }}
              onBlur={commit}
            >
              <For each={AMORTIZATION_TYPES}>{(t) => <option value={t}>{label(t)}</option>}</For>
            </select>
          </div>
          <div class="field">
            <label>Lien position</label>
            <select
              value={field('lien_priority')}
              onChange={(e) => {
                setField('lien_priority', e.currentTarget.value);
                commit();
              }}
              onBlur={commit}
            >
              <For each={LIEN_POSITIONS}>{(t) => <option value={t}>{label(t)}</option>}</For>
            </select>
          </div>
          <div class="field">
            <label>Application date</label>
            <input
              type="date"
              value={field('application_date') ?? ''}
              onInput={(e) => setField('application_date', e.currentTarget.value || null)}
              onBlur={commit}
            />
          </div>
        </div>

        <div class="terms-math">
          <div class="math-stat">
            <span class="math-value mono">
              {ltv() !== null ? `${ltv()!.toFixed(1)}%` : '—'}
            </span>
            <span class="math-label">Loan-to-value</span>
          </div>
          <div class="math-stat">
            <span class="math-value mono">
              {monthlyPI() !== null ? money(monthlyPI()!) : '—'}
            </span>
            <span class="math-label">Est. monthly P&amp;I</span>
          </div>
          <div class="math-stat">
            <span class="math-value mono">
              {field('loan_amount') !== null ? money(field('loan_amount')!) : '—'}
            </span>
            <span class="math-label">Loan amount</span>
          </div>
          <div class="math-stat">
            <span class="math-value mono">
              {field('property_value') !== null ? money(field('property_value')!) : '—'}
            </span>
            <span class="math-label">Property value</span>
          </div>
        </div>

        <Show when={status() === 'error'}>
          <div class="form-error">{error()}</div>
        </Show>
      </Show>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="terms"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'terms')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="terms" demo={props.demo} />
    </div>
  );
}
