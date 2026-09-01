import { createResource, createSignal, Match, Show, Switch } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { AppStatus, LoanSummary } from './types';
import { Dashboard } from './screens/Dashboard';
import { LoanList } from './screens/LoanList';
import { IncomeScreen } from './screens/loan/IncomeScreen';
import './App.css';

type Route =
  | { name: 'dashboard' }
  | { name: 'loans' }
  | { name: 'income'; loanId: string };

function App() {
  const [route, setRoute] = createSignal<Route>({ name: 'dashboard' });
  const [status] = createResource(() => invoke<AppStatus>('get_status'));

  const openLoan = (loan: LoanSummary) =>
    setRoute({ name: 'income', loanId: loan.id });

  return (
    <div class="app">
      <nav class="topbar">
        <div class="brand">
          <span class="brand-mark">◆</span> Orion
        </div>
        <div class="topbar-nav">
          <button
            classList={{ active: route().name === 'dashboard' }}
            onClick={() => setRoute({ name: 'dashboard' })}
          >
            Dashboard
          </button>
          <button
            classList={{ active: route().name !== 'dashboard' }}
            onClick={() => setRoute({ name: 'loans' })}
          >
            Loans
          </button>
        </div>
        <div class="topbar-status">
          <Show when={status()}>
            <span
              class="env-badge"
              classList={{ warn: !status()!.db_connected }}
              title={
                status()!.db_connected
                  ? 'Connected to database'
                  : 'No database — showing demo data'
              }
            >
              {status()!.db_connected ? 'DB connected' : 'Demo mode'}
            </span>
            <span class="env-badge">
              AI: {status()!.ai_provider} · {status()!.ai_model}
            </span>
          </Show>
        </div>
      </nav>

      <main class="content">
        <Switch>
          <Match when={route().name === 'dashboard'}>
            <Dashboard onOpenLoans={() => setRoute({ name: 'loans' })} />
          </Match>
          <Match when={route().name === 'loans'}>
            <LoanList onOpenLoan={openLoan} />
          </Match>
          <Match when={route().name === 'income'}>
            <IncomeScreen
              loanId={(route() as { name: 'income'; loanId: string }).loanId}
              onBack={() => setRoute({ name: 'loans' })}
            />
          </Match>
        </Switch>
      </main>
    </div>
  );
}

export default App;
