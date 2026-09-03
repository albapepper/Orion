// Arrow-key movement between fields: arrows walk the form geometrically —
// left/right stay in the visual row, up/down jump to the nearest field
// above or below. Text editing wins first: left/right only escape the
// field once the caret is already at its edge, and multi-line textareas
// keep native caret movement entirely.

const EDITOR = 'input, select, textarea';

// Input types where arrows are safe to hijack. Date/radio/checkbox/select
// keep their native arrow behavior.
const MOVABLE = new Set(['text', 'number', 'search', 'url', 'tel', 'password', '']);

const isMovable = (el: EventTarget | null): el is HTMLInputElement =>
  el instanceof HTMLInputElement && MOVABLE.has(el.type);

export function fieldArrowNav(e: KeyboardEvent): boolean {
  if (!isMovable(e.target)) return false;
  const input = e.target as HTMLInputElement;
  const start = input.selectionStart ?? 0;
  const end = input.selectionEnd ?? 0;
  const atStart = start === 0 && end === 0;
  const atEnd = start === input.value.length && end === input.value.length;

  let dir: 'left' | 'right' | 'up' | 'down' | null = null;
  if (e.key === 'ArrowLeft' && atStart) dir = 'left';
  else if (e.key === 'ArrowRight' && atEnd) dir = 'right';
  else if (e.key === 'ArrowUp') dir = 'up';
  else if (e.key === 'ArrowDown') dir = 'down';
  if (!dir) return false;

  const screen = input.closest('.screen');
  if (!screen) return false;

  const all = Array.from(screen.querySelectorAll<HTMLElement>(EDITOR)).filter(
    (el) => !el.hasAttribute('disabled') && el.offsetParent !== null,
  );
  const from = input.getBoundingClientRect();

  if (dir === 'left' || dir === 'right') {
    const inRow = all.filter((el) => {
      if (el === input) return false;
      const r = el.getBoundingClientRect();
      return r.top < from.bottom - 4 && r.bottom > from.top + 4;
    });
    const side = inRow.filter((el) => {
      const r = el.getBoundingClientRect();
      return dir === 'left' ? r.right <= from.left + 4 : r.left >= from.right - 4;
    });
    side.sort((a, b) => {
      const ra = a.getBoundingClientRect();
      const rb = b.getBoundingClientRect();
      return dir === 'left' ? rb.right - ra.right : ra.left - rb.left;
    });
    const target =
      side[0] ?? all[all.indexOf(input) + (dir === 'left' ? -1 : 1)];
    if (!target) return false;
    e.preventDefault();
    target.focus();
    if (target instanceof HTMLInputElement && target.type === 'text') {
      const edge = dir === 'left' ? target.value.length : 0;
      target.setSelectionRange(edge, edge);
    }
    return true;
  }

  const column = all.filter((el) => {
    if (el === input) return false;
    const r = el.getBoundingClientRect();
    return dir === 'up' ? r.bottom <= from.top + 4 : r.top >= from.bottom - 4;
  });
  const centerX = (r: DOMRect) => r.left + r.width / 2;
  column.sort((a, b) => {
    const ra = a.getBoundingClientRect();
    const rb = b.getBoundingClientRect();
    const da = dir === 'up' ? from.top - ra.bottom : ra.top - from.bottom;
    const db = dir === 'up' ? from.top - rb.bottom : rb.top - from.bottom;
    return da - db || Math.abs(centerX(ra) - centerX(from)) - Math.abs(centerX(rb) - centerX(from));
  });
  if (!column[0]) return false;
  e.preventDefault();
  column[0].focus();
  return true;
}
