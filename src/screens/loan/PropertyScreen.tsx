import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { LoanDetail, SubjectProperty } from '../../types';
import { TaskPanel } from '../../components/TaskPanel';
import { LoanHeader } from '../../components/LoanHeader';
import { PizzaTracker } from '../../components/PizzaTracker';
import { LoanColumns } from '../../components/LoanColumns';
import { UnderwriterPanel } from '../../components/UnderwriterPanel';

interface Props {
  loanId: string;
  demo: boolean;
  onNavigate: (screen: string) => void;
}

const PROPERTY_TYPES = ['detached', 'condo', 'pud', 'manufactured', 'multi_family', 'other'];
const OCCUPANCY = ['primary', 'secondary', 'investment'];

const label = (s: string) =>
  s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());

export function PropertyScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId, screen: 'property' }),
  );
  const [edits, setEdits] = createSignal<Partial<SubjectProperty>>({});
  const [status, setStatus] = createSignal<'saving' | 'saved' | 'error' | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const field = <K extends keyof SubjectProperty>(f: K): SubjectProperty[K] => {
    const e = edits();
    return (f in e ? e[f] : detail()?.property?.[f]) as SubjectProperty[K];
  };

  const setField = <K extends keyof SubjectProperty>(f: K, v: SubjectProperty[K]) => {
    setEdits((e) => ({ ...e, [f]: v }));
  };

  const hasEdits = () => Object.keys(edits()).length > 0;

  const commit = async () => {
    if (!hasEdits() || props.demo) return;
    setStatus('saving');
    setError(null);
    try {
      await invoke('save_subject_property', {
        loanId: props.loanId,
        property: {
          address_line: field('address_line'),
          city: field('city'),
          state: field('state'),
          postal_code: field('postal_code'),
          property_type: field('property_type'),
          occupancy: field('occupancy'),
          year_built: field('year_built'),
          units: Number(field('units')) || 1,
          estimated_value: field('estimated_value'),
          notes: field('notes'),
        },
      });
      setEdits({});
      refetch();
      setStatus('saved');
      setTimeout(() => setStatus((s) => (s === 'saved' ? null : s)), 1800);
    } catch (err) {
      setStatus('error');
      setError(String(err));
    }
  };

  const textInput = (f: keyof SubjectProperty, ph?: string) => (
    <input
      type="text"
      placeholder={ph}
      value={(field(f) as string) ?? ''}
      onInput={(e) => setField(f, e.currentTarget.value || null)}
      onBlur={commit}
      onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
    />
  );

  const intInput = (f: 'year_built' | 'units', ph?: string) => (
    <input
      type="number"
      min={f === 'year_built' ? '1800' : '1'}
      max={f === 'year_built' ? '2100' : '4'}
      placeholder={ph}
      value={field(f) ?? ''}
      onInput={(e) =>
        setField(
          f,
          Number.isNaN(e.currentTarget.valueAsNumber) ? null : e.currentTarget.valueAsNumber,
        )
      }
      onBlur={commit}
      onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
    />
  );

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="property" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        lock={detail()?.lock ?? null}
        loanId={props.loanId}
        demo={props.demo}
        work={
          <Show when={detail()} fallback={<div class="screen-loading">Loading…</div>}>
        <div class="screen-title-row">
          <h2 class="screen-title">Subject property</h2>
          <span class={`save-status ${status() ?? ''}`}>
            {status() === 'saving' ? 'Saving…' : status() === 'saved' ? 'Saved ✓' : null}
          </span>
        </div>
        <Show when={props.demo}>
          <div class="demo-note">Demo mode — edits stay on this screen and are not persisted.</div>
        </Show>

        <div class="form-grid">
          <div class="field field-wide">
            <label>Street address</label>
            {textInput('address_line', 'Street address')}
          </div>
          <div class="field">
            <label>City</label>
            {textInput('city')}
          </div>
          <div class="field field-row">
            <div class="field">
              <label>State</label>
              {textInput('state', 'WA')}
            </div>
            <div class="field">
              <label>ZIP</label>
              {textInput('postal_code')}
            </div>
          </div>
          <div class="field">
            <label>Property type</label>
            <select
              value={field('property_type')}
              onChange={(e) => {
                setField('property_type', e.currentTarget.value);
                commit();
              }}
              onBlur={commit}
            >
              <For each={PROPERTY_TYPES}>{(t) => <option value={t}>{label(t)}</option>}</For>
            </select>
          </div>
          <div class="field">
            <label>Occupancy</label>
            <select
              value={field('occupancy')}
              onChange={(e) => {
                setField('occupancy', e.currentTarget.value);
                commit();
              }}
              onBlur={commit}
            >
              <For each={OCCUPANCY}>{(o) => <option value={o}>{label(o)}</option>}</For>
            </select>
          </div>
          <div class="field">
            <label>Year built</label>
            {intInput('year_built')}
          </div>
          <div class="field">
            <label>Units</label>
            {intInput('units')}
          </div>
          <div class="field">
            <label>Estimated value</label>
            <input
              type="number"
              min="0"
              value={field('estimated_value') ?? ''}
              onInput={(e) =>
                setField(
                  'estimated_value',
                  Number.isNaN(e.currentTarget.valueAsNumber)
                    ? null
                    : e.currentTarget.valueAsNumber,
                )
              }
              onBlur={commit}
              onKeyDown={(e) => e.key === 'Enter' && e.currentTarget.blur()}
            />
          </div>
          <div class="field field-wide">
            <label>Notes</label>
            {textInput('notes', 'Anything unusual about the property…')}
          </div>
        </div>

        <Show when={status() === 'error'}>
          <div class="form-error">{error()}</div>
        </Show>
      </Show>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="property"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'property')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="property" demo={props.demo} />
    </div>
  );
}
