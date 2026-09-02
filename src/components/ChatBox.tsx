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

  return (
    <aside class="chat-box">
      <div class="chat-header">
        <div class="chat-model">
          <span class="model-dot" />
          <span class="model-name">{props.status?.ai_model ?? '…'}</span>
          <span class="model-provider">{props.status?.ai_provider ?? ''}</span>
        </div>
      </div>

      <div class="chat-messages">
        <Show when={entries().length === 0 && !loading()}>
          <div class="chat-empty">
            Context-aware chat — answers use what's on your screen. Press{' '}
            <kbd class="keycap inline">/</kbd> to type from anywhere.
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
        <input
          ref={inputRef}
          type="text"
          value={input()}
          onInput={(e) => setInput(e.currentTarget.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') send();
            if (e.key === 'Escape') e.currentTarget.blur();
          }}
          placeholder={`Ask about ${props.screen}…`}
          disabled={loading()}
        />
        <button onClick={send} disabled={loading()}>
          Send
        </button>
      </div>
    </aside>
  );
}
