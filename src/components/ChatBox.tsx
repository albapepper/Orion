import { createSignal, For, onCleanup, onMount, Show, Switch, Match } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { AiResponse, AppStatus } from '../types';

interface ChatMessage {
  role: 'user' | 'assistant';
  content: string;
}

// A divider marks where the conversation moved to a different screen,
// so discussions about income vs. assets read as separate threads.
type Entry = ChatMessage | { role: 'divider'; screen: string };

interface Props {
  loanId: string | null;
  screen: string; // whichever screen is active
  status?: AppStatus;
}

export function ChatBox(props: Props) {
  const [entries, setEntries] = createSignal<Entry[]>([]);
  const [input, setInput] = createSignal('');
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  const [lastScreen, setLastScreen] = createSignal<string | null>(null);
  let inputRef: HTMLInputElement | undefined;

  // "/" anywhere in the app focuses the chat input
  const focusHandler = () => inputRef?.focus();
  onMount(() => window.addEventListener('orion:focus-chat', focusHandler));
  onCleanup(() => window.removeEventListener('orion:focus-chat', focusHandler));

  const history = () =>
    entries().filter((e): e is ChatMessage => e.role !== 'divider');

  const send = async () => {
    const text = input().trim();
    if (!text || loading()) return;

    if (lastScreen() !== props.screen) {
      setEntries((m) => [...m, { role: 'divider', screen: props.screen }]);
      setLastScreen(props.screen);
    }

    setEntries((m) => [...m, { role: 'user', content: text }]);
    setInput('');
    setLoading(true);
    setError(null);

    try {
      const response = await invoke<AiResponse>('ai_chat', {
        loanId: props.loanId,
        screen: props.screen,
        messages: history(),
      });
      setEntries((m) => [...m, { role: 'assistant', content: response.content }]);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const submitPrompt = (prompt: string) => {
    setInput(prompt);
    queueMicrotask(send);
  };

  const contextLabel = () => props.screen === 'dashboard'
    ? 'Your pipeline'
    : props.screen === 'loans'
      ? 'All loans'
      : props.screen.replace(/_/g, ' ');

  return (
    <aside class="chat-box">
      <div class="chat-header">
        <div class="chat-title-block">
          <span class="chat-eyebrow">Local intelligence</span>
          <span class="chat-title">Orion AI</span>
        </div>
        <div class="chat-model" title={`${props.status?.ai_model ?? 'Local model'} · ${props.status?.ai_provider ?? 'On device'}`}>
          <span class="model-dot" />
          <span class="model-state">Local</span>
        </div>
      </div>

      <div class="chat-context">
        <span class="context-label">Looking at</span>
        <span class="context-value">{contextLabel()}</span>
      </div>

      <div class="chat-messages">
        <Show when={entries().length === 0 && !loading()}>
          <div class="chat-empty">
            <div class="chat-orbit" aria-hidden="true"><span /></div>
            <h3>Ask in context</h3>
            <p>I can see where each file is on the path and help choose the next move. Your data stays on this device.</p>
            <div class="chat-suggestions">
              <button onClick={() => submitPrompt('What should I work on next?')}>What should I work on next?</button>
              <button onClick={() => submitPrompt('Summarize anything that needs attention.')}>What needs attention?</button>
            </div>
          </div>
        </Show>
        <For each={entries()}>
          {(entry) => (
            <Switch>
              <Match when={entry.role === 'divider'}>
                <div class="chat-divider">
                  <span>{(entry as { screen: string }).screen}</span>
                </div>
              </Match>
              <Match when={entry.role !== 'divider'}>
                <div class={`message ${entry.role}`}>
                  {(entry as ChatMessage).content}
                </div>
              </Match>
            </Switch>
          )}
        </For>
        {loading() && <div class="message assistant loading">Thinking…</div>}
        <Show when={error()}>
          <div class="message error">{error()}</div>
        </Show>
      </div>

      <div class="chat-input">
        <div class="chat-composer">
          <input
          ref={inputRef}
          type="text"
          value={input()}
          onInput={(e) => setInput(e.currentTarget.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') send();
            if (e.key === 'Escape') e.currentTarget.blur();
          }}
          placeholder={`Ask about ${contextLabel().toLowerCase()}…`}
          disabled={loading()}
          />
          <button onClick={send} disabled={loading()} aria-label="Send message">
            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true"><path d="m4 10 12-6-4 12-2.2-4.2L4 10Z" /></svg>
          </button>
        </div>
        <span class="composer-hint">Press <kbd>/</kbd> anywhere to ask</span>
      </div>
    </aside>
  );
}
