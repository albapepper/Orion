import { createResource, createSignal, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanClosing, LoanDetail } from '../../types';
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

// Closing (P5-05/06): the signing appointment. One record per loan —
// when, where, who conducts it, and what the borrower brings. Scheduling
// a date is what moves the pipeline to 'closing' (stepper on Overview).
export function ClosingScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId, screen: 'closing' }),
  );
  const [edits, setEdits] = createSignal<Partial<LoanClosing>>({});
  const [status, setStatus] = createSignal<'saving' | 'saved' | 'error' | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const closing = (): LoanClosing | null => {
    const e = edits();
    const base = detail()?.closing ?? null;
    return base ? { ...base, ...e } : null;
  };

  const field = <K extends keyof LoanClosing>(f: K): LoanClosing[K] => {
    const c = closing();
    return (c?.[f] ?? null) as LoanClosing[K];
  };

  const setField = <K extends keyof LoanClosing>(f: K, v: LoanClosing[K]) => {
    setEdits((e) => ({ ...e, [f]: v }));
  };

  const commit = async () => {
    if (!hasEdits() || props.demo) return;
    setStatus('saving');
    setError(null);
    try {
      await invoke('save_closing', {
        loanId: props.loanId,
        closing: {
          closing_date: field('closing_date'),
          closing_location: field('closing_location'),
          settlement_agent: field('settlement_agent'),
          closing_costs: field('closing_costs'),
          notes: field('notes'),
        },
      });
      setEdits({});
      refetch();
      setStatus('saved');
      setTimeout(() => setStatus((s) => (s === 'saved' ? null : s)), 1800);
    } catch (err) {
      setStatus('error');
      setError(String(err));
    }
  };

  const hasEdits = () => Object.keys(edits()).length > 0;

  const textInput = (f: keyof LoanClosing, extra?: Record<string, unknown>) => (
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
      <PizzaTracker current="closing" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        lock={detail()?.lock ?? null}
        loanId={props.loanId}
        demo={props.demo}
        work={
          <Show when={detail()} fallback={<div class="screen-loading">Loading…</div>}>
            <div class="screen-title-row">
              <h2 class="screen-title">Closing</h2>
              <span class={`save-status ${status() ?? ''}`}>
                {status() === 'saving' ? 'Saving…' : status() === 'saved' ? 'Saved ✓' : null}
              </span>
            </div>
            <Show when={props.demo}>
              <div class="demo-note">Demo mode — edits stay on this screen and are not persisted.</div>
            </Show>

            <div class="form-grid">
              <div class="field">
                <label>Closing date</label>
                <input
                  type="date"
                  value={field('closing_date') ?? ''}
                  onInput={(e) => setField('closing_date', e.currentTarget.value || null)}
                  onBlur={commit}
                />
              </div>
              <div class="field">
                <label>Location</label>
                {textInput('closing_location', { placeholder: 'Title office, address…' })}
              </div>
              <div class="field">
                <label>Settlement agent</label>
                {textInput('settlement_agent')}
              </div>
              <div class="field">
                <label>Closing costs</label>
                <input
                  type="number"
                  min="0"
                  value={field('closing_costs') ?? ''}
                  onInput={(e) => {
                    const v = e.currentTarget.valueAsNumber;
                    setField('closing_costs', Number.isNaN(v) ? null : v);
                  }}
                  onBlur={commit}
                />
              </div>
            </div>
            <div class="field" style="margin-top: 12px">
              <label>Notes</label>
              <textarea
                rows={3}
                value={field('notes') ?? ''}
                onInput={(e) => setField('notes', e.currentTarget.value || null)}
                onBlur={commit}
              />
            </div>

            <Show when={field('closing_costs') != null}>
              <p class="screen-hint">
                Cash to close is the borrower's number — costs entered here are
                the file's working figure, not a CD replacement.
              </p>
            </Show>
            <Show when={error()}>
              <div class="form-error">{error()}</div>
            </Show>
          </Show>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="closing"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'closing')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="closing" demo={props.demo} />
    </div>
  );
}
