import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanDetail, LoanIncome } from '../../types';
import { INCOME_TYPES } from '../../types';
import { TaskPanel } from '../../components/TaskPanel';
import { LoanHeader } from '../../components/LoanHeader';

interface Props {
  loanId: string;
  onBack: () => void;
}

export function IncomeScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId }),
  );
  const [saving, setSaving] = createSignal(false);
  const [saveError, setSaveError] = createSignal<string | null>(null);
  const [edits, setEdits] = createSignal<Record<string, Partial<LoanIncome>>>({});

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

  const saveRow = async (row: LoanIncome) => {
    setSaving(true);
    setSaveError(null);
    try {
      await invoke('save_income', {
        loanId: props.loanId,
        income: {
          id: row.id,
          borrower_id: row.borrower_id,
          income_type: rowValue(row, 'income_type'),
          monthly_amount: Number(rowValue(row, 'monthly_amount')),
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
    } catch (e) {
      setSaveError(String(e));
    } finally {
      setSaving(false);
    }
  };

  return (
    <div class="screen">
      <LoanHeader detail={detail()} screenLabel="Income Analysis" onBack={props.onBack} />

      <Show when={detail()} fallback={<div class="screen-loading">Loading…</div>}>
        <table class="data-table editable">
          <thead>
            <tr>
              <th>Type</th>
              <th>Monthly amount</th>
              <th>Employer</th>
              <th>Years on job</th>
              <th>Notes</th>
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
                      onChange={(e) => setField(row.id, 'income_type', e.currentTarget.value)}
                    >
                      <For each={[...INCOME_TYPES]}>
                        {(t) => <option value={t}>{t}</option>}
                      </For>
                    </select>
                  </td>
                  <td>
                    <input
                      type="number"
                      value={rowValue(row, 'monthly_amount')}
                      onInput={(e) =>
                        setField(row.id, 'monthly_amount', e.currentTarget.valueAsNumber)
                      }
                    />
                  </td>
                  <td>
                    <input
                      type="text"
                      value={rowValue(row, 'employer_name') ?? ''}
                      onInput={(e) => setField(row.id, 'employer_name', e.currentTarget.value)}
                    />
                  </td>
                  <td>
                    <input
                      type="number"
                      step="0.1"
                      value={rowValue(row, 'years_on_job') ?? ''}
                      onInput={(e) =>
                        setField(row.id, 'years_on_job', e.currentTarget.valueAsNumber)
                      }
                    />
                  </td>
                  <td>
                    <input
                      type="text"
                      value={rowValue(row, 'notes') ?? ''}
                      onInput={(e) => setField(row.id, 'notes', e.currentTarget.value)}
                    />
                  </td>
                  <td>
                    <button
                      class="primary small"
                      disabled={saving() || !edits()[row.id]}
                      onClick={() => saveRow(row)}
                    >
                      Save
                    </button>
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
              <td colspan="4" />
            </tr>
          </tfoot>
        </table>
        <Show when={saveError()}>
          <div class="form-error">{saveError()}</div>
        </Show>
      </Show>

      <TaskPanel loanId={props.loanId} screen="income" />
    </div>
  );
}
