import { createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { UwReview, UwReviewStatus } from '../types';
import { canEditReview, ROLE_LABELS, user } from '../lib/session';

interface Props {
  loanId: string;
  screen: string;
  review?: UwReview;
  demo?: boolean;
}

const STATUSES: UwReviewStatus[] = ['pending', 'verified', 'flagged'];

const STATUS_LABEL: Record<UwReviewStatus, string> = {
  pending: 'Pending',
  verified: 'Verified',
  flagged: 'Flagged',
};

// The underwriter column: verification status + notes for one screen.
// Everyone touching the loan reads it here; the fields are live only for
// underwriters — dynamic per signed-in user, enforced again server-side.
export function UnderwriterPanel(props: Props) {
  const editable = () => canEditReview(user());

  // Local edits sit on top of props.review, which may still be loading.
  const [local, setLocal] = createSignal<{ status?: UwReviewStatus; note?: string }>({});
  const status = () => local().status ?? props.review?.status ?? 'pending';
  const note = () => local().note ?? props.review?.note ?? '';
  const [saved, setSaved] = createSignal<'saving' | 'saved' | 'error' | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const savedFlash = () => {
    setSaved('saved');
    setTimeout(() => setSaved((s) => (s === 'saved' ? null : s)), 1800);
  };

  const commit = async (next?: { status?: UwReviewStatus; note?: string }) => {
    if (!editable()) return;
    const payload = {
      status: next?.status ?? status(),
      note: next?.note ?? note(),
    };
    if (props.demo) {
      setLocal(payload);
      savedFlash();
      return;
    }
    setSaved('saving');
    setError(null);
    try {
      const updated = await invoke<UwReview>('save_uw_review', {
        loanId: props.loanId,
        review: { screen: props.screen, status: payload.status, note: payload.note },
      });
      setLocal({ status: updated.status as UwReviewStatus, note: updated.note ?? '' });
      setSaved('saved');
      setTimeout(() => setSaved((s) => (s === 'saved' ? null : s)), 1800);
    } catch (err) {
      setSaved('error');
      setError(String(err));
    }
  };

  return (
    <div class="uw-panel" classList={{ locked: !editable() }}>
      <div class="uw-panel-header">
        <span class="uw-panel-title">Underwriter</span>
        <Show when={user()}>
          <span class="uw-panel-role" title={user()!.email}>
            {ROLE_LABELS[user()!.role] ?? user()!.role}
          </span>
        </Show>
      </div>

      <div class="uw-status-row" role="group" aria-label="Review status">
        <For each={STATUSES}>
          {(s) => (
            <button
              type="button"
              class="uw-status"
              classList={{ active: status() === s, [s]: true }}
              disabled={!editable()}
              onClick={() => {
                setLocal((l) => ({ ...l, status: s }));
                commit({ status: s });
              }}
            >
              {STATUS_LABEL[s]}
            </button>
          )}
        </For>
      </div>

      <textarea
        class="uw-note"
        placeholder="Verification notes — paystubs, bank statements, conditions…"
        rows={7}
        value={note()}
        readOnly={!editable()}
        onInput={(e) => setLocal((l) => ({ ...l, note: e.currentTarget.value }))}
        onBlur={() => note() !== (props.review?.note ?? '') && commit()}
      />

      <Show
        when={editable()}
        fallback={
          <div class="uw-lock-hint">
            <Show
              when={user()}
              fallback="Sign in as an underwriter to edit the review."
            >
              Underwriter only — you're signed in as{' '}
              <strong>{user()!.name}</strong> ({ROLE_LABELS[user()!.role]}).
              Switch personas on the Team screen (⌃m).
            </Show>
          </div>
        }
      >
        <div class="uw-panel-footer">
          <span class={`save-status ${saved() ?? ''}`}>
            {saved() === 'saving' ? 'Saving…' : saved() === 'saved' ? 'Saved ✓' : null}
          </span>
          <Show when={props.review?.updated_by_name}>
            <span class="uw-updated" title="Last review update">
              {props.review!.updated_by_name}
              {' · '}
              {new Date(props.review!.updated_at).toLocaleDateString()}
            </span>
          </Show>
        </div>
      </Show>

      <Show when={saved() === 'error'}>
        <div class="form-error">{error()}</div>
      </Show>
    </div>
  );
}
