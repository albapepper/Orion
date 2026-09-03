// Shared scaffolding for list-style 1003 screens (assets, liabilities,
// REO): rows filtered per borrower, autosave on blur/Enter, add/remove,
// demo-aware. Configure with columns + command names; no per-screen
// plumbing beyond the payload mapping.

import { createResource, createSignal, For, JSX, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import { TaskPanel } from '../../components/TaskPanel';
import { LoanHeader } from '../../components/LoanHeader';
import { PizzaTracker } from '../../components/PizzaTracker';
import { LoanColumns } from '../../components/LoanColumns';
import { UnderwriterPanel } from '../../components/UnderwriterPanel';
import type { LoanDetail } from '../../types';
import type { LoanScreen } from './stages';

export interface ColDef<T> {
  key: keyof T & string;
  label: string;
  type: 'text' | 'number' | 'select' | 'check';
  options?: string[];
  step?: string;
  placeholder?: string;
  title?: string;
}

interface BaseRow {
  id: string;
  borrower_id: string;
  loan_id: string;
  company_id: string;
}

interface BorrowerRow extends BaseRow {
  first_name: string;
  last_name: string;
}

interface Props<T extends BaseRow> {
  loanId: string;
  demo: boolean;
  onNavigate: (screen: string) => void;
  screen: LoanScreen;
  title: string;
  addLabel: string;
  emptyText: string;
  columns: ColDef<T>[];
  commands: { save: string; remove?: string };
  saveArg: string;
  removeArg?: string;
  toPayload: (row: T) => Record<string, unknown>;
  newRow: (borrower: BaseRow) => T;
  summary?: (rows: T[]) => JSX.Element | null;
}

export function LoanListScreen<T extends BaseRow>(props: Props<T>) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<unknown>('get_loan_detail', { loanId, screen: props.screen }) as never,
  );
  const [selectedId, setSelectedId] = createSignal<string | null>(null);
  const [edits, setEdits] = createSignal<Record<string, Partial<T>>>({});
  const [localRows, setLocalRows] = createSignal<T[]>([]);
  const [rowStatus, setRowStatus] = createSignal<Record<string, 'saving' | 'saved' | 'error'>>({});
  const [rowErrors, setRowErrors] = createSignal<Record<string, string>>({});

  const borrowers = (): BorrowerRow[] =>
    ((detail() as never as { borrowers?: BorrowerRow[] })?.borrowers ?? []);
  const selected = () => borrowers().find((b) => b.id === selectedId()) ?? borrowers()[0];

  const resourceRows = () =>
    ((detail() as never as Record<string, T[]> | undefined)?.[props.screen] ??
      []) as T[];

  const rows = (): T[] => [
    ...resourceRows().filter((r) => r.borrower_id === selected()?.id),
    ...(props.demo ? localRows().filter((r) => r.borrower_id === selected()?.id) : []),
  ];

  const rowValue = <K extends keyof T & string>(row: T, key: K): T[K] => {
    const e = edits()[row.id];
    return (e && key in e ? e[key] : row[key]) as T[K];
  };

  const setField = (row: T, key: keyof T & string, value: unknown) => {
    setEdits((e) => ({ ...e, [row.id]: { ...e[row.id], [key]: value } }));
  };

  const savedFlash = (rowId: string) => {
    setRowStatus((s) => ({ ...s, [rowId]: 'saved' }));
    setTimeout(
      () => setRowStatus((s) => (s[rowId] === 'saved' ? { ...s, [rowId]: undefined as never } : s)),
      1800,
    );
  };

  const commitRow = async (row: T) => {
    const patch = edits()[row.id];
    if (!patch || props.demo) return;
    setRowStatus((s) => ({ ...s, [row.id]: 'saving' }));
    try {
      const merged = { ...row, ...patch };
      await invoke(props.commands.save, {
        loanId: props.loanId,
        [props.saveArg]: props.toPayload(merged),
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

  const addRow = async () => {
    const b = selected();
    if (!b) return;
    const row = props.newRow(b);
    if (props.demo) {
      setLocalRows((r) => [...r, row]);
      return;
    }
    try {
      await invoke(props.commands.save, {
        loanId: props.loanId,
        [props.saveArg]: props.toPayload({ ...row, id: null as never }),
      });
      refetch();
    } catch (err) {
      setRowErrors((e) => ({ ...e, '_add': String(err) }));
    }
  };

  const removeRow = async (row: T) => {
    if (props.demo) {
      setLocalRows((r) => r.filter((x) => x.id !== row.id));
      return;
    }
    if (!props.commands.remove || !props.removeArg) return;
    try {
      await invoke(props.commands.remove, { [props.removeArg]: row.id });      refetch();
    } catch (err) {
      setRowErrors((e) => ({ ...e, [row.id]: String(err) }));
    }
  };

  const cell = (row: T, col: ColDef<T>) => {
    switch (col.type) {
      case 'select':
        return (
          <select
            value={rowValue(row, col.key) as string}
            onChange={(e) => {
              setField(row, col.key, e.currentTarget.value);
              commitRow(row);
            }}
            onBlur={() => commitRow(row)}
          >
            <For each={col.options ?? []}>
              {(o) => <option value={o}>{o.replace(/_/g, ' ')}</option>}
            </For>
          </select>
        );
      case 'number':
        return (
          <input
            type="number"
            min="0"
            step={col.step}
            value={(rowValue(row, col.key) as number) ?? ''}
            onInput={(e) =>
              setField(
                row,
                col.key,
                Number.isNaN(e.currentTarget.valueAsNumber) ? null : e.currentTarget.valueAsNumber,
              )
            }
            onBlur={() => commitRow(row)}
            onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
          />
        );
      case 'check':
        return (
          <input
            type="checkbox"
            checked={(rowValue(row, col.key) as boolean) ?? false}
            onChange={(e) => {
              setField(row, col.key, e.currentTarget.checked);
              commitRow(row);
            }}
          />
        );
      default:
        return (
          <input
            type="text"
            placeholder={col.placeholder}
            value={(rowValue(row, col.key) as string) ?? ''}
            onInput={(e) => setField(row, col.key, e.currentTarget.value || null)}
            onBlur={() => commitRow(row)}
            onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
          />
        );
    }
  };

  return (
    <div class="screen">
      <LoanHeader detail={detail() as never} />
      <PizzaTracker
        current={props.screen}
        detail={detail() as LoanDetail | undefined}
        onNavigate={props.onNavigate}
      />

      <LoanColumns
        lock={(detail() as LoanDetail | undefined)?.lock ?? null}
        work={
          <>
            <div class="screen-title-row">
              <h2 class="screen-title">{props.title}</h2>
              <button class="ghost small" style="margin-left: auto" onClick={addRow}>
                {props.addLabel}
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
                      classList={{ active: b.id === selected()?.id }}
                      onClick={() => setSelectedId(b.id)}
                    >
                      {b.first_name} {b.last_name}
                    </button>
                  )}
                </For>
              </div>
            </Show>

      <table class="data-table editable">
        <thead>
          <tr>
            <For each={props.columns}>{(col) => <th title={col.title}>{col.label}</th>}</For>
            <th />
          </tr>
        </thead>
        <tbody>
          <For
            each={rows()}
            fallback={<tr><td colspan={props.columns.length + 1}>{props.emptyText}</td></tr>}
          >
            {(row) => (
              <tr>
                <For each={props.columns}>
                  {(col) => (
                    <td classList={{ 'check-cell': col.type === 'check' }}>{cell(row, col)}</td>
                  )}
                </For>
                <td class="row-status-cell">
                  <Show when={props.commands.remove}>
                    <button
                      class="ghost small"
                      title="Remove"
                      onClick={() => removeRow(row)}
                    >
                      ✕
                    </button>
                  </Show>
                  <span class={`row-status ${rowStatus()[row.id] ?? ''}`}>
                    {rowStatus()[row.id] === 'saving' ? 'Saving…' : rowStatus()[row.id] === 'saved' ? 'Saved ✓' : null}
                  </span>
                  <Show when={rowStatus()[row.id] === 'error'}>
                    <div class="row-error">{rowErrors()[row.id]}</div>
                  </Show>
                </td>
              </tr>
            )}
          </For>
        </tbody>
        <Show when={props.summary}>
          <tfoot>{props.summary?.(rows())}</tfoot>
        </Show>
      </table>
          </>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen={props.screen}
            review={
              (detail() as LoanDetail | undefined)?.uw_reviews?.find(
                (r) => r.screen === props.screen,
              )
            }
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen={props.screen} demo={props.demo} />
    </div>
  );
}
