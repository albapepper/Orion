import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { Condition, ConditionStatus, LoanDetail } from '../../types';
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

const STATUSES: ConditionStatus[] = ['open', 'submitted', 'cleared', 'waived'];

const label = (s: string) =>
  s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());

const isOutstanding = (c: Condition) => c.status === 'open' || c.status === 'submitted';

const isOverdue = (c: Condition) =>
  isOutstanding(c) && c.clear_by !== null && new Date(c.clear_by) < new Date();

const dueLabel = (c: Condition) => {
  if (!c.clear_by) return null;
  const days = Math.round(
    (new Date(c.clear_by).getTime() - Date.now()) / 86_400_000,
  );
  if (c.status === 'cleared' || c.status === 'waived') return null;
  if (days < 0) return `overdue ${-days}d`;
  if (days === 0) return 'due today';
  return `due ${days}d`;
};

// Underwriting's checklist for this loan. Outstanding conditions are the
// blockers — they drive the red ring on the tracker and what the AI
// points at first. The status flow: open → submitted → cleared (or waived).
export function ConditionsScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId, screen: 'conditions' }),
  );
  const [title, setTitle] = createSignal('');
  const [detailText, setDetailText] = createSignal('');
  const [source, setSource] = createSignal<'underwriter' | 'processor'>('underwriter');
  const [priority, setPriority] = createSignal<'low' | 'normal' | 'high'>('normal');
  const [clearBy, setClearBy] = createSignal('');
  const [error, setError] = createSignal<string | null>(null);
  const [localConditions, setLocalConditions] = createSignal<Condition[] | null>(null);

  const conditions = () => localConditions() ?? (detail()?.conditions ?? []);
  const outstanding = () => conditions().filter(isOutstanding).length;

  const resetForm = () => {
    setTitle('');
    setDetailText('');
    setPriority('normal');
    setClearBy('');
  };

  const addCondition = async () => {
    const t = title().trim();
    if (!t) return;
    if (props.demo) {
      setLocalConditions([
        {
          id: crypto.randomUUID(),
          loan_id: props.loanId,
          company_id: '',
          title: t,
          detail: detailText() || null,
          source: source(),
          status: 'open',
          priority: priority(),
          clear_by: clearBy() ? new Date(`${clearBy()}T00:00:00Z`).toISOString() : null,
          cleared_at: null,
          cleared_by: null,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
          cleared_by_name: null,
        },
        ...(localConditions() ?? conditions()),
      ]);
      resetForm();
      return;
    }
    try {
      await invoke('create_condition', {
        loanId: props.loanId,
        title: t,
        detail: detailText() || null,
        source: source(),
        priority: priority(),
        clearBy: clearBy() || null,
      });
      resetForm();
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const setStatus = async (condition: Condition, status: ConditionStatus) => {
    if (props.demo) {
      setLocalConditions(
        (localConditions() ?? conditions()).map((c) =>
          c.id === condition.id
            ? {
                ...c,
                status,
                cleared_at:
                  status === 'cleared' || status === 'waived'
                    ? new Date().toISOString()
                    : null,
              }
            : c,
        ),
      );
      return;
    }
    try {
      await invoke('set_condition_status', { conditionId: condition.id, status });
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const removeCondition = async (condition: Condition) => {
    if (props.demo) {
      setLocalConditions(
        (localConditions() ?? conditions()).filter((c) => c.id !== condition.id),
      );
      return;
    }
    try {
      await invoke('remove_condition', { conditionId: condition.id });
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="conditions" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        lock={detail()?.lock ?? null}
        loanId={props.loanId}
        demo={props.demo}
        work={
          <>
            <div class="screen-title-row">
              <h2 class="screen-title">Underwriting conditions</h2>
              <span class="save-status">
                {outstanding()} outstanding of {conditions().length}
              </span>
            </div>
            <Show when={props.demo}>
              <div class="demo-note">Demo mode — edits stay on this screen and are not persisted.</div>
            </Show>

            <div class="cond-add">
              <input
                type="text"
                placeholder="Condition title…"
                value={title()}
                onInput={(e) => setTitle(e.currentTarget.value)}
                onKeyDown={(e) => e.key === 'Enter' && addCondition()}
              />
              <input
                type="text"
                placeholder="Detail (optional)…"
                value={detailText()}
                onInput={(e) => setDetailText(e.currentTarget.value)}
                onKeyDown={(e) => e.key === 'Enter' && addCondition()}
              />
              <select
                value={source()}
                onChange={(e) => setSource(e.currentTarget.value as 'underwriter' | 'processor')}
                title="Requested by"
              >
                <option value="underwriter">Underwriter</option>
                <option value="processor">Processor</option>
              </select>
              <select
                value={priority()}
                onChange={(e) => setPriority(e.currentTarget.value as 'low' | 'normal' | 'high')}
                title="Priority"
              >
                <option value="low">Low</option>
                <option value="normal">Normal</option>
                <option value="high">High</option>
              </select>
              <input
                type="date"
                title="Clear by"
                value={clearBy()}
                onInput={(e) => setClearBy(e.currentTarget.value)}
              />
              <button onClick={addCondition}>Add</button>
            </div>

            <table class="data-table">
              <thead>
                <tr>
                  <th>Condition</th>
                  <th>Status</th>
                  <th>Priority</th>
                  <th>Clear by</th>
                  <th>Cleared</th>
                  <th />
                </tr>
              </thead>
              <tbody>
                <For
                  each={conditions()}
                  fallback={
                    <tr>
                      <td colspan={6}>No conditions — underwriting hasn't listed any.</td>
                    </tr>
                  }
                >
                  {(cond) => (
                    <tr classList={{ 'cond-done-row': !isOutstanding(cond) }}>
                      <td>
                        <span class="cond-title">{cond.title}</span>
                        <Show when={cond.detail}>
                          <span class="cond-detail">{cond.detail}</span>
                        </Show>
                      </td>
                      <td>
                        <select
                          value={cond.status}
                          classList={{ 'cond-overdue': isOverdue(cond) }}
                          onChange={(e) => setStatus(cond, e.currentTarget.value as ConditionStatus)}
                        >
                          <For each={STATUSES}>{(s) => <option value={s}>{label(s)}</option>}</For>
                        </select>
                      </td>
                      <td>
                        <Show
                          when={cond.priority !== 'normal'}
                          fallback={<span class="loans-none">—</span>}
                        >
                          <span
                            class="task-chip"
                            classList={{ priority: cond.priority === 'high' }}
                          >
                            {cond.priority}
                          </span>
                        </Show>
                      </td>
                      <td>
                        <Show when={dueLabel(cond)} fallback={<span class="loans-none">—</span>}>
                          <span class="task-chip" classList={{ overdue: isOverdue(cond) }}>
                            {dueLabel(cond)}
                          </span>
                        </Show>
                      </td>
                      <td>
                        <Show
                          when={cond.cleared_by_name}
                          fallback={<span class="loans-none">—</span>}
                        >
                          {cond.cleared_by_name}
                        </Show>
                      </td>
                      <td>
                        <button
                          class="ghost small"
                          title="Remove condition"
                          onClick={() => removeCondition(cond)}
                        >
                          ✕
                        </button>
                      </td>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
            <Show when={error()}>
              <div class="task-error">{error()}</div>
            </Show>
          </>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="conditions"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'conditions')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="conditions" demo={props.demo} />
    </div>
  );
}
