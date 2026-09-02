import { createSignal, For, onCleanup, onMount, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { AiResponse, AppStatus } from '../types';

interface Message {
  role: 'user' | 'assistant';
  content: string;
}

interface Props {
  loanId: string | null;
  screen: string; // whichever screen is active
  status?: AppStatus;
}

export function ChatBox(props: Props) {
  const [messages, setMessages] = createSignal<Message[]>([]);
  const [input, setInput] = createSignal('');
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  let inputRef: HTMLInputElement | undefined;

  // "/" anywhere in the app focuses the chat input
  const focusHandler = () => inputRef?.focus();
  onMount(() => window.addEventListener('orion:focus-chat', focusHandler));
  onCleanup(() => window.removeEventListener('orion:focus-chat', focusHandler));

  const send = async () => {
    const text = input().trim();
    if (!text || loading()) return;

    setMessages((m) => [...m, { role: 'user', content: text }]);
    setInput('');
    setLoading(true);
    setError(null);

    try {
      const response = await invoke<AiResponse>('ai_chat', {
        loanId: props.loanId,
        screen: props.screen,
        messages: messages(),
      });
      setMessages((m) => [...m, { role: 'assistant', content: response.content }]);
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
        <span class="screen-badge">{props.screen}</span>
      </div>

      <div class="chat-messages">
        <Show when={messages().length === 0 && !loading()}>
          <div class="chat-empty">
            Context-aware chat for the {props.screen} screen. Press{' '}
            <kbd class="keycap inline">/</kbd> to type from anywhere.
          </div>
        </Show>
        <For each={messages()}>
          {(msg) => <div class={`message ${msg.role}`}>{msg.content}</div>}
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
