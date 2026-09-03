import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { Borrower, LoanDetail } from '../../types';
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

const MARITAL = ['married', 'unmarried', 'separated'];
const CITIZENSHIP = ['us_citizen', 'permanent_resident', 'non_permanent_resident'];

const label = (s: string) =>
  s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());

type WorkDep = { key: string; id: string | null; age: number | null };

export function BorrowerScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId }),
  );
  const [selectedId, setSelectedId] = createSignal<string | null>(null);
  const [edits, setEdits] = createSignal<Record<string, Partial<Borrower>>>({});
  const [depWork, setDepWork] = createSignal<Record<string, WorkDep[]>>({});
  const [status, setStatus] = createSignal<'saving' | 'saved' | 'error' | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const borrowers = () => detail()?.borrowers ?? [];
  const selected = () =>
    borrowers().find((b) => b.id === selectedId()) ?? borrowers()[0];

  const field = <K extends keyof Borrower>(f: K): Borrower[K] => {
    const b = selected();
    if (!b) return undefined as never;
    const e = edits()[b.id];
    return (e && f in e ? e[f] : b[f]) as Borrower[K];
  };

  const setField = <K extends keyof Borrower>(f: K, v: Borrower[K]) => {
    const b = selected();
    if (!b) return;
    setEdits((e) => ({ ...e, [b.id]: { ...e[b.id], [f]: v } }));
  };

  const hasEdits = () => {
    const b = selected();
    return b ? Object.keys(edits()[b.id] ?? {}).length > 0 : false;
  };

  const commit = async () => {
    const b = selected();
    if (!b || !hasEdits() || props.demo) return;
    setStatus('saving');
    setError(null);
    try {
      await invoke('save_borrower', {
        borrowerId: b.id,
        borrower: {
          first_name: field('first_name'),
          last_name: field('last_name'),
          middle_name: field('middle_name'),
          suffix_name: field('suffix_name'),
          ssn: field('ssn'),
          birth_date: field('birth_date'),
          marital_status: field('marital_status'),
          citizenship_status: field('citizenship_status'),
          email: field('email'),
          phone: field('phone'),
        },
      });
      setEdits((e) => {
        const next = { ...e };
        delete next[b.id];
        return next;
      });
      refetch();
      setStatus('saved');
      setTimeout(() => setStatus((s) => (s === 'saved' ? null : s)), 1800);
    } catch (err) {
      setStatus('error');
      setError(String(err));
    }
  };

  // ── Dependents: working list per borrower, committed as a whole ──

  const depList = (): WorkDep[] => {
    const b = selected();
    if (!b) return [];
    const w = depWork()[b.id];
    if (w) return w;
    return (detail()?.dependents ?? [])
      .filter((d) => d.borrower_id === b.id)
      .map((d) => ({ key: d.id, id: d.id, age: d.age }));
  };

  const setDepList = (list: WorkDep[]) => {
    const b = selected();
    if (!b) return;
    setDepWork((s) => ({ ...s, [b.id]: list }));
  };

  const commitDependents = async () => {
    const b = selected();
    if (!b || props.demo) return;
    const list = depWork()[b.id];
    if (!list) return;
    setError(null);
    try {
      await invoke('save_dependents', {
        loanId: props.loanId,
        input: {
          borrower_id: b.id,
          ages: list.map((w) => w.age),
        },
      });
      refetch();
      setDepWork((s) => {
        const next = { ...s };
        delete next[b.id];
        return next;
      });
      setStatus('saved');
      setTimeout(() => setStatus((s) => (s === 'saved' ? null : s)), 1800);
    } catch (err) {
      setStatus('error');
      setError(String(err));
    }
  };

  const addDependent = () => {
    setDepList([
      ...depList(),
      { key: crypto.randomUUID(), id: null, age: null },
    ]);
    commitDependents();
  };

  const removeDependent = (key: string) => {
    setDepList(depList().filter((d) => d.key !== key));
    commitDependents();
  };

  const textInput = (f: keyof Borrower, extra?: Record<string, unknown>) => (
    <input
      type="text"
      value={(field(f) as string) ?? ''}
      onInput={(e) => setField(f, e.currentTarget.value as never)}
      onBlur={commit}
      onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
      {...extra}
    />
  );

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="borrower" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        work={
          <>
            <Show when={selected()} fallback={<div class="screen-loading">Loading…</div>}>
        <div class="screen-title-row">
          <h2 class="screen-title">Applicant profile</h2>
          <span class={`save-status ${status() ?? ''}`}>
            {status() === 'saving' ? 'Saving…' : status() === 'saved' ? 'Saved ✓' : null}
          </span>
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
                  <Show when={b.is_primary}> ·</Show>
                </button>
              )}
            </For>
          </div>
        </Show>

        <div class="form-grid">
          <div class="field">
            <label>First name</label>
            {textInput('first_name')}
          </div>
          <div class="field">
            <label>Middle name</label>
            {textInput('middle_name')}
          </div>
          <div class="field">
            <label>Last name</label>
            {textInput('last_name')}
          </div>
          <div class="field">
            <label>Suffix</label>
            {textInput('suffix_name', { placeholder: 'Jr., III…' })}
          </div>
          <div class="field">
            <label>Social security number</label>
            {textInput('ssn', { placeholder: '123-45-6789', autocomplete: 'off' })}
          </div>
          <div class="field">
            <label>Date of birth</label>
            <input
              type="date"
              value={field('birth_date') ?? ''}
              onInput={(e) => setField('birth_date', e.currentTarget.value || null)}
              onBlur={commit}
            />
          </div>
          <div class="field">
            <label>Marital status</label>
            <select
              value={field('marital_status') ?? ''}
              onChange={(e) => {
                setField('marital_status', e.currentTarget.value || null);
                commit();
              }}
            >
              <option value="">Not entered</option>
              <For each={MARITAL}>{(m) => <option value={m}>{label(m)}</option>}</For>
            </select>
          </div>
          <div class="field">
            <label>Citizenship</label>
            <select
              value={field('citizenship_status') ?? ''}
              onChange={(e) => {
                setField('citizenship_status', e.currentTarget.value || null);
                commit();
              }}
            >
              <option value="">Not entered</option>
              <For each={CITIZENSHIP}>{(c) => <option value={c}>{label(c)}</option>}</For>
            </select>
          </div>
          <div class="field">
            <label>Email</label>
            {textInput('email')}
          </div>
          <div class="field">
            <label>Phone</label>
            {textInput('phone')}
          </div>
        </div>

        <div class="screen-title-row" style="margin-top: 32px">
          <h2 class="screen-title">Dependents</h2>
          <button class="ghost small" onClick={addDependent}>
            + Add
          </button>
        </div>
        <div class="dependents-panel">
          <Show
            when={depList().length > 0}
            fallback={<div class="task-empty">No dependents on this loan.</div>}
          >
            <For each={depList()}>
              {(dep, i) => (
                <div class="dep-row">
                  <span class="dep-index mono">{i() + 1}</span>
                  <input
                    type="number"
                    min="0"
                    max="99"
                    placeholder="Age"
                    value={dep.age ?? ''}
                    onInput={(e) => {
                      const v = e.currentTarget.valueAsNumber;
                      const age = Number.isNaN(v) ? null : v;
                      setDepList(
                        depList().map((d) =>
                          d.key === dep.key ? { ...d, age } : d,
                        ),
                      );
                    }}
                    onBlur={commitDependents}
                    onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
                  />
                  <button
                    class="ghost small dep-remove"
                    title="Remove dependent"
                    onClick={() => removeDependent(dep.key)}
                  >
                    Remove
                  </button>
                </div>
              )}
            </For>
          </Show>
        </div>

        <Show when={status() === 'error'}>
          <div class="form-error">{error()}</div>
        </Show>
            </Show>
          </>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="borrower"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'borrower')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="borrower" />
    </div>
  );
}
