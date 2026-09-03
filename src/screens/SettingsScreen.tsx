import { createResource, createSignal, For, onMount, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { AppStatus, Config } from '../types';

// App-level settings (P5-02): a real admin surface over the config —
// company identity, database, assistant, AUS endpoints — with the
// wizard's test buttons wired in. Team and persona live on ⌃m.
export function SettingsScreen() {
  const [status, { refetch }] = createResource(() => invoke<AppStatus>('get_status'));
  const [cfg, setCfg] = createSignal<Config | null>(null);
  const [error, setError] = createSignal<string | null>(null);
  const [saved, setSaved] = createSignal(false);
  const [busy, setBusy] = createSignal(false);
  const [companyName, setCompanyName] = createSignal('');
  const [dbVersion, setDbVersion] = createSignal<string | null>(null);
  const [aiReply, setAiReply] = createSignal<string | null>(null);

  onMount(async () => {
    try {
      setCfg(await invoke<Config>('get_config'));
      const s = await invoke<AppStatus>('get_status');
      setCompanyName(s.company_name ?? '');
    } catch (e) {
      setError(String(e));
    }
  });

  const flash = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 1800);
  };

  const run = async (fn: () => Promise<void>) => {
    setBusy(true);
    setError(null);
    try {
      await fn();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const renameCompany = () =>
    run(async () => {
      await invoke('rename_company', { name: companyName() });
      refetch();
      flash();
    });

  const testDb = () =>
    run(async () => {
      const v = await invoke<string>('test_db_connection', { url: cfg()!.database.url });
      setDbVersion(v);
    });

  const saveDb = () =>
    run(async () => {
      await invoke('save_config', { config: { ...cfg()!, database: { url: cfg()!.database.url } } });
      refetch();
      flash();
    });

  const saveAi = () =>
    run(async () => {
      await invoke('save_config', { config: { ...cfg()!, ai: cfg()!.ai } });
      refetch();
      flash();
    });

  const testAi = () =>
    run(async () => {
      const reply = await invoke<string>('test_ai_connection');
      setAiReply(reply.trim());
    });

  const saveAus = () =>
    run(async () => {
      await invoke('save_config', { config: { ...cfg()!, aus: cfg()!.aus } });
      refetch();
      flash();
    });

  return (
    <div class="screen">
      <div class="screen-title-row">
        <h2 class="screen-title">Settings</h2>
        <Show when={saved()}>
          <span class="save-status ok">Saved ✓</span>
        </Show>
      </div>

      <Show when={status()} fallback={<div class="screen-loading">Loading…</div>}>
        <div class="settings-list">
          <For
            each={[
              {
                key: 'Database',
                value: status()!.db_connected ? 'Connected' : 'Demo mode — not connected',
                ok: status()!.db_connected,
              },
              { key: 'Assistant', value: `${status()!.ai_provider} · ${status()!.ai_model}`, ok: true },
              {
                key: 'Company',
                value: status()!.company_name ?? 'Not set',
                ok: !!status()!.company_name,
              },
              { key: 'Configured', value: status()!.configured ? 'Yes' : 'Not yet', ok: status()!.configured },
            ]}
          >
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

        <Show when={cfg()}>
          <h2 class="section-title">Company</h2>
          <div class="user-create">
            <input
              type="text"
              value={companyName()}
              onInput={(e) => setCompanyName(e.currentTarget.value)}
            />
            <button
              class="ghost"
              disabled={busy() || !companyName().trim()}
              onClick={renameCompany}
            >
              Rename
            </button>
          </div>

          <h2 class="section-title">Database</h2>
          <div class="field">
            <label>Postgres URL</label>
            <input
              class="mono"
              style="width: 100%"
              value={cfg()!.database.url}
              onInput={(e) =>
                setCfg((c) => c && { ...c, database: { url: e.currentTarget.value } })
              }
            />
          </div>
          <Show when={dbVersion()}>
            <p class="wizard-ok">Connected: {dbVersion()}</p>
          </Show>
          <div class="wizard-actions">
            <button class="ghost small" disabled={busy()} onClick={testDb}>
              Test connection
            </button>
            <button class="ghost small" disabled={busy()} onClick={saveDb}>
              Save & reconnect
            </button>
          </div>

          <h2 class="section-title">Assistant</h2>
          <div class="form-grid">
            <div class="field">
              <label>Endpoint</label>
              <input
                class="mono"
                value={cfg()!.ai.endpoint}
                onInput={(e) =>
                  setCfg((c) => c && { ...c, ai: { ...c.ai, endpoint: e.currentTarget.value } })
                }
              />
            </div>
            <div class="field">
              <label>Model</label>
              <input
                class="mono"
                value={cfg()!.ai.model}
                onInput={(e) =>
                  setCfg((c) => c && { ...c, ai: { ...c.ai, model: e.currentTarget.value } })
                }
              />
            </div>
            <div class="field">
              <label>API key (cloud only)</label>
              <input
                type="password"
                class="mono"
                value={cfg()!.ai.api_key}
                onInput={(e) =>
                  setCfg((c) => c && { ...c, ai: { ...c.ai, api_key: e.currentTarget.value } })
                }
              />
            </div>
          </div>
          <Show when={aiReply()}>{(reply) => <p class="wizard-ok">Assistant replied: {reply()}</p>}</Show>
          <div class="wizard-actions">
            <button class="ghost small" disabled={busy()} onClick={saveAi}>
              Save assistant
            </button>
            <button class="ghost small" disabled={busy()} onClick={testAi}>
              Test assistant
            </button>
          </div>

          <h2 class="section-title">Automated underwriting</h2>
          <p class="screen-hint" style="margin: 0 0 12px">
            Sandbox endpoints for DU and LPA. Left empty, submissions are
            scored by the built-in mock engine.
          </p>
          <div class="form-grid">
            <div class="field">
              <label>DU endpoint</label>
              <input
                class="mono"
                value={cfg()!.aus.du_endpoint}
                onInput={(e) =>
                  setCfg((c) => c && { ...c, aus: { ...c.aus, du_endpoint: e.currentTarget.value } })
                }
              />
            </div>
            <div class="field">
              <label>LPA endpoint</label>
              <input
                class="mono"
                value={cfg()!.aus.lpa_endpoint}
                onInput={(e) =>
                  setCfg((c) => c && { ...c, aus: { ...c.aus, lpa_endpoint: e.currentTarget.value } })
                }
              />
            </div>
          </div>
          <div class="wizard-actions">
            <button class="ghost small" disabled={busy()} onClick={saveAus}>
              Save AUS endpoints
            </button>
          </div>
        </Show>

        <Show when={error()}>
          <div class="form-error">{error()}</div>
        </Show>
        <p class="screen-hint">
          Team roster and persona switching live on <kbd class="keycap inline">⌃M</kbd>.
        </p>
      </Show>
    </div>
  );
}
