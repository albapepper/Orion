import { createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { AiResponse } from '../types';

interface Message {
  role: 'user' | 'assistant';
  content: string;
}

interface Props {
  loanId: string;
  screen: string; // passed from whichever screen is active
}

export function AiSidebar(props: Props) {
  const [messages, setMessages] = createSignal<Message[]>([]);
  const [input, setInput] = createSignal('');
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);

  const send = async () => {
    const text = input().trim();
    if (!text || loading()) return;

    const userMsg: Message = { role: 'user', content: text };
    setMessages((m) => [...m, userMsg]);
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
    <aside class="ai-sidebar">
      <div class="ai-header">
        <span class="ai-title">AI Assistant</span>
        <span class="screen-badge">{props.screen}</span>
      </div>

      <div class="ai-messages">
        <Show when={messages().length === 0 && !loading()}>
          <div class="ai-empty">
            Ask anything about this loan's {props.screen} data — answers use
            what's on screen right now.
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

      <div class="ai-input">
        <input
          type="text"
          value={input()}
          onInput={(e) => setInput(e.currentTarget.value)}
          onKeyDown={(e) => e.key === 'Enter' && send()}
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
