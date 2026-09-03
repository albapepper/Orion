import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanDetail, LoanIncome } from '../../types';
import { INCOME_TYPES } from '../../types';
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

type RowStatus = 'saving' | 'saved' | 'error';

const twoYearAvg = (row: LoanIncome) =>
  row.prior_year_monthly !== null
    ? (Number(row.prior_year_monthly) + Number(row.monthly_amount)) / 2
    : null;

export function IncomeScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId }),
  );
  const [edits, setEdits] = createSignal<Record<string, Partial<LoanIncome>>>({});
  const [rowStatus, setRowStatus] = createSignal<Record<string, RowStatus>>({});
  const [rowErrors, setRowErrors] = createSignal<Record<string, string>>({});

  const rowValue = <K extends keyof LoanIncome>(row: LoanIncome, field: K): LoanIncome[K] => {
    const edit = edits()[row.id];
    return (edit && field in edit ? edit[field] : row[field]) as LoanIncome[K];
  };

  const setField = (rowId: string, field: keyof LoanIncome, value: unknown) => {
    setEdits((e) => ({ ...e, [rowId]: { ...e[rowId], [field]: value } }));
  };

  const totalMonthly = () =>
    (detail()?.income ?? []).reduce(
      (sum, row) => sum + Number(rowValue(row, 'monthly_amount') ?? 0),
      0,
    );

  const totalTwoYear = () =>
    (detail()?.income ?? []).reduce(
      (sum, row) => sum + (twoYearAvg(row) ?? Number(rowValue(row, 'monthly_amount') ?? 0)),
      0,
    );

  const savedFlash = (rowId: string) => {
    setRowStatus((s) => ({ ...s, [rowId]: 'saved' }));
    setTimeout(() => {
      setRowStatus((s) => (s[rowId] === 'saved' ? { ...s, [rowId]: undefined as never } : s));
    }, 1800);
  };

  // Autocommit: fires on field blur / Enter. No save button to hunt for.
  const commitRow = async (row: LoanIncome) => {
    if (!edits()[row.id]) return;
    if (props.demo) return; // demo keeps edits on screen, nothing persisted

    setRowStatus((s) => ({ ...s, [row.id]: 'saving' }));
    setRowErrors((e) => ({ ...e, [row.id]: '' }));
    try {
      await invoke('save_income', {
        loanId: props.loanId,
        income: {
          id: row.id,
          borrower_id: row.borrower_id,
          income_type: rowValue(row, 'income_type'),
          monthly_amount: Number(rowValue(row, 'monthly_amount')) || 0,
          prior_year_monthly: rowValue(row, 'prior_year_monthly'),
          verified: rowValue(row, 'verified'),
          employer_name: rowValue(row, 'employer_name'),
          years_on_job: rowValue(row, 'years_on_job'),
          notes: rowValue(row, 'notes'),
        },
      });
      setEdits((e) => {
        const next = { ...e };
        delete next[row.id];
        return next;
      });
      refetch();
      savedFlash(row.id);
    } catch (err) {
      setRowStatus((s) => ({ ...s, [row.id]: 'error' }));
      setRowErrors((e) => ({ ...e, [row.id]: String(err) }));
    }
  };

  const statusLabel = (rowId: string) => {
    const s = rowStatus()[rowId];
    if (s === 'saving') return 'Saving…';
    if (s === 'saved') return 'Saved ✓';
    return null;
  };

  const numInput = (
    row: LoanIncome,
    f: 'monthly_amount' | 'prior_year_monthly' | 'years_on_job',
    step?: string,
  ) => (
    <input
      type="number"
      min="0"
      step={step}
      value={rowValue(row, f) ?? ''}
      onInput={(e) =>
        setField(
          row.id,
          f,
          Number.isNaN(e.currentTarget.valueAsNumber)
            ? f === 'monthly_amount'
              ? 0
              : null
            : e.currentTarget.valueAsNumber,
        )
      }
      onBlur={() => commitRow(row)}
      onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
    />
  );

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="income" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        work={
          <Show when={detail()} fallback={<div class="screen-loading">Loading…</div>}>
        <Show when={props.demo}>
          <div class="demo-note">Demo mode — edits stay on this screen and are not persisted.</div>
        </Show>
        <table class="data-table editable">
          <thead>
            <tr>
              <th>Type</th>
              <th>Monthly amount</th>
              <th>Prior year (mo)</th>
              <th>Employer</th>
              <th>Years on job</th>
              <th>Notes</th>
              <th title="Verified">✓</th>
              <th />
            </tr>
          </thead>
          <tbody>
            <For each={detail()!.income}>
              {(row) => (
                <tr>
                  <td>
                    <select
                      value={rowValue(row, 'income_type')}
                      onChange={(e) => {
                        setField(row.id, 'income_type', e.currentTarget.value);
                        commitRow(row);
                      }}
                      onBlur={() => commitRow(row)}
                    >
                      <For each={[...INCOME_TYPES]}>
                        {(t) => <option value={t}>{t.replace(/_/g, ' ')}</option>}
                      </For>
                    </select>
                  </td>
                  <td>{numInput(row, 'monthly_amount')}</td>
                  <td>{numInput(row, 'prior_year_monthly')}</td>
                  <td>
                    <input
                      type="text"
                      value={rowValue(row, 'employer_name') ?? ''}
                      onInput={(e) => setField(row.id, 'employer_name', e.currentTarget.value)}
                      onBlur={() => commitRow(row)}
                      onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
                    />
                  </td>
                  <td>{numInput(row, 'years_on_job', '0.1')}</td>
                  <td>
                    <input
                      type="text"
                      value={rowValue(row, 'notes') ?? ''}
                      onInput={(e) => setField(row.id, 'notes', e.currentTarget.value)}
                      onBlur={() => commitRow(row)}
                      onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
                    />
                  </td>
                  <td class="check-cell">
                    <input
                      type="checkbox"
                      checked={rowValue(row, 'verified')}
                      title="Verified"
                      onChange={(e) => {
                        setField(row.id, 'verified', e.currentTarget.checked);
                        commitRow(row);
                      }}
                    />
                  </td>
                  <td class="row-status-cell">
                    <span class={`row-status ${rowStatus()[row.id] ?? ''}`}>
                      {statusLabel(row.id)}
                    </span>
                    <Show when={rowStatus()[row.id] === 'error'}>
                      <div class="row-error">{rowErrors()[row.id]}</div>
                    </Show>
                  </td>
                </tr>
              )}
            </For>
          </tbody>
          <tfoot>
            <tr>
              <td>Total qualifying</td>
              <td class="mono">
                ${totalMonthly().toLocaleString(undefined, { maximumFractionDigits: 0 })}/mo
              </td>
              <td />
              <td colspan="3">
                2-yr avg (prior-year rows):{' '}
                <span class="mono">
                  ${totalTwoYear().toLocaleString(undefined, { maximumFractionDigits: 0 })}/mo
                </span>
              </td>
              <td colspan="2" />
            </tr>
          </tfoot>
        </table>
          </Show>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="income"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'income')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="income" />
    </div>
  );
}
