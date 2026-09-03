import { createResource, createSignal, createEffect, Match, onCleanup, onMount, Switch } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { AppStatus, LoanSummary } from './types';
import { NavTray } from './components/NavTray';
import { ChatBox } from './components/ChatBox';
import { KeyLegend } from './components/KeyLegend';
import { setUser } from './lib/session';
import { OverviewScreen } from './screens/OverviewScreen';
import { Dashboard } from './screens/Dashboard';
import { IncomeScreen } from './screens/loan/IncomeScreen';
import { TermsScreen } from './screens/loan/TermsScreen';
import { BorrowerScreen } from './screens/loan/BorrowerScreen';
import { EmploymentScreen } from './screens/loan/EmploymentScreen';
import { AssetsScreen } from './screens/loan/AssetsScreen';
import { LiabilitiesScreen } from './screens/loan/LiabilitiesScreen';
import { PropertyScreen } from './screens/loan/PropertyScreen';
import { ReoScreen } from './screens/loan/ReoScreen';
import { DeclarationsScreen } from './screens/loan/DeclarationsScreen';
import { DemographicScreen } from './screens/loan/DemographicScreen';
import { StubScreen } from './screens/loan/StubScreen';
import { LOAN_SCREENS, nextStage, prevStage, type LoanScreen } from './screens/loan/stages';
import { SettingsScreen } from './screens/SettingsScreen';
import { fieldArrowNav } from './lib/fieldNav';
import './App.css';

type Route =
  | { name: 'dashboard' }
  | { name: 'settings' }
  | { name: 'overview'; loanId: string }
  | { name: LoanScreen; loanId: string };

