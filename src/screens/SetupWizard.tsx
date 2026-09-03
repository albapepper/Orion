import { createSignal, For, onMount, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { Config, Company } from '../types';

interface Props {
  onFinished: () => void;
  onDemo: () => void;
}

type Step = 'welcome' | 'database' | 'company' | 'assistant' | 'done';

const STEPS: Array<[Step, string]> = [
  ['welcome', 'Welcome'],
  ['database', 'Database'],
  ['company', 'Company'],
  ['assistant', 'Assistant'],
  ['done', 'Done'],
];

// P5-01: the first-run wizard. The backend commands existed from the
// start — this drives them in order: connect, deploy schema, create
// the company row, check the assistant. "Continue in demo mode" is
// always available: demo is a feature, not a failure state.
export function SetupWizard(props: Props) {
  const [step, setStep] = createSignal<Step>('welcome');
  const [cfg, setCfg] = createSignal<Config | null>(null);
  const [dbUrl, setDbUrl] = createSignal('');
  const [dbVersion, setDbVersion] = createSignal<string | null>(null);
  const [schemaDeployed, setSchemaDeployed] = createSignal(false);
  const [companyName, setCompanyName] = createSignal('');
  const [companies, setCompanies] = createSignal<Company[]>([]);
  const [companyId, setCompanyId] = createSignal<string | null>(null);
  const [aiTested, setAiTested] = createSignal<string | null>(null);
  const [busy, setBusy] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);

  onMount(async () => {
    try {
      const c = await invoke<Config>('get_config');
      setCfg(c);
      setDbUrl(c.database.url);
    } catch (e) {
      setError(String(e));
    }
  });

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

  const testDb = () =>
    run(async () => {
      const v = await invoke<string>('test_db_connection', { url: dbUrl() });
      setDbVersion(v);
    });

  const deploySchema = () =>
    run(async () => {
      await invoke('deploy_schema', { url: dbUrl() });
      setSchemaDeployed(true);
      setCompanies(await invoke<Company[]>('list_companies'));
    });

  const createCompany = () =>
    run(async () => {
      const c = await invoke<Company>('create_company', { name: companyName() });
      setCompanyId(c.id);
    });

  const useExisting = (c: Company) =>
    run(async () => {
      await invoke('set_company', { companyId: c.id });
      setCompanyId(c.id);
    });

  // Persist everything so far, then round-trip the assistant. Saving
  // first is what rebuilds the AI provider from the new settings.
  const saveAndTestAi = () =>
    run(async () => {
      const c = cfg()!;
      await invoke('save_config', {
        config: {
          ...c,
          database: { url: dbUrl() },
          app: { ...c.app, company_id: companyId() ?? c.app.company_id },
        },
      });
      const reply = await invoke<string>('test_ai_connection');
      setAiTested(reply.trim());
      setStep('done');
    });

  const finish = () =>
    run(async () => {
      // A skip of the assistant still needs the DB + company persisted.
      const c = cfg()!;
      await invoke('save_config', {
        config: {
          ...c,
          database: { url: dbUrl() },
          app: { ...c.app, company_id: companyId() ?? c.app.company_id },
        },
      });
      props.onFinished();
    });

  const stepIndex = () => STEPS.findIndex(([s]) => s === step());

  return (
    <div class="wizard">
      <div class="wizard-card">
        <div class="wizard-steps">
          <For each={STEPS}>
            {([s, label], i) => (
              <span
                class="wizard-step"
                classList={{ active: step() === s, past: i() < stepIndex() }}
              >
                {label}
              </span>
            )}
          </For>
        </div>

        <Show when={step() === 'welcome'}>
          <h1 class="wizard-title">Welcome to Orion</h1>
          <p class="wizard-text">
            Orion runs entirely on your machine: your database, your
            assistant, your data. Three quick steps connect this install
            to a Postgres database and set up your company — or explore
            first with the built-in demo file.
          </p>
          <div class="wizard-actions">
            <button class="ghost" onClick={() => setStep('database')}>
              Set up for real
            </button>
            <button class="ghost" onClick={() => props.onDemo()}>
              Continue in demo mode
            </button>
          </div>
        </Show>

        <Show when={step() === 'database'}>
          <h1 class="wizard-title">Connect your database</h1>
          <p class="wizard-text">
            Orion stores everything in your own Postgres. Nothing leaves
            the machine you point it at.
          </p>
          <div class="field">
            <label>Postgres URL</label>
            <input
              class="wizard-input mono"
              placeholder="postgres://user:pass@localhost:5432/orion"
              value={dbUrl()}
              onInput={(e) => setDbUrl(e.currentTarget.value)}
            />
          </div>
          <Show when={dbVersion()}>
            <p class="wizard-ok">Connected: {dbVersion()}</p>
          </Show>
          <Show when={schemaDeployed()}>
            <p class="wizard-ok">Schema deployed — all migrations applied.</p>
          </Show>
          <div class="wizard-actions">
            <button class="ghost" disabled={busy() || !dbUrl().trim()} onClick={testDb}>
              Test connection
            </button>
            <Show
              when={dbVersion()}
              fallback={<span class="wizard-note">Test first, then deploy the schema.</span>}
            >
              <button class="ghost" disabled={busy()} onClick={deploySchema}>
                {schemaDeployed() ? 'Schema ready ✓' : 'Deploy schema'}
              </button>
            </Show>
            <Show when={schemaDeployed()}>
              <button class="ghost" onClick={() => setStep('company')}>
                Continue →
              </button>
            </Show>
          </div>
        </Show>

        <Show when={step() === 'company'}>
          <h1 class="wizard-title">Your company</h1>
          <p class="wizard-text">
            One company per install. Create yours, or pick an existing
            row if this database was set up before.
          </p>
          <Show when={companies().length > 0}>
            <div class="field">
              <label>Existing companies</label>
              <div class="wizard-companies">
                <For each={companies()}>
                  {(c) => (
                    <button
                      class="ghost small"
                      classList={{ 'wizard-picked': companyId() === c.id }}
                      onClick={() => useExisting(c)}
                    >
                      {c.name}
                    </button>
                  )}
                </For>
              </div>
            </div>
          </Show>
          <div class="field">
            <label>Or create a new company</label>
            <input
              class="wizard-input"
              placeholder="Company name"
              value={companyName()}
              onInput={(e) => setCompanyName(e.currentTarget.value)}
            />
          </div>
          <div class="wizard-actions">
            <button
              class="ghost"
              disabled={busy() || !companyName().trim()}
              onClick={createCompany}
            >
              Create company
            </button>
            <Show when={companyId()}>
              <button class="ghost" onClick={() => setStep('assistant')}>
                Continue →
              </button>
            </Show>
          </div>
        </Show>

        <Show when={step() === 'assistant'}>
          <h1 class="wizard-title">The assistant</h1>
          <p class="wizard-text">
            Orion's AI runs locally through Ollama by default — no data
            leaves your machine. This step is optional; everything else
            works without it.
          </p>
          <div class="field">
            <label>Endpoint</label>
            <input
              class="wizard-input mono"
              value={cfg()?.ai.endpoint ?? ''}
              onInput={(e) =>
                setCfg((c) => c && { ...c, ai: { ...c.ai, endpoint: e.currentTarget.value } })
              }
            />
          </div>
          <div class="field">
            <label>Model</label>
            <input
              class="wizard-input mono"
              value={cfg()?.ai.model ?? ''}
              onInput={(e) =>
                setCfg((c) => c && { ...c, ai: { ...c.ai, model: e.currentTarget.value } })
              }
            />
          </div>
          <Show when={aiTested()}>
            <p class="wizard-ok">Assistant replied: {aiTested()}</p>
          </Show>
          <div class="wizard-actions">
            <button class="ghost" disabled={busy()} onClick={saveAndTestAi}>
              Save & test assistant
            </button>
            <button class="ghost" disabled={busy()} onClick={finish}>
              Skip — finish setup
            </button>
          </div>
        </Show>

        <Show when={step() === 'done'}>
          <h1 class="wizard-title">You're set up</h1>
          <p class="wizard-text">
            Database connected and schema deployed. Company configured.
            {aiTested()
              ? ' Assistant checked.'
              : ' The assistant was left untested — you can check it any time in Settings.'}
            The team signs in from Team (⌃m) → "Working as". Start your
            first file with <strong>+ New loan</strong> on the loans screen
            (⌃n).
          </p>
          <div class="wizard-actions">
            <button class="ghost" onClick={() => props.onFinished()}>
              Open Orion →
            </button>
          </div>
        </Show>

        <Show when={error()}>
          <div class="task-error">{error()}</div>
        </Show>

        <Show when={step() !== 'welcome'}>
          <div class="wizard-footer">
            <button class="ghost small" onClick={() => props.onDemo()}>
              Continue in demo mode
            </button>
          </div>
        </Show>
      </div>
    </div>
  );
}
