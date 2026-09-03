import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanDetail, LoanPricingOption, RateOption, RateSheet } from '../../types';
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

const label = (s: string) =>
  s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());

const money = (v: number | null | undefined) =>
  v == null ? '—' : `$${v.toLocaleString(undefined, { maximumFractionDigits: 0 })}`;

const monthlyPI = (amount: number, rate: number, months: number | null) => {
  if (!months || months <= 0) return null;
  const r = rate / 100 / 12;
  if (r <= 0) return amount / months;
  return (amount * r * Math.pow(1 + r, months)) / (Math.pow(1 + r, months) - 1);
};

// Pricing: build option rows for this loan from the active rate sheet.
// P&I and the dollar cost of points are computed against the loan's own
// terms; the LO marks one option as the pick, which is what flows toward
// locking later.
export function PricingScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) =>
      invoke<LoanDetail>('get_loan_detail', { loanId, screen: 'pricing' }),
  );
  const [sheets] = createResource(() => invoke<RateSheet[]>('list_rate_sheets'));
  const activeSheet = () => sheets()?.find((s) => s.is_active);
  const [sheetOptions] = createResource(
    () => activeSheet()?.id,
    (id) => invoke<RateOption[]>('list_rate_options', { rateSheetId: id }),
  );

  const [localOptions, setLocalOptions] = createSignal<LoanPricingOption[] | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const options = () => localOptions() ?? (detail()?.pricing ?? []);
  const selected = () => options().find((o) => o.selected);

  const terms = () => detail()?.terms;
  const loanAmount = () => terms()?.loan_amount ?? null;

  const addFromSheet = async (opt: RateOption) => {
    if (props.demo) {
      const t = terms();
      const pi =
        t?.loan_amount != null
          ? monthlyPI(t.loan_amount, opt.rate, t.term_months ?? null)
          : null;
      setLocalOptions([
        ...(localOptions() ?? options()),
        {
          id: crypto.randomUUID(),
          loan_id: props.loanId,
          company_id: '',
          product: opt.product,
          rate: opt.rate,
          points: opt.points,
          monthly_pi: pi,
          points_cost:
            t?.loan_amount != null ? (opt.points / 100) * t.loan_amount : null,
          comp_plan: opt.comp_plan,
          selected: false,
          created_by: null,
          created_at: new Date().toISOString(),
        },
      ]);
      return;
    }
    try {
      await invoke('add_loan_pricing', {
        loanId: props.loanId,
        product: opt.product,
        rate: opt.rate,
        points: opt.points,
        compPlan: opt.comp_plan,
      });
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  // Sheet rows and loan options live in different tables with different
  // ids — match on the price itself to gray out already-added rows.
  const onSheet = (opt: RateOption) =>
    !options().some(
      (o) =>
        o.product === opt.product && o.rate === opt.rate && o.points === opt.points,
    );

  const selectOption = async (option: LoanPricingOption) => {
    if (props.demo) {
      setLocalOptions(
        (localOptions() ?? options()).map((o) => ({
          ...o,
          selected: o.id === option.id,
        })),
      );
      return;
    }
    try {
      await invoke('select_loan_pricing', {
        loanId: props.loanId,
        loanPricingId: option.id,
      });
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const removeOption = async (option: LoanPricingOption) => {
    if (props.demo) {
      setLocalOptions((localOptions() ?? options()).filter((o) => o.id !== option.id));
      return;
    }
    try {
      await invoke('remove_loan_pricing', { loanPricingId: option.id });
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="pricing" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        lock={detail()?.lock ?? null}
        work={
          <>
            <div class="screen-title-row">
              <h2 class="screen-title">Pricing</h2>
              <span class="save-status">{options().length} options built</span>
            </div>
            <Show when={props.demo}>
              <div class="demo-note">Demo mode — edits stay on this screen and are not persisted.</div>
            </Show>

            <Show when={loanAmount()} fallback={<div class="demo-note">Fill in Loan Terms first — the math needs a loan amount.</div>}>
              <div class="pricing-summary">
                <div class="math-stat">
                  <span class="math-value mono">{money(loanAmount())}</span>
                  <span class="math-label">Loan amount</span>
                </div>
                <div class="math-stat">
                  <span class="math-value mono">
                    {selected() ? `${selected()!.rate.toFixed(3)}%` : '—'}
                  </span>
                  <span class="math-label">Selected rate</span>
                </div>
                <div class="math-stat">
                  <span class="math-value mono">
                    {selected()?.monthly_pi ? money(selected()!.monthly_pi) : '—'}
                  </span>
                  <span class="math-label">Selected P&I</span>
                </div>
                <div class="math-stat">
                  <span class="math-value mono">
                    {selected()?.points_cost ? money(selected()!.points_cost) : '—'}
                  </span>
                  <span class="math-label">Points cost</span>
                </div>
              </div>
            </Show>

            <h2 class="section-title">From the active sheet</h2>
            <Show
              when={sheetOptions()}
              fallback={<div class="demo-note">No active rate sheet — set one up in Rate sheets (⌃y).</div>}
            >
              <table class="data-table">
                <thead>
                  <tr>
                    <th>Product</th>
                    <th>Rate</th>
                    <th>Points</th>
                    <th>Comp plan</th>
                    <th />
                  </tr>
                </thead>
                <tbody>
                  <For each={sheetOptions() ?? []}>
                    {(opt) => (
                      <tr>
                        <td>{label(opt.product)}</td>
                        <td class="mono">{opt.rate.toFixed(3)}%</td>
                        <td class="mono">
                          {opt.points > 0 ? `+${opt.points.toFixed(3)}` : opt.points.toFixed(3)}
                        </td>
                        <td>{opt.comp_plan ?? '—'}</td>
                        <td>
                          <Show
                            when={onSheet(opt)}
                            fallback={<span class="loans-none">added</span>}
                          >
                            <button class="ghost small" onClick={() => addFromSheet(opt)}>
                              + Add
                            </button>
                          </Show>
                        </td>
                      </tr>
                    )}
                  </For>
                </tbody>
              </table>
            </Show>

            <h2 class="section-title">Options for this loan</h2>
            <table class="data-table">
              <thead>
                <tr>
                  <th>Pick</th>
                  <th>Product</th>
                  <th>Rate</th>
                  <th>Points</th>
                  <th>Monthly P&I</th>
                  <th>Points cost</th>
                  <th />
                </tr>
              </thead>
              <tbody>
                <For
                  each={options()}
                  fallback={
                    <tr>
                      <td colspan={7}>Nothing built yet — add rows from the sheet above.</td>
                    </tr>
                  }
                >
                  {(option) => (
                    <tr classList={{ 'pricing-selected': option.selected }}>
                      <td>
                        <input
                          type="radio"
                          name="pricing-pick"
                          checked={option.selected}
                          onChange={() => selectOption(option)}
                          title="Make this the selected option"
                        />
                      </td>
                      <td>{label(option.product)}</td>
                      <td class="mono">{option.rate.toFixed(3)}%</td>
                      <td class="mono">
                        {option.points > 0 ? `+${option.points.toFixed(3)}` : option.points.toFixed(3)}
                      </td>
                      <td class="mono">{money(option.monthly_pi)}</td>
                      <td class="mono">{money(option.points_cost)}</td>
                      <td>
                        <button
                          class="ghost small"
                          title="Remove option"
                          onClick={() => removeOption(option)}
                        >
                          ✕
                        </button>
                      </td>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
            <Show when={error()}>
              <div class="task-error">{error()}</div>
            </Show>
          </>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="pricing"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'pricing')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="pricing" demo={props.demo} />
    </div>
  );
}
