import { createSignal, For, onMount, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { AiResponse, LoanSummary } from '../types';
import { LOAN_STATUSES } from '../types';

interface Props {
  loans: LoanSummary[];
  onOpenLoan: (loan: LoanSummary) => void;
}

// How urgently each stage needs a person's attention. Higher = sooner.
const STATUS_URGENCY: Record<string, number> = {
  closing: 6,
  approval: 5,
  underwriting: 4,
  processing: 3,
  application: 2,
  funded: 0,
};

const daysSince = (iso: string) =>
  Math.floor((Date.now() - new Date(iso).getTime()) / 86_400_000);

const idleLabel = (iso: string) => {
  const d = daysSince(iso);
  if (d === 0) return 'touched today';
  if (d === 1) return 'idle 1 day';
  return `idle ${d} days`;
};

export function Dashboard(props: Props) {
  // The work queue is deterministic — flags first, then stage urgency,
  // then stalest first — so it agrees with the AI's priority order.
  const workQueue = () =>
    props.loans
      .filter((l) => l.status !== 'funded')
      .slice()
      .sort((a, b) => {
        if (a.flagged_reviews !== b.flagged_reviews)
          return b.flagged_reviews - a.flagged_reviews;
        const urgency =
          (STATUS_URGENCY[b.status] ?? 1) - (STATUS_URGENCY[a.status] ?? 1);
        if (urgency !== 0) return urgency;
        return new Date(a.updated_at).getTime() - new Date(b.updated_at).getTime();
      });

  const countFor = (status: string) =>
    props.loans.filter((l) => l.status === status).length;

  // ── AI triage: the assistant reads the pipeline and answers ──

  const [triage, setTriage] = createSignal<string | null>(null);
  const [triageLoading, setTriageLoading] = createSignal(false);
  const [triageError, setTriageError] = createSignal<string | null>(null);

  const loadTriage = async () => {
    if (triageLoading()) return;
    setTriageLoading(true);
    setTriageError(null);
    try {
      const response = await invoke<AiResponse>('ai_chat', {
        loanId: null,
        screen: 'dashboard',
        messages: [
          { role: 'user', content: 'What should I work on next?' },
        ],
      });
      setTriage(response.content);
    } catch (e) {
      setTriageError(String(e));
    } finally {
      setTriageLoading(false);
    }
  };

  onMount(loadTriage);

  return (
    <div class="screen dashboard">
      <div class="pipeline-strip">
        <For each={[...LOAN_STATUSES]}>
          {(status) => (
            <div class="pipeline-stat" classList={{ empty: countFor(status) === 0 }}>
              <span class="pipeline-count">{countFor(status)}</span>
              <span class="pipeline-label">{status}</span>
            </div>
          )}
        </For>
      </div>

      <div class="ai-summary">
        <div class="ai-summary-header">
          <span class="ai-summary-title">What should I work on next?</span>
          <button
            class="ghost small"
            onClick={loadTriage}
            disabled={triageLoading()}
            title="Re-run the pipeline triage"
          >
            {triageLoading() ? 'Reading pipeline…' : '↻ Refresh'}
          </button>
        </div>
        <Show
          when={!triageLoading()}
          fallback={<div class="ai-summary-body ai-summary-loading">Reading the pipeline…</div>}
        >
          <Show
            when={!triageError()}
            fallback={<div class="ai-summary-body ai-summary-error">{triageError()}</div>}
          >
            <Show
              when={triage()}
              fallback={
                <div class="ai-summary-body ai-summary-loading">Press refresh to ask the assistant.</div>
              }
            >
              <div class="ai-summary-body">{triage()}</div>
            </Show>
          </Show>
        </Show>
      </div>

      <h2 class="section-title">Up next</h2>
      <Show
        when={workQueue().length > 0}
        fallback={
          <div class="screen-loading">Nothing needs attention. Pipeline is clear.</div>
        }
      >
        <div class="queue">
          <For each={workQueue()}>
            {(loan, i) => (
              <button class="queue-card" onClick={() => props.onOpenLoan(loan)}>
                <span class="queue-rank mono">{i() + 1}</span>
                <span class="queue-who">
                  <span class="queue-borrower">{loan.borrower_name ?? 'No borrower yet'}</span>
                  <span class="queue-loan mono">{loan.loan_number}</span>
                </span>
                <span class="queue-meta">
                  <Show when={loan.flagged_reviews > 0}>
                    <span
                      class="queue-flag"
                      title={`${loan.flagged_reviews} flagged underwriting review(s)`}
                    >
                      ⚠ {loan.flagged_reviews} flagged
                    </span>
                  </Show>
                  <span class={`status-pill ${loan.status}`}>{loan.status}</span>
                  <span class="queue-idle">{idleLabel(loan.updated_at)}</span>
                  <span class="queue-assigned">
                    {loan.assigned_to_name ?? 'Unassigned'}
                  </span>
                </span>
              </button>
            )}
          </For>
        </div>
      </Show>
    </div>
  );
}
