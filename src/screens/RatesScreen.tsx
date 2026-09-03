import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanProduct, RateOption, RateSheet } from '../types';

const PRODUCTS: LoanProduct[] = ['30yr_fixed', '20yr_fixed', '15yr_fixed', '7_1_arm', '5_1_arm'];

const label = (s: string) =>
  s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());

const dateLabel = (iso: string | null) =>
  iso
    ? new Date(iso).toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' })
    : '—';

// Rate sheets admin: what each lender is paying this week. The pricing
// screen (and its AI) build options from the active sheet, so keeping
// this current is what makes quotes credible.
export function RatesScreen(props: { demo?: boolean }) {
  const [sheets, { refetch }] = createResource(() =>
    invoke<RateSheet[]>('list_rate_sheets'),
  );
  const [sheetId, setSheetId] = createSignal<string | null>(null);

  const selectedId = () =>
    sheetId() ?? sheets()?.find((s) => s.is_active)?.id ?? sheets()?.[0]?.id ?? null;
  const [options, { refetch: refetchOptions }] = createResource(
    selectedId,
    (id) => invoke<RateOption[]>('list_rate_options', { rateSheetId: id }),
  );

  const [newLender, setNewLender] = createSignal('');
  const [newEffective, setNewEffective] = createSignal('');
  const [newExpires, setNewExpires] = createSignal('');

  const [optProduct, setOptProduct] = createSignal<LoanProduct>('30yr_fixed');
  const [optRate, setOptRate] = createSignal('');
  const [optPoints, setOptPoints] = createSignal('');
  const [optComp, setOptComp] = createSignal('');

  const [error, setError] = createSignal<string | null>(null);
  const [localOptions, setLocalOptions] = createSignal<RateOption[] | null>(null);

  const rows = () => localOptions() ?? (options() ?? []);

  const createSheet = async () => {
    const lender = newLender().trim();
    if (!lender) return;
    try {
      await invoke('create_rate_sheet', {
        lender,
        effectiveDate: newEffective() || null,
        expiresDate: newExpires() || null,
      });
      setNewLender('');
      setNewEffective('');
      setNewExpires('');
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const activate = async (sheet: RateSheet) => {
    if (props.demo) return;
    try {
      await invoke('set_rate_sheet_active', { rateSheetId: sheet.id, active: true });
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const addOption = async () => {
    const id = selectedId();
    const rate = Number.parseFloat(optRate());
    if (!id || Number.isNaN(rate)) return;
    const points = Number.parseFloat(optPoints()) || 0;
    if (props.demo) {
      setLocalOptions([
        ...(localOptions() ?? rows()),
        {
          id: crypto.randomUUID(),
          rate_sheet_id: id,
          company_id: '',
          product: optProduct(),
          rate,
          points,
          comp_plan: optComp() || null,
          min_loan_amount: null,
          max_loan_amount: null,
          created_at: new Date().toISOString(),
        },
      ]);
      setOptRate('');
      setOptPoints('');
      return;
    }
    try {
      await invoke('add_rate_option', {
        rateSheetId: id,
        product: optProduct(),
        rate,
        points,
        compPlan: optComp() || null,
      });
      setOptRate('');
      setOptPoints('');
      setError(null);
      refetchOptions();
    } catch (e) {
      setError(String(e));
    }
  };

  const removeOption = async (option: RateOption) => {
    if (props.demo) {
      setLocalOptions((localOptions() ?? rows()).filter((o) => o.id !== option.id));
      return;
    }
    try {
      await invoke('remove_rate_option', { rateOptionId: option.id });
      setError(null);
      refetchOptions();
    } catch (e) {
      setError(String(e));
    }
  };

  const selected = () => sheets()?.find((s) => s.id === selectedId());

  return (
    <div class="screen">
      <div class="screen-title-row">
        <h2 class="screen-title">Rate sheets</h2>
        <Show when={selected()}>
          <span class="save-status">
            {selected()!.lender} · {dateLabel(selected()!.effective_date)} –{' '}
            {dateLabel(selected()!.expires_date)}
          </span>
        </Show>
      </div>
      <Show when={props.demo}>
        <div class="demo-note">Demo mode — edits stay on this screen and are not persisted.</div>
      </Show>

      <div class="rates-layout">
        <aside class="rates-sheets">
          <div class="tray-heading">Lenders</div>
          <For each={sheets() ?? []}>
            {(sheet) => (
              <button
                class="tray-item"
                classList={{ active: sheet.id === selectedId() }}
                onClick={() => {
                  setSheetId(sheet.id);
                  setLocalOptions(null);
                }}
              >
                <span class="tray-label">
                  <span class="loan-number">{sheet.lender}</span>
                  <span class="loan-borrower">
                    {sheet.is_active ? 'Active' : 'Inactive'} · {dateLabel(sheet.effective_date)}
                  </span>
                </span>
              </button>
            )}
          </For>

          <div class="rates-new">
            <input
              type="text"
              placeholder="New lender name…"
              value={newLender()}
              onInput={(e) => setNewLender(e.currentTarget.value)}
              onKeyDown={(e) => e.key === 'Enter' && createSheet()}
            />
            <div class="rates-dates">
              <input
                type="date"
                title="Effective date"
                value={newEffective()}
                onInput={(e) => setNewEffective(e.currentTarget.value)}
              />
              <input
                type="date"
                title="Expires date"
                value={newExpires()}
                onInput={(e) => setNewExpires(e.currentTarget.value)}
              />
            </div>
            <button class="ghost small" onClick={createSheet}>
              + Add sheet
            </button>
          </div>
        </aside>

        <div class="rates-options">
          <Show when={selected() && !selected()!.is_active} fallback={null}>
            <button class="ghost small rates-activate" onClick={() => selected() && activate(selected()!)}>
              Make this the active sheet
            </button>
          </Show>

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
              <For
                each={rows()}
                fallback={
                  <tr>
                    <td colspan={5}>No options on this sheet yet.</td>
                  </tr>
                }
              >
                {(option) => (
                  <tr>
                    <td>{label(option.product)}</td>
                    <td class="mono">{option.rate.toFixed(3)}%</td>
                    <td class="mono">
                      {option.points > 0 ? `+${option.points.toFixed(3)}` : option.points.toFixed(3)}
                    </td>
                    <td>{option.comp_plan ?? '—'}</td>
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

          <div class="rates-add">
            <select
              value={optProduct()}
              onChange={(e) => setOptProduct(e.currentTarget.value as LoanProduct)}
              title="Product"
            >
              <For each={PRODUCTS}>{(p) => <option value={p}>{label(p)}</option>}</For>
            </select>
            <input
              type="number"
              step="0.001"
              min="0"
              placeholder="Rate %"
              title="Note rate"
              value={optRate()}
              onInput={(e) => setOptRate(e.currentTarget.value)}
              onKeyDown={(e) => e.key === 'Enter' && addOption()}
            />
            <input
              type="number"
              step="0.125"
              placeholder="Points"
              title="Points — positive = borrower pays, negative = lender credit"
              value={optPoints()}
              onInput={(e) => setOptPoints(e.currentTarget.value)}
              onKeyDown={(e) => e.key === 'Enter' && addOption()}
            />
            <input
              type="text"
              placeholder="Comp plan…"
              value={optComp()}
              onInput={(e) => setOptComp(e.currentTarget.value)}
              onKeyDown={(e) => e.key === 'Enter' && addOption()}
            />
            <button onClick={addOption}>Add</button>
          </div>
        </div>
      </div>
      <Show when={error()}>
        <div class="task-error">{error()}</div>
      </Show>
    </div>
  );
}
