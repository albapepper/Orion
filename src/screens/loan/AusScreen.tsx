import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { AusMessage, AusSubmission, LoanDetail } from '../../types';
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

const timeOf = (iso: string) =>
  new Date(iso).toLocaleString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit',
  });

// The recommendation badge color tells the story at a glance. DU and
// LPA speak different dialects — DU: Approve/Refer, LPA: Accept/Caution.
const recClass = (rec: string | null) => {
  switch (rec) {
    case 'Approve/Eligible':
    case 'Accept':
      return 'aus-rec-approve';
    case 'Refer':
      return 'aus-rec-refer';
    case 'Refer with Caution':
    case 'Caution':
    case 'Unable to Determine':
      return 'aus-rec-caution';
    default:
      return 'aus-rec-unknown';
  }
};

const sevClass = (s: string) => `aus-sev-${s}`;

// AUS: submit the serialized loan to Desktop Underwriter and read the
// findings here. Mock mode (no DU endpoint configured) scores the
// file's own numbers so the workflow is demonstrable end to end.
export function AusScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId, screen: 'aus' }),
  );

  // Demo submissions can't persist — newly generated ones stack here.
  const [localSubs, setLocalSubs] = createSignal<AusSubmission[] | null>(null);
  const [submitting, setSubmitting] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);

  const submissions = () => localSubs() ?? (detail()?.aus ?? []);

  const submit = async (engine: 'du' | 'lpa') => {
    if (submitting()) return;
    setSubmitting(true);
    setError(null);
    try {
      const sub = await invoke<AusSubmission>(
        engine === 'du' ? 'submit_du' : 'submit_lpa',
        { loanId: props.loanId },
      );
      if (props.demo) {
        setLocalSubs([sub, ...submissions()]);
      } else {
        refetch();
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setSubmitting(false);
    }
  };

  const messagesOf = (s: AusSubmission): AusMessage[] => s.messages ?? [];

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="aus" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        lock={detail()?.lock ?? null}
        loanId={props.loanId}
        demo={props.demo}
        work={
          <>
            <div class="screen-title-row">
              <h2 class="screen-title">Automated Underwriting</h2>
              <span class="save-status">{submissions().length} submissions</span>
            </div>
            <Show when={props.demo}>
              <div class="demo-note">Demo mode — new submissions stack here and are not persisted.</div>
            </Show>

            <div class="aus-actions">
              <button class="ghost" disabled={submitting()} onClick={() => submit('du')}>
                {submitting() ? 'Submitting…' : 'Submit to DU'}
              </button>
              <button class="ghost" disabled={submitting()} onClick={() => submit('lpa')}>
                {submitting() ? 'Submitting…' : 'Submit to LPA'}
              </button>
              <Show
                when={!props.demo}
                fallback={<span class="aus-mode-badge">MOCK ENGINE</span>}
              >
                <span class="aus-mode-note">
                  Uses the configured DU / LPA sandbox endpoints.
                </span>
              </Show>
            </div>
            <Show when={error()}>
              <div class="task-error">{error()}</div>
            </Show>

            <Show
              when={submissions().length}
              fallback={
                <div class="demo-note">
                  No submissions yet. The file is sent as MISMO 3.4 — price it first so the
                  engine has terms to evaluate.
                </div>
              }
            >
              <For each={submissions()}>
                {(s) => (
                  <div class="aus-card">
                    <div class="aus-card-header">
                      <span class="aus-system">{s.system.toUpperCase()}</span>
                      <span class={`aus-rec ${recClass(s.recommendation)}`}>
                        {s.recommendation ?? 'No recommendation parsed'}
                      </span>
                      <span class="aus-when">{timeOf(s.submitted_at)}</span>
                    </div>
                    <Show when={s.case_id}>
                      <div class="aus-case mono">Case ID {s.case_id}</div>
                    </Show>
                    <ul class="aus-findings">
                      <For each={messagesOf(s)}>
                        {(m) => (
                          <li class={sevClass(m.severity)}>
                            <span class="aus-sev-dot" />
                            {m.text}
                          </li>
                        )}
                      </For>
                    </ul>
                    <Show when={s.raw_response}>
                      <details class="mismo-preview-wrap">
                        <summary>Raw response</summary>
                        <pre class="mismo-preview mono">{s.raw_response}</pre>
                      </details>
                    </Show>
                  </div>
                )}
              </For>
            </Show>
          </>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="aus"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'aus')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="aus" demo={props.demo} />
    </div>
  );
}
