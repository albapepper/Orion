import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { Declaration, LoanDetail } from '../../types';
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

// URLA §5 — same keys as the backend context (loans/screens/declarations.rs)
const QUESTIONS: Array<[string, string]> = [
  ['primary_residence', "Is this the borrower's primary residence?"],
  ['bankruptcy_7yr', 'Has the borrower declared bankruptcy in the last 7 years?'],
  ['foreclosure_7yr', 'Has the borrower had property foreclosed upon in the last 7 years?'],
  ['lawsuit_7yr', 'Has the borrower been a party to a lawsuit in the last 7 years?'],
  [
    'obligated_foreclosure_7yr',
    'Has the borrower been obligated on a loan that resulted in foreclosure, short sale, or deed-in-lieu in the last 7 years?',
  ],
  ['delinquent_federal', 'Is the borrower presently delinquent on any federal debt?'],
  [
    'alimony_child_support',
    'Is the borrower obligated to pay alimony, child support, or separate maintenance?',
  ],
  ['co_signer', 'Is the borrower a co-signer or guarantor on any debt not listed?'],
  ['us_citizen', 'Is the borrower a U.S. citizen?'],
  ['permanent_resident', 'Is the borrower a permanent resident alien?'],
];

export function DeclarationsScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId }),
  );
  const [selectedId, setSelectedId] = createSignal<string | null>(null);
  // Working edits keyed `${borrowerId}:${questionKey}`
  const [edits, setEdits] = createSignal<Record<string, Partial<Declaration>>>({});
  const [status, setStatus] = createSignal<'saving' | 'saved' | 'error' | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const borrowers = () => detail()?.borrowers ?? [];
  const selected = () =>
    borrowers().find((b) => b.id === selectedId()) ?? borrowers()[0];

  const rowFor = (key: string): Declaration | undefined =>
    (detail()?.declarations ?? []).find(
      (d) => d.borrower_id === selected()?.id && d.question_key === key,
    );

  const setEdit = (key: string, patch: Partial<Declaration>) => {
    const k = `${selected()?.id}:${key}`;
    setEdits((e) => ({ ...e, [k]: { ...e[k], ...patch } }));
  };

  const commit = async (key: string) => {
    const b = selected();
    if (!b || props.demo) return;
    const k = `${b.id}:${key}`;
    const e = edits()[k];
    if (!e || (e.answer === undefined && e.explanation === undefined)) return;
    const row = rowFor(key);
    setStatus('saving');
    setError(null);
    try {
      await invoke('save_declaration', {
        loanId: props.loanId,
        declaration: {
          borrower_id: b.id,
          question_key: key,
          answer: e.answer !== undefined ? e.answer : (row?.answer ?? null),
          explanation:
            e.explanation !== undefined ? e.explanation : (row?.explanation ?? null),
        },
      });
      setEdits((all) => {
        const next = { ...all };
        delete next[k];
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

  const answeredCount = () =>
    QUESTIONS.filter(([key]) => rowFor(key)?.answer !== null && rowFor(key)?.answer !== undefined)
      .length;

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="declarations" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        work={
          <Show when={selected()} fallback={<div class="screen-loading">Loading…</div>}>
        <div class="screen-title-row">
          <h2 class="screen-title">Declarations — URLA §5</h2>
          <span class="task-count">{answeredCount()}/{QUESTIONS.length} answered</span>
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
                </button>
              )}
            </For>
          </div>
        </Show>

        <div class="decl-list">
          <For each={QUESTIONS}>
            {([key, text]) => {
              const answer = () => rowFor(key)?.answer ?? null;
              return (
                <div class="decl-row">
                  <div class="decl-question">{text}</div>
                  <div class="decl-answer">
                    <button
                      class="decl-toggle"
                      classList={{ active: answer() === true, yes: answer() === true }}
                      onClick={() => {
                        setEdit(key, { answer: answer() === true ? null : true });
                        commit(key);
                      }}
                    >
                      Yes
                    </button>
                    <button
                      class="decl-toggle"
                      classList={{ active: answer() === false, no: answer() === false }}
                      onClick={() => {
                        setEdit(key, { answer: answer() === false ? null : false });
                        commit(key);
                      }}
                    >
                      No
                    </button>
                  </div>
                  <input
                    type="text"
                    class="decl-explanation"
                    placeholder="Explanation (required for Yes)…"
                    value={rowFor(key)?.explanation ?? ''}
                    onInput={(e) => setEdit(key, { explanation: e.currentTarget.value || null })}
                    onBlur={() => commit(key)}
                    onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
                  />
                </div>
              );
            }}
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
            screen="declarations"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'declarations')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="declarations" />
    </div>
  );
}
