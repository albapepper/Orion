import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanContact } from '../types';

interface Props {
  loanId: string;
  demo: boolean;
}

const ROLES = ['realtor', 'title', 'escrow', 'insurance', 'other'];

const label = (s: string) =>
  s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());

// The deal's people (P5-10): realtor, title, escrow, insurance — a
// quiet directory under every loan screen, sibling of the task panel.
// Anyone on the file can read it; writes obey the loan lock.
export function ContactsPanel(props: Props) {
  const [contacts, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanContact[]>('list_contacts', { loanId }),
  );
  // Demo adds/removes can't persist — they stack locally.
  const [local, setLocal] = createSignal<LoanContact[] | null>(null);
  const [adding, setAdding] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  const [draft, setDraft] = createSignal<Partial<LoanContact>>({ role: 'realtor' });

  const rows = () => local() ?? (contacts() ?? []);

  const add = async () => {
    setError(null);
    try {
      const saved = await invoke<LoanContact>('add_loan_contact', {
        loanId: props.loanId,
        contact: {
          id: crypto.randomUUID(),
          loan_id: props.loanId,
          company_id: '',
          role: draft().role ?? 'other',
          name: draft().name ?? '',
          company: draft().company ?? null,
          email: draft().email ?? null,
          phone: draft().phone ?? null,
          notes: null,
        },
      });
      if (props.demo) {
        setLocal([saved, ...(local() ?? (contacts() ?? []))]);
      } else {
        refetch();
      }
      setAdding(false);
      setDraft({ role: 'realtor' });
    } catch (e) {
      setError(String(e));
    }
  };

  const remove = async (c: LoanContact) => {
    setError(null);
    try {
      await invoke('remove_loan_contact', { contactId: c.id });
      if (props.demo) {
        setLocal((local() ?? (contacts() ?? [])).filter((x) => x.id !== c.id));
      } else {
        refetch();
      }
    } catch (e) {
      setError(String(e));
    }
  };

  const draftField = (f: 'name' | 'company' | 'phone' | 'email') => (
    <input
      type="text"
      placeholder={label(f)}
      value={(draft()[f] as string) ?? ''}
      onInput={(e) => setDraft((d) => ({ ...d, [f]: e.currentTarget.value || null }))}
    />
  );

  return (
    <section class="contacts-panel">
      <div class="screen-title-row">
        <h2 class="screen-title">Contacts</h2>
        <span class="save-status">{rows().length}</span>
        <button
          class="ghost small"
          style="margin-left: auto"
          onClick={() => setAdding(!adding())}
        >
          {adding() ? 'Cancel' : '+ Add contact'}
        </button>
      </div>

      <Show when={adding()}>
        <div class="contact-add">
          <select
            value={draft().role ?? 'realtor'}
            onChange={(e) => setDraft((d) => ({ ...d, role: e.currentTarget.value }))}
          >
            <For each={ROLES}>{(r) => <option value={r}>{label(r)}</option>}</For>
          </select>
          {draftField('name')}
          {draftField('company')}
          {draftField('phone')}
          <button class="ghost small" disabled={!draft().name} onClick={add}>
            Save
          </button>
        </div>
      </Show>

      <Show
        when={rows().length > 0}
        fallback={<div class="task-empty">No contacts on this loan yet — the deal team lives here.</div>}
      >
        <div class="contact-grid">
          <For each={rows()}>
            {(c) => (
              <div class="contact-card">
                <div class="contact-head">
                  <span class="contact-role">{label(c.role)}</span>
                  <button
                    class="ghost small"
                    title="Remove contact"
                    onClick={() => remove(c)}
                  >
                    ✕
                  </button>
                </div>
                <div class="contact-name">{c.name}</div>
                <Show when={c.company}>
                  <div class="contact-line">{c.company}</div>
                </Show>
                <Show when={c.phone}>
                  <div class="contact-line mono">{c.phone}</div>
                </Show>
                <Show when={c.email}>
                  <div class="contact-line mono">{c.email}</div>
                </Show>
              </div>
            )}
          </For>
        </div>
      </Show>

      <Show when={error()}>
        <div class="task-error">{error()}</div>
      </Show>
    </section>
  );
}
