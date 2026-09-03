import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { UserAccount } from '../types';
import { ROLE_LABELS, setUser, user } from '../lib/session';

interface Props {
  demo: boolean;
}

const ROLES = ['loan_officer', 'processor', 'underwriter', 'admin'];

// Team management (P5-03): the roster, its roles, and the persona
// picker that used to live on Settings. Editing is admin-only on the
// backend — everyone can look, admins can touch.
export function UsersScreen(props: Props) {
  const [users, { refetch }] = createResource(() => invoke<UserAccount[]>('list_users'));
  const [error, setError] = createSignal<string | null>(null);

  // ── Create ────────────────────────────────────────────────────────
  const [newName, setNewName] = createSignal('');
  const [newEmail, setNewEmail] = createSignal('');
  const [newRole, setNewRole] = createSignal('loan_officer');
  const [creating, setCreating] = createSignal(false);

  const createUser = async () => {
    setCreating(true);
    setError(null);
    try {
      await invoke('create_user', {
        email: newEmail(),
        name: newName(),
        role: newRole(),
        phone: null,
        title: null,
      });
      setNewName('');
      setNewEmail('');
      setNewRole('loan_officer');
      refetch();
    } catch (e) {
      setError(String(e));
    } finally {
      setCreating(false);
    }
  };

  // ── Per-row edits ─────────────────────────────────────────────────
  const [edits, setEdits] = createSignal<Record<string, Partial<UserAccount>>>({});
  const [rowBusy, setRowBusy] = createSignal<string | null>(null);

  const editValue = (u: UserAccount, key: keyof UserAccount): UserAccount[typeof key] => {
    const e = edits()[u.id];
    return (e && key in e ? e[key] : u[key]) as UserAccount[typeof key];
  };

  const saveRow = async (u: UserAccount) => {
    const patch = edits()[u.id];
    if (!patch || props.demo) return;
    setRowBusy(u.id);
    setError(null);
    try {
      await invoke('update_user', {
        userId: u.id,
        name: editValue(u, 'name'),
        role: editValue(u, 'role'),
        phone: editValue(u, 'phone'),
        title: editValue(u, 'title'),
      });
      setEdits((e) => {
        const next = { ...e };
        delete next[u.id];
        return next;
      });
      refetch();
    } catch (e) {
      setError(String(e));
    } finally {
      setRowBusy(null);
    }
  };

  const toggleActive = async (u: UserAccount) => {
    setError(null);
    try {
      await invoke('set_user_active', { userId: u.id, active: !u.active });
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const switchTo = async (id: string) => {
    setError(null);
    try {
      const account = await invoke<UserAccount>('sign_in_as', { userId: id });
      setUser(account);
    } catch (e) {
      setError(String(e));
    }
  };

  const textInput = (u: UserAccount, key: 'name' | 'phone' | 'title') => (
    <input
      type="text"
      value={(editValue(u, key) as string) ?? ''}
      onInput={(e) =>
        setEdits((ed) => ({ ...ed, [u.id]: { ...ed[u.id], [key]: e.currentTarget.value || null } }))
      }
      onBlur={() => saveRow(u)}
      onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
    />
  );

  return (
    <div class="screen">
      <div class="screen-title-row">
        <h2 class="screen-title">Team</h2>
        <span class="save-status">{users()?.length ?? '…'} members</span>
      </div>
      <Show when={props.demo}>
        <div class="demo-note">
          Demo mode — the roster below is the demo cast. Team changes need a
          configured database.
        </div>
      </Show>

      <Show when={users()} fallback={<div class="screen-loading">Loading…</div>}>
        <table class="data-table editable">
          <thead>
            <tr>
              <th>Name</th>
              <th>Email</th>
              <th>Role</th>
              <th>Phone</th>
              <th>Title</th>
              <th>Status</th>
              <th />
            </tr>
          </thead>
          <tbody>
            <For each={users() ?? []}>
              {(u) => (
                <tr classList={{ 'user-inactive': !u.active }}>
                  <td>{textInput(u, 'name')}</td>
                  <td class="mono">{u.email}</td>
                  <td>
                    <select
                      value={editValue(u, 'role') as string}
                      onChange={(e) => {
                        setEdits((ed) => ({ ...ed, [u.id]: { ...ed[u.id], role: e.currentTarget.value as UserAccount['role'] } }));
                        saveRow(u);
                      }}
                    >
                      <For each={ROLES}>
                        {(r) => <option value={r}>{ROLE_LABELS[r] ?? r}</option>}
                      </For>
                    </select>
                  </td>
                  <td>{textInput(u, 'phone')}</td>
                  <td>{textInput(u, 'title')}</td>
                  <td>
                    <Show
                      when={u.active}
                      fallback={<span class="loans-none">deactivated</span>}
                    >
                      <span class="settings-val ok">active</span>
                    </Show>
                  </td>
                  <td class="row-status-cell">
                    <Show when={rowBusy() === u.id}>
                      <span class="row-status saving">Saving…</span>
                    </Show>
                    <button
                      class="ghost small"
                      title={u.active ? 'Deactivate user' : 'Reactivate user'}
                      onClick={() => toggleActive(u)}
                    >
                      {u.active ? '⏸' : '▶'}
                    </button>
                  </td>
                </tr>
              )}
            </For>
          </tbody>
        </table>

        <div class="screen-title-row" style="margin-top: 30px">
          <h2 class="screen-title">Add a teammate</h2>
        </div>
        <div class="user-create">
          <input
            type="text"
            placeholder="Full name"
            value={newName()}
            onInput={(e) => setNewName(e.currentTarget.value)}
          />
          <input
            type="text"
            placeholder="email@company.com"
            class="mono"
            value={newEmail()}
            onInput={(e) => setNewEmail(e.currentTarget.value)}
          />
          <select value={newRole()} onChange={(e) => setNewRole(e.currentTarget.value)}>
            <For each={ROLES}>
              {(r) => <option value={r}>{ROLE_LABELS[r] ?? r}</option>}
            </For>
          </select>
          <button class="ghost" disabled={creating() || !newName() || !newEmail()} onClick={createUser}>
            + Add user
          </button>
        </div>

        <div class="screen-title-row" style="margin-top: 30px">
          <h2 class="screen-title">Working as</h2>
        </div>
        <p class="screen-hint" style="margin: 0 0 12px">
          Your persona decides which fields are editable — underwriter notes
          and verifications unlock only for underwriters.
        </p>
        <div class="settings-list">
          <For each={(users() ?? []).filter((u) => u.active)}>
            {(account) => (
              <button
                type="button"
                class="settings-row settings-user"
                classList={{ current: user()?.id === account.id }}
                onClick={() => switchTo(account.id)}
              >
                <span class="settings-key">
                  <span class="identity-avatar">{account.name.charAt(0)}</span>
                  {account.name}
                  <span class="settings-role">{ROLE_LABELS[account.role] ?? account.role}</span>
                </span>
                <Show
                  when={user()?.id === account.id}
                  fallback={<span class="settings-val">Switch to</span>}
                >
                  <span class="settings-val ok">Signed in ✓</span>
                </Show>
              </button>
            )}
          </For>
        </div>

        <Show when={error()}>
          <div class="form-error">{error()}</div>
        </Show>
      </Show>
    </div>
  );
}
