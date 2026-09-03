import { createMemo, createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { Task, UserAccount } from '../types';
import { user } from '../lib/session';

interface Props {
  demo?: boolean;
  onOpenLoanScreen: (loanId: string, screen: string | null) => void;
}

// The unified task view across every loan in the shop — triage lives
// here: tick things off, put an owner on them, jump to the loan screen
// that spawned the task.
export function TasksScreen(props: Props) {
  const [scope, setScope] = createSignal<'mine' | 'all'>('all');
  const [statusFilter, setStatusFilter] = createSignal<'open' | 'done' | 'all'>('open');
  const [localTasks, setLocalTasks] = createSignal<Task[] | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const [tasksRes, { refetch }] = createResource(() =>
    invoke<Task[]>('list_all_tasks'),
  );
  const [users] = createResource(() => invoke<UserAccount[]>('list_users'));

  const tasks = () => localTasks() ?? (tasksRes() ?? []);

  const filtered = createMemo(() => {
    const me = user();
    return tasks().filter((t) => {
      if (scope() === 'mine' && (!me || t.assigned_to !== me.id)) return false;
      if (statusFilter() === 'open' && t.status === 'done') return false;
      if (statusFilter() === 'done' && t.status !== 'done') return false;
      return true;
    });
  });

  const isOverdue = (t: Task) =>
    t.status !== 'done' && t.due_date !== null && new Date(t.due_date) < new Date();

  const dueLabel = (t: Task) => {
    if (!t.due_date) return null;
    const days = Math.round(
      (new Date(t.due_date).getTime() - Date.now()) / 86_400_000,
    );
    if (days < 0) return `overdue ${-days}d`;
    if (days === 0) return 'due today';
    return `due ${days}d`;
  };

  const setLocal = (id: string, patch: Partial<Task>) => {
    setLocalTasks(
      (localTasks() ?? tasks()).map((t) => (t.id === id ? { ...t, ...patch } : t)),
    );
  };

  const toggle = async (task: Task) => {
    const next = task.status === 'done' ? 'open' : 'done';
    if (props.demo) {
      setLocal(task.id, {
        status: next,
        completed_at: next === 'done' ? new Date().toISOString() : null,
      });
      return;
    }
    try {
      await invoke('set_task_status', { taskId: task.id, status: next });
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const reassign = async (task: Task, userId: string) => {
    if (props.demo) {
      const u = (users() ?? []).find((x) => x.id === userId);
      setLocal(task.id, {
        assigned_to: userId || null,
        assigned_to_name: u?.name ?? null,
      });
      return;
    }
    try {
      await invoke('assign_task', { taskId: task.id, assignedTo: userId || null });
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <div class="screen">
      <div class="screen-title-row">
        <h2 class="screen-title">Tasks</h2>
        <span class="save-status">{filtered().length} shown</span>
      </div>

      <div class="loans-toolbar">
        <select
          value={scope()}
          onChange={(e) => setScope(e.currentTarget.value as 'mine' | 'all')}
          title="Whose tasks"
        >
          <option value="all">Everyone's tasks</option>
          <option value="mine">My tasks</option>
        </select>
        <select
          value={statusFilter()}
          onChange={(e) =>
            setStatusFilter(e.currentTarget.value as 'open' | 'done' | 'all')
          }
          title="Status"
        >
          <option value="open">Open</option>
          <option value="done">Done</option>
          <option value="all">All</option>
        </select>
      </div>

      <Show
        when={filtered().length > 0}
        fallback={<div class="screen-loading">No tasks here.</div>}
      >
        <table class="data-table tasks-table">
          <thead>
            <tr>
              <th />
              <th>Task</th>
              <th>Loan</th>
              <th>Priority</th>
              <th>Due</th>
              <th>Owner</th>
            </tr>
          </thead>
          <tbody>
            <For each={filtered()}>
              {(task) => (
                <tr classList={{ 'task-done-row': task.status === 'done' }}>
                  <td>
                    <input
                      type="checkbox"
                      checked={task.status === 'done'}
                      onChange={() => toggle(task)}
                      title={task.status === 'done' ? 'Reopen' : 'Complete'}
                    />
                  </td>
                  <td>
                    <span class="task-title">{task.title}</span>
                    <Show when={task.screen}>
                      <span class="task-chip">{task.screen}</span>
                    </Show>
                  </td>
                  <td>
                    <button
                      class="link"
                      onClick={() => props.onOpenLoanScreen(task.loan_id, task.screen)}
                    >
                      <span class="mono">{task.loan_number}</span>
                    </button>
                  </td>
                  <td>
                    <Show
                      when={task.priority !== 'normal'}
                      fallback={<span class="loans-none">—</span>}
                    >
                      <span
                        class="task-chip"
                        classList={{ priority: task.priority === 'high' }}
                      >
                        {task.priority}
                      </span>
                    </Show>
                  </td>
                  <td>
                    <Show when={dueLabel(task)} fallback={<span class="loans-none">—</span>}>
                      <span class="task-chip" classList={{ overdue: isOverdue(task) }}>
                        {dueLabel(task)}
                      </span>
                    </Show>
                  </td>
                  <td>
                    <select
                      value={task.assigned_to ?? ''}
                      onChange={(e) => reassign(task, e.currentTarget.value)}
                      title="Assign to"
                    >
                      <option value="">Unassigned</option>
                      <For each={users() ?? []}>
                        {(u) => <option value={u.id}>{u.name}</option>}
                      </For>
                    </select>
                  </td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </Show>
      <Show when={error()}>
        <div class="task-error">{error()}</div>
      </Show>
    </div>
  );
}
