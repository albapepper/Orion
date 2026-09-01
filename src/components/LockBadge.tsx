import { Show } from 'solid-js';
import type { LockInfo } from '../types';

interface Props {
  lock: LockInfo | null;
  currentUserId: string | null;
}

export function LockBadge(props: Props) {
  const mine = () => props.lock && props.lock.locked_by === props.currentUserId;

  return (
    <Show
      when={props.lock}
      fallback={<span class="lock-badge free">Read-only · no editor</span>}
    >
      <Show
        when={mine()}
        fallback={
          <span class="lock-badge other">
            {props.lock!.user_name} is editing
            {props.lock!.screen ? ` · ${props.lock!.screen} screen` : ''}
          </span>
        }
      >
        <span class="lock-badge mine">Editing</span>
      </Show>
    </Show>
  );
}
