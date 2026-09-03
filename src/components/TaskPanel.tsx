import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { Task } from '../types';

interface Props {
  loanId: string;
  screen: string;
  demo?: boolean;
}

const isOverdue = (t: Task) =>
  t.status !== 'done' && t.due_date !== null && new Date(t.due_date) < new Date();

const dueLabel = (t: Task) => {
  if (!t.due_date) return null;
  const days = Math.round(
    (new Date(t.due_date).getTime() - Date.now()) / 86_400_000,
  );
  if (t.status === 'done') return null;
  if (days < 0) return `overdue ${-days}d`;
  if (days === 0) return 'due today';
  return `due ${days}d`;
};

// Per-screen quick capture: add a task, tick it off. Managing owners and
// triage across every loan lives in the unified Tasks view (⌃k).
export function TaskPanel(props: Props) {
  const [collapsed, setCollapsed] = createSignal(false);
  const [newTitle, setNewTitle] = createSignal('');
  const [newDue, setNewDue] = createSignal('');
  const [newPriority, setNewPriority] = createSignal('normal');
  const [error, setError] = createSignal<string | null>(null);
  const [localTasks, setLocalTasks] = createSignal<Task[] | null>(null);
  const [tasksRes, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<Task[]>('list_tasks', { loanId }),
  );

  const tasks = () => localTasks() ?? (tasksRes() ?? []);

  const addTask = async () => {
    const title = newTitle().trim();
    if (!title) return;
    if (props.demo) {
      setLocalTasks([
        ...(localTasks() ?? tasks()),
        {
          id: crypto.randomUUID(),
          loan_id: props.loanId,
          loan_number: '',
          title,
          description: null,
          assigned_to: null,
          assigned_to_name: null,
          screen: props.screen,
          status: 'open',
          priority: newPriority() as Task['priority'],
          due_date: newDue() ? new Date(`${newDue()}T00:00:00Z`).toISOString() : null,
          created_at: new Date().toISOString(),
          completed_at: null,
        },
      ]);
      setNewTitle('');
      setNewDue('');
      setNewPriority('normal');
      return;
    }
    try {
      await invoke('create_task', {
        loanId: props.loanId,
        title,
        screen: props.screen,
        dueDate: newDue() || null,
        priority: newPriority(),
      });
      setNewTitle('');
      setNewDue('');
      setNewPriority('normal');
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const toggle = async (task: Task) => {
    if (props.demo) {
      setLocalTasks(
        (localTasks() ?? tasks()).map((t) =>
          t.id === task.id
            ? {
                ...t,
                status: t.status === 'done' ? 'open' : 'done',
                completed_at: t.status === 'done' ? null : new Date().toISOString(),
              }
            : t,
        ),
      );
      return;
    }
    try {
      await invoke('set_task_status', {
        taskId: task.id,
        status: task.status === 'done' ? 'open' : 'done',
      });
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const open = () => tasks().filter((t) => t.status !== 'done').length;

  return (
    <section class="task-panel" classList={{ collapsed: collapsed() }}>
      <button class="task-header" onClick={() => setCollapsed(!collapsed())}>
        <span>Tasks</span>
        <span class="task-count">{open()} open</span>
        <span class="chevron">{collapsed() ? '▸' : '▾'}</span>
      </button>

      <Show when={!collapsed()}>
        <div class="task-list">
          <For
            each={tasks()}
            fallback={<div class="task-empty">No outstanding items on this loan.</div>}
          >
            {(task) => (
              <label class="task-item" classList={{ done: task.status === 'done' }}>
                <input
                  type="checkbox"
                  checked={task.status === 'done'}
                  onChange={() => toggle(task)}
                />
                <span class="task-body">
                  <span class="task-title">{task.title}</span>
                  <span class="task-chips">
                    <Show when={task.priority === 'high'}>
                      <span class="task-chip priority">high</span>
                    </Show>
                    <Show when={dueLabel(task)}>
                      <span class="task-chip" classList={{ overdue: isOverdue(task) }}>
                        {dueLabel(task)}
                      </span>
                    </Show>
                    <Show when={task.assigned_to_name}>
                      <span class="task-chip">{task.assigned_to_name}</span>
                    </Show>
                    <Show when={task.screen}>
                      <span class="task-chip">{task.screen}</span>
                    </Show>
                  </span>
                </span>
              </label>
            )}
          </For>
        </div>
        <div class="task-add">
          <input
            type="text"
            placeholder="Add a task…"
            value={newTitle()}
            onInput={(e) => setNewTitle(e.currentTarget.value)}
            onKeyDown={(e) => e.key === 'Enter' && addTask()}
          />
          <input
            type="date"
            class="task-add-date"
            title="Due date"
            value={newDue()}
            onInput={(e) => setNewDue(e.currentTarget.value)}
          />
          <select
            class="task-add-priority"
            title="Priority"
            value={newPriority()}
            onChange={(e) => setNewPriority(e.currentTarget.value)}
          >
            <option value="low">Low</option>
            <option value="normal">Normal</option>
            <option value="high">High</option>
          </select>
          <button onClick={addTask}>Add</button>
        </div>
        <Show when={error()}>
          <div class="task-error">{error()}</div>
        </Show>
      </Show>
    </section>
  );
}
