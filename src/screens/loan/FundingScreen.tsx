import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanDetail, LoanFunding } from '../../types';
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

const METHODS = ['wire', 'check', 'ach'];

// Funding (P5-07/08): the money moved. A funded date here is what
// turns the tracker's last stage green and completes the pipeline.
export function FundingScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId, screen: 'funding' }),
  );
  const [edits, setEdits] = createSignal<Partial<LoanFunding>>({});
  const [status, setStatus] = createSignal<'saving' | 'saved' | 'error' | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const funding = (): LoanFunding | null => {
    const e = edits();
    const base = detail()?.funding ?? null;
    return base ? { ...base, ...e } : null;
  };

  const field = <K extends keyof LoanFunding>(f: K): LoanFunding[K] => {
    const f0 = funding();
    return (f0?.[f] ?? null) as LoanFunding[K];
  };

  const setField = <K extends keyof LoanFunding>(f: K, v: LoanFunding[K]) => {
    setEdits((e) => ({ ...e, [f]: v }));
  };

  const commit = async () => {
    if (!hasEdits() || props.demo) return;
    setStatus('saving');
    setError(null);
    try {
      await invoke('save_funding', {
        loanId: props.loanId,
        funding: {
          funded_date: field('funded_date'),
          method: field('method'),
          reference: field('reference'),
          amount: field('amount'),
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

  const textInput = (f: keyof LoanFunding, extra?: Record<string, unknown>) => (
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
      <PizzaTracker current="funding" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        lock={detail()?.lock ?? null}
        loanId={props.loanId}
        demo={props.demo}
        work={
          <Show when={detail()} fallback={<div class="screen-loading">Loading…</div>}>
            <div class="screen-title-row">
              <h2 class="screen-title">Funding</h2>
              <span class={`save-status ${status() ?? ''}`}>
                {status() === 'saving' ? 'Saving…' : status() === 'saved' ? 'Saved ✓' : null}
              </span>
            </div>
            <Show when={props.demo}>
              <div class="demo-note">Demo mode — edits stay on this screen and are not persisted.</div>
            </Show>

            <Show
              when={field('funded_date')}
              fallback={<div class="demo-note">No disbursement on record yet — funding completes the pipeline.</div>}
            >
              <p class="screen-hint" style="margin-top: 0">
                Funded {field('funded_date')} — move the pipeline stepper on
                Overview to 'funded' to close the file out.
              </p>
            </Show>

            <div class="form-grid">
              <div class="field">
                <label>Funded date</label>
                <input
                  type="date"
                  value={field('funded_date') ?? ''}
                  onInput={(e) => setField('funded_date', e.currentTarget.value || null)}
                  onBlur={commit}
                />
              </div>
              <div class="field">
                <label>Method</label>
                <select
                  value={field('method') ?? ''}
                  onChange={(e) => {
                    setField('method', e.currentTarget.value || null);
                    commit();
                  }}
                >
                  <option value="">Not entered</option>
                  <For each={METHODS}>{(m) => <option value={m}>{m}</option>}</For>
                </select>
              </div>
              <div class="field">
                <label>Reference</label>
                {textInput('reference', { placeholder: 'Wire confirmation, check no…' })}
              </div>
              <div class="field">
                <label>Amount</label>
                <input
                  type="number"
                  min="0"
                  value={field('amount') ?? ''}
                  onInput={(e) => {
                    const v = e.currentTarget.valueAsNumber;
                    setField('amount', Number.isNaN(v) ? null : v);
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

            <Show when={error()}>
              <div class="form-error">{error()}</div>
            </Show>
          </Show>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="funding"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'funding')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="funding" demo={props.demo} />
    </div>
  );
}