function App() {
  const [route, setRoute] = createSignal<Route>({ name: 'dashboard' });
  const [history, setHistory] = createSignal<Route[]>([]);
  const [legendOpen, setLegendOpen] = createSignal(false);
  const [activeLoanId, setActiveLoanId] = createSignal<string | null>(null);

  const [status] = createResource(() => invoke<AppStatus>('get_status'));
  const [loans] = createResource(() => invoke<LoanSummary[]>('list_loans'));
  // Signed-in persona drives locks, audit, and dynamic field permissions.
  const [who] = createResource(() => invoke<import('./types').UserAccount | null>('whoami'));
  createEffect(() => setUser(who() ?? null));

  const navigate = (next: Route) => {
    setHistory((h) => [...h, route()]);
    setRoute(next);
    if ('loanId' in next) setActiveLoanId(next.loanId);
  };

  // Home: back to the previous page, one step at a time
  const goBack = () => {
    const h = history();
    if (h.length === 0) return;
    setRoute(h[h.length - 1]);
    setHistory(h.slice(0, -1));
  };

  // Navigating to an app screen; loan screens fall back to the top recent
  // loan when none is open, so keyboard-only flow never dead-ends.
  const openScreen = (id: string) => {
    if (id === 'dashboard') {
      navigate({ name: 'dashboard' });
      return;
    }
    if (id === 'settings') {
      navigate({ name: 'settings' });
      return;
    }
    const loanId = activeLoanId() ?? loans()?.[0]?.id;
    if (!loanId) return;
    if (id === 'overview') {
      navigate({ name: 'overview', loanId });
      return;
    }
    navigate({ name: id as LoanScreen, loanId });
  };

  // Entering a loan starts at the top of the cascade: the borrower
  const openLoan = (loan: LoanSummary) => navigate({ name: 'borrower', loanId: loan.id });

  // Walk the cascade without leaving the loan; overview acts as the
  // head of the cascade (⌃] enters it at the borrower).
  const shiftStage = (dir: 1 | -1) => {
    const r = route();
    if (r.name === 'dashboard' || r.name === 'settings') return;
    if (r.name === 'overview') {
      if (dir === 1) navigate({ name: 'borrower', loanId: r.loanId });
      return;
    }
    const target = dir === 1 ? nextStage(r.name) : prevStage(r.name);
    if (target) navigate({ name: target, loanId: 'loanId' in r ? r.loanId : activeLoanId()! });
  };

  const inEditor = (el: EventTarget | null) =>
    el instanceof HTMLElement && ['INPUT', 'TEXTAREA', 'SELECT'].includes(el.tagName);

  const onKeyDown = (e: KeyboardEvent) => {
    // Page shortcuts are Ctrl-modified so they work even mid-typing
    if (e.ctrlKey && !e.metaKey && !e.altKey) {
      if (e.key === 'd') {
        e.preventDefault();
        openScreen('dashboard');
        return;
      }
      if (e.key === 's') {
        e.preventDefault();
        navigate({ name: 'settings' });
        return;
      }
      const screen = LOAN_SCREENS.find((s) => s.startsWith(e.key));
      if (screen) {
        e.preventDefault();
        openScreen(screen);
        return;
      }
      if (e.key === '[') {
        e.preventDefault();
        shiftStage(-1);
        return;
      }
      if (e.key === ']') {
        e.preventDefault();
        shiftStage(1);
        return;
      }
      const n = parseInt(e.key, 10);
      if (n >= 1 && n <= 9) {
        e.preventDefault();
        const loan = loans()?.[n - 1];
        if (loan) openLoan(loan);
      }
      return;
    }

    if (e.metaKey || e.altKey) return;

    // Arrows walk the form: between fields when the caret is at an edge
    if (
      e.key.startsWith('Arrow') &&
      inEditor(e.target) &&
      fieldArrowNav(e)
    ) {
      return;
    }

    if (e.key === 'Escape') {
      if (legendOpen()) setLegendOpen(false);
      else if (inEditor(e.target)) (e.target as HTMLElement).blur();
      return;
    }

    if (inEditor(e.target)) return;

    switch (e.key) {
      case '/':
        e.preventDefault();
        window.dispatchEvent(new Event('orion:focus-chat'));
        break;
      case '?':
        setLegendOpen((v) => !v);
        break;
      // Home is the universal back button — but only on loan screens.
      // Field → tracker → dashboard. The first press inside a text field
      // parks the caret at the start (standard editing); from there it
      // backs out. Elsewhere (dashboard, chat) Home behaves natively.
      case 'Home': {
        const target = e.target as HTMLElement;
        if (inEditor(target) && target.closest('.screen')) {
          e.preventDefault();
          const el = target as HTMLInputElement | HTMLTextAreaElement;
          if ((el.selectionStart ?? 0) === 0 && (el.selectionEnd ?? 0) === 0) {
            el.blur();
            window.dispatchEvent(new Event('orion:focus-tracker'));
          } else {
            el.setSelectionRange(0, 0);
          }
        } else if (!inEditor(target)) {
          e.preventDefault();
          if (route().name !== 'dashboard') openScreen('dashboard');
          else goBack();
        }
        break;
      }
    }
  };

  onMount(() => window.addEventListener('keydown', onKeyDown));
  onCleanup(() => window.removeEventListener('keydown', onKeyDown));

  const currentLoanId = () => {
    const r = route();
    return 'loanId' in r ? r.loanId : activeLoanId();
  };

  return (
    <div class="app">
      <NavTray
        activeNav={route().name}
        onNavigate={openScreen}
        recentLoans={loans() ?? []}
        activeLoanId={activeLoanId()}
        onOpenLoan={openLoan}
        onShowLegend={() => setLegendOpen(true)}
        status={status()}
        user={who() ?? null}
      />

      <main class="workspace">
        <Switch>
          <Match when={route().name === 'dashboard'}>
            <Dashboard loans={loans() ?? []} onOpenLoan={openLoan} />
          </Match>
          <Match when={route().name === 'settings'}>
            <SettingsScreen />
          </Match>
          <Match when={route().name === 'overview'}>
            <OverviewScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'income'}>
            <IncomeScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'borrower'}>
            <BorrowerScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'employment'}>
            <EmploymentScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'terms'}>
            <TermsScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'assets'}>
            <AssetsScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'liabilities'}>
            <LiabilitiesScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'property'}>
            <PropertyScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'reo'}>
            <ReoScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'declarations'}>
            <DeclarationsScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'demographic'}>
            <DemographicScreen
              loanId={(route() as { loanId: string }).loanId}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
          <Match when={route().name === 'credit'}>
            <StubScreen
              loanId={(route() as { loanId: string }).loanId}
              screen={route().name as LoanScreen}
              demo={status() ? !status()!.db_connected : false}
              onNavigate={openScreen}
            />
          </Match>
        </Switch>
      </main>

      <ChatBox loanId={currentLoanId()} screen={route().name} status={status()} />

      <KeyLegend open={legendOpen()} onClose={() => setLegendOpen(false)} />
    </div>
  );
}

export default App;
