import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { Employment, LoanDetail } from '../../types';
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

type Row = Employment & { key?: string }; // key set on demo-only local rows

export function EmploymentScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId }),
  );
  const [selectedId, setSelectedId] = createSignal<string | null>(null);
  const [edits, setEdits] = createSignal<Record<string, Partial<Employment>>>({});
  const [localRows, setLocalRows] = createSignal<Row[]>([]);
  const [status, setStatus] = createSignal<'saving' | 'saved' | 'error' | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const borrowers = () => detail()?.borrowers ?? [];
  const selected = () =>
    borrowers().find((b) => b.id === selectedId()) ?? borrowers()[0];

  const rows = (): Row[] => [
    ...(detail()?.employments ?? []).filter(
      (e) => e.borrower_id === selected()?.id,
    ),
    ...(props.demo ? localRows() : []),
  ];

  const rowValue = <K extends keyof Employment>(row: Row, f: K): Employment[K] => {
    const e = edits()[row.id];
    return (e && f in e ? e[f] : row[f]) as Employment[K];
  };

  const setField = <K extends keyof Employment>(row: Row, f: K, v: Employment[K]) => {
    setEdits((e) => ({ ...e, [row.id]: { ...e[row.id], [f]: v } }));
  };

  const flash = () => {
    setStatus('saved');
    setTimeout(() => setStatus((s) => (s === 'saved' ? null : s)), 1800);
  };

  const commitRow = async (row: Row) => {
    const patch = edits()[row.id];
    if (!patch || props.demo) return;
    setStatus('saving');
    setError(null);
    try {
      await invoke('save_employment', {
        loanId: props.loanId,
        employment: {
          id: row.key ?? row.id,
          borrower_id: row.borrower_id,
          employer_name: rowValue(row, 'employer_name'),
          position: rowValue(row, 'position'),
          self_employed: rowValue(row, 'self_employed'),
          monthly_income: rowValue(row, 'monthly_income'),
          start_date: rowValue(row, 'start_date'),
          end_date: rowValue(row, 'is_current') ? null : rowValue(row, 'end_date'),
          is_current: rowValue(row, 'is_current'),
          months_in_line_of_work: rowValue(row, 'months_in_line_of_work'),
          business_phone: rowValue(row, 'business_phone'),
        },
      });
      setEdits((e) => {
        const next = { ...e };
        delete next[row.id];
        return next;
      });
      refetch();
      flash();
    } catch (err) {
      setStatus('error');
      setError(String(err));
    }
  };

  const addEmployment = async () => {
    const b = selected();
    if (!b) return;
    if (props.demo) {
      setLocalRows((r) => [
        ...r,
        {
          id: crypto.randomUUID(),
          key: crypto.randomUUID(),
          borrower_id: b.id,
          loan_id: b.loan_id,
          company_id: b.company_id,
          employer_name: null,
          position: null,
          self_employed: false,
          monthly_income: null,
          start_date: null,
          end_date: null,
          is_current: true,
          months_in_line_of_work: null,
          business_phone: null,
        },
      ]);
      return;
    }
    try {
      await invoke('save_employment', {
        loanId: props.loanId,
        employment: {
          id: null,
          borrower_id: b.id,
          employer_name: null,
          position: null,
          self_employed: false,
          monthly_income: null,
          start_date: null,
          end_date: null,
          is_current: true,
          months_in_line_of_work: null,
          business_phone: null,
        },
      });
      refetch();
      flash();
    } catch (err) {
      setError(String(err));
    }
  };

  const removeRow = async (row: Row) => {
    if (props.demo) {
      setLocalRows((r) => r.filter((x) => x.id !== row.id));
      return;
    }
    try {
      await invoke('delete_employment', { employmentId: row.id });
      refetch();
      flash();
    } catch (err) {
      setError(String(err));
    }
  };

  const textInput = (row: Row, f: 'employer_name' | 'position' | 'business_phone', ph?: string) => (
    <input
      type="text"
      placeholder={ph}
      value={rowValue(row, f) ?? ''}
      onInput={(e) => setField(row, f, e.currentTarget.value || null)}
      onBlur={() => commitRow(row)}
      onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
    />
  );

  const numInput = (row: Row, f: 'monthly_income' | 'months_in_line_of_work', step?: string) => (
    <input
      type="number"
      min="0"
      step={step}
      value={rowValue(row, f) ?? ''}
      onInput={(e) =>
        setField(
          row,
          f,
          Number.isNaN(e.currentTarget.valueAsNumber) ? null : e.currentTarget.valueAsNumber,
        )
      }
      onBlur={() => commitRow(row)}
      onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
    />
  );

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="employment" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        work={
          <Show when={selected()} fallback={<div class="screen-loading">Loading…</div>}>
        <div class="screen-title-row">
          <h2 class="screen-title">Two-year work history</h2>
          <span class={`save-status ${status() ?? ''}`}>
            {status() === 'saving' ? 'Saving…' : status() === 'saved' ? 'Saved ✓' : null}
          </span>
          <button class="ghost small" style="margin-left: auto" onClick={addEmployment}>
            + Add employment
          </button>
        </div>
        <Show when={props.demo}>
          <div class="demo-note">Demo mode — edits stay on this screen and are not persisted.</div>
        </Show>

        <Show when={borrowers().length > 1}>
          <div class="borrower-tabs">
            <For each={borrowers()}>
              {(b) => (
                <button
                  class="borrower-tab"
                  classList={{ active: b.id === selected()!.id }}
                  onClick={() => setSelectedId(b.id)}
                >
                  {b.first_name} {b.last_name}
                </button>
              )}
            </For>
          </div>
        </Show>

        <div class="employment-list">
          <For
            each={rows()}
            fallback={
              <div class="screen-loading">
                No employment history yet. Add the current employer first.
              </div>
            }
          >
            {(row) => (
              <div class="emp-block">
                <div class="emp-block-header">
                  <span class="emp-employer">
                    {rowValue(row, 'employer_name') || 'New employment'}
                  </span>
                  <label class="check-field">
                    <input
                      type="checkbox"
                      checked={rowValue(row, 'is_current')}
                      onChange={(e) => {
                        setField(row, 'is_current', e.currentTarget.checked);
                        if (e.currentTarget.checked) setField(row, 'end_date', null);
                        commitRow(row);
                      }}
                    />
                    Current job
                  </label>
                  <label class="check-field">
                    <input
                      type="checkbox"
                      checked={rowValue(row, 'self_employed')}
                      onChange={(e) => {
                        setField(row, 'self_employed', e.currentTarget.checked);
                        commitRow(row);
                      }}
                    />
                    Self-employed
                  </label>
                  <button
                    class="ghost small emp-remove"
                    title="Remove this employment"
                    onClick={() => removeRow(row)}
                  >
                    Remove
                  </button>
                </div>
                <div class="emp-grid">
                  <div class="field">
                    <label>Employer</label>
                    {textInput(row, 'employer_name', 'Employer name')}
                  </div>
                  <div class="field">
                    <label>Position</label>
                    {textInput(row, 'position', 'Job title')}
                  </div>
                  <div class="field">
                    <label>Monthly income</label>
                    {numInput(row, 'monthly_income')}
                  </div>
                  <div class="field">
                    <label>Start date</label>
                    <input
                      type="date"
                      value={rowValue(row, 'start_date') ?? ''}
                      onInput={(e) =>
                        setField(row, 'start_date', e.currentTarget.value || null)
                      }
                      onBlur={() => commitRow(row)}
                    />
                  </div>
                  <div class="field">
                    <label>End date</label>
                    <input
                      type="date"
                      value={rowValue(row, 'end_date') ?? ''}
                      disabled={rowValue(row, 'is_current')}
                      onInput={(e) =>
                        setField(row, 'end_date', e.currentTarget.value || null)
                      }
                      onBlur={() => commitRow(row)}
                    />
                  </div>
                  <div class="field">
                    <label>Months in line of work</label>
                    {numInput(row, 'months_in_line_of_work')}
                  </div>
                  <div class="field">
                    <label>Business phone</label>
                    {textInput(row, 'business_phone', '555-0100')}
                  </div>
                </div>
              </div>
            )}
          </For>
        </div>

        <Show when={status() === 'error'}>
          <div class="form-error">{error()}</div>
        </Show>
      </Show>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="employment"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'employment')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="employment" demo={props.demo} />
    </div>
  );
}
