import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { Task } from '../types';

interface Props {
  loanId: string;
  screen: string;
}

export function TaskPanel(props: Props) {
  const [collapsed, setCollapsed] = createSignal(false);
  const [newTitle, setNewTitle] = createSignal('');
  const [error, setError] = createSignal<string | null>(null);
  const [tasks, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<Task[]>('list_tasks', { loanId }),
  );

  const addTask = async () => {
    const title = newTitle().trim();
    if (!title) return;
    try {
      await invoke('create_task', {
        loanId: props.loanId,
        title,
        screen: props.screen,
      });
      setNewTitle('');
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const toggle = async (task: Task) => {
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

  const open = () => (tasks() ?? []).filter((t) => t.status !== 'done').length;

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
            each={tasks() ?? []}
            fallback={<div class="task-empty">No outstanding items on this loan.</div>}
          >
            {(task) => (
              <label class="task-item" classList={{ done: task.status === 'done' }}>
                <input
                  type="checkbox"
                  checked={task.status === 'done'}
                  onChange={() => toggle(task)}
                />
                <span class="task-title">{task.title}</span>
                <Show when={task.screen}>
                  <span class="task-screen">{task.screen}</span>
                </Show>
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
          <button onClick={addTask}>Add</button>
        </div>
        <Show when={error()}>
          <div class="task-error">{error()}</div>
        </Show>
      </Show>
    </section>
  );
}
