import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { AppStatus, UserAccount } from '../types';
import { ROLE_LABELS, setUser, user } from '../lib/session';

// App-level settings: what Orion loaded, read straight from the core,
// plus the persona picker that drives dynamic field locking.
export function SettingsScreen() {
  const [status] = createResource(() => invoke<AppStatus>('get_status'));
  const [users, { refetch: refetchUsers }] = createResource(() =>
    invoke<UserAccount[]>('list_users'),
  );
  const [switchError, setSwitchError] = createSignal<string | null>(null);

  const rows = () => {
    const s = status();
    if (!s) return [];
    return [
      {
        key: 'Database',
        value: s.db_connected ? 'Connected' : 'Demo mode — not connected',
        ok: s.db_connected,
      },
      { key: 'Assistant provider', value: s.ai_provider, ok: true },
      { key: 'Assistant model', value: s.ai_model, ok: true },
      { key: 'Configured', value: s.configured ? 'Yes' : 'Not yet', ok: s.configured },
    ];
  };

  const switchTo = async (id: string) => {
    setSwitchError(null);
    try {
      const account = await invoke<UserAccount>('sign_in_as', { userId: id });
      setUser(account);
      refetchUsers();
    } catch (err) {
      setSwitchError(String(err));
    }
  };

  return (
    <div class="screen">
      <div class="screen-title-row">
        <h2 class="screen-title">Settings</h2>
      </div>

      <Show when={status()} fallback={<div class="screen-loading">Loading…</div>}>
        <div class="settings-list">
          <For each={rows()}>
            {(row) => (
              <div class="settings-row">
                <span class="settings-key">{row.key}</span>
                <span class="settings-val" classList={{ warn: !row.ok, ok: row.ok }}>
                  {row.value}
                </span>
              </div>
            )}
          </For>
        </div>
        <p class="screen-hint">
          Database and assistant settings are managed in Orion's configuration;
          this panel reflects what the app loaded at startup.
        </p>

        <div class="screen-title-row" style="margin-top: 32px">
          <h2 class="screen-title">Working as</h2>
        </div>
        <p class="screen-hint" style="margin: 0 0 12px">
          Your persona decides which fields are editable — underwriter notes
          and verifications unlock only for underwriters.
        </p>
        <div class="settings-list">
          <Show when={users()} fallback={<div class="screen-loading">Loading…</div>}>
            <For each={users()}>
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
          </Show>
        </div>
        <Show when={switchError()}>
          <div class="form-error">{switchError()}</div>
        </Show>
      </Show>
    </div>
  );
}
