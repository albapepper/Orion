import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { Demographics, LoanDetail } from '../../types';
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

const ETHNICITY = ['hispanic_or_latino', 'not_hispanic_or_latino', 'not_provided'];
const RACE = [
  'american_indian_or_alaska_native',
  'asian',
  'black_or_african_american',
  'native_hawaiian_or_pacific_islander',
  'white',
  'not_provided',
];
const SEX = ['male', 'female', 'not_provided'];
const BASIS = ['face_to_face', 'not_face_to_face'];

const label = (s: string) =>
  s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());

// HMDA government monitoring (URLA §8). Compliance capture only:
// no AI context, no inference, values never logged.
export function DemographicScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId, screen: 'demographic' }),
  );
  const [selectedId, setSelectedId] = createSignal<string | null>(null);
  const [edits, setEdits] = createSignal<Record<string, Partial<Demographics>>>({});
  const [status, setStatus] = createSignal<'saving' | 'saved' | 'error' | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const borrowers = () => detail()?.borrowers ?? [];
  const selected = () =>
    borrowers().find((b) => b.id === selectedId()) ?? borrowers()[0];

  const rowFor = (bid: string | undefined): Demographics | undefined =>
    (detail()?.demographics ?? []).find((d) => d.borrower_id === bid);

  const field = <K extends keyof Demographics>(f: K): Demographics[K] => {
    const b = selected();
    const e = b ? edits()[b.id] : undefined;
    return (e && f in e ? e[f] : rowFor(b?.id)?.[f]) as Demographics[K];
  };

  const setField = <K extends keyof Demographics>(f: K, v: Demographics[K]) => {
    const b = selected();
    if (!b) return;
    setEdits((e) => ({ ...e, [b.id]: { ...e[b.id], [f]: v } }));
  };

  const commit = async () => {
    const b = selected();
    const e = b ? edits()[b.id] : undefined;
    if (!b || !e || props.demo) return;
    const row = rowFor(b.id);
    setStatus('saving');
    setError(null);
    try {
      await invoke('save_demographics', {
        loanId: props.loanId,
        input: {
          borrower_id: b.id,
          ethnicity:
            e.ethnicity_choices !== undefined
              ? e.ethnicity_choices
              : (row?.ethnicity_choices ?? []),
          race:
            e.race_choices !== undefined ? e.race_choices : (row?.race_choices ?? []),
          sex: e.sex !== undefined ? e.sex : (row?.sex ?? null),
          collection_basis:
            e.collection_basis !== undefined
              ? e.collection_basis
              : (row?.collection_basis ?? null),
        },
      });
      setEdits((all) => {
        const next = { ...all };
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

  const select = (f: 'sex' | 'collection_basis', opts: string[]) => (
    <select
      value={(field(f) as string) ?? ''}
      onChange={(e) => {
        setField(f, e.currentTarget.value || null);
        commit();
      }}
    >
      <option value="">Not collected</option>
      <For each={opts}>{(o) => <option value={o}>{label(o)}</option>}</For>
    </select>
  );

  // Multi-select "all that apply" (P3-15). Checking the decline box
  // clears the rest; checking any real box un-declines.
  const choices = (f: 'ethnicity_choices' | 'race_choices', opts: string[]) => {
    const current = (field(f) as string[]) ?? [];
    return (
      <div class="check-group">
        <For each={opts}>
          {(o) => (
            <label class="check-option">
              <input
                type="checkbox"
                checked={current.includes(o)}
                onChange={() => {
                  let next: string[];
                  if (o === 'not_provided') {
                    next = current.includes(o) ? [] : ['not_provided'];
                  } else {
                    next = current.filter((c) => c !== 'not_provided');
                    next = next.includes(o) ? next.filter((c) => c !== o) : [...next, o];
                  }
                  setField(f, next);
                  commit();
                }}
              />
              {label(o)}
            </label>
          )}
        </For>
      </div>
    );
  };

  const complete = () => {
    const row = rowFor(selected()?.id);
    return Boolean(
      row &&
        (row.ethnicity_choices.length > 0 || row.race_choices.length > 0) &&
        row.sex &&
        row.collection_basis,
    );
  };

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="demographic" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        lock={detail()?.lock ?? null}
        loanId={props.loanId}
        demo={props.demo}
        work={
          <Show when={selected()} fallback={<div class="screen-loading">Loading…</div>}>
        <div class="screen-title-row">
          <h2 class="screen-title">Government monitoring — URLA §8</h2>
          <span class="task-count">{complete() ? 'Complete' : 'Incomplete'}</span>
          <span class={`save-status ${status() ?? ''}`}>
            {status() === 'saving' ? 'Saving…' : status() === 'saved' ? 'Saved ✓' : null}
          </span>
        </div>
        <div class="demo-note" style={props.demo ? '' : 'display:none'}>
          Demo mode — edits stay on this screen and are not persisted.
        </div>

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

        <div class="form-grid">
          <div class="field">
            <label>Ethnicity — all that apply</label>
            {choices('ethnicity_choices', ETHNICITY)}
          </div>
          <div class="field">
            <label>Race — all that apply</label>
            {choices('race_choices', RACE)}
          </div>
          <div class="field">
            <label>Sex</label>
            {select('sex', SEX)}
          </div>
          <div class="field">
            <label>Collected</label>
            {select('collection_basis', BASIS)}
          </div>
        </div>
        <p class="screen-hint">
          Collected for HMDA compliance — check every box the borrower
          selected, or "Not Provided" if they decline. Answers are
          recorded exactly as given — never inferred, summarized, or
          sent to the assistant.
        </p>

        <Show when={status() === 'error'}>
          <div class="form-error">{error()}</div>
        </Show>
      </Show>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="demographic"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'demographic')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="demographic" demo={props.demo} />
    </div>
  );
}
