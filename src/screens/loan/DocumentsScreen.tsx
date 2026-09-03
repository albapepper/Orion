import { createResource, createSignal, For, Show } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';
import type { Document, DocumentType, LoanDetail } from '../../types';
import { TaskPanel } from '../../components/TaskPanel';
import { LoanHeader } from '../../components/LoanHeader';
import { PizzaTracker } from '../../components/PizzaTracker';
import { LoanColumns } from '../../components/LoanColumns';
import { UnderwriterPanel } from '../../components/UnderwriterPanel';
import { LOAN_SCREENS, SCREEN_LABELS } from './stages';

interface Props {
  loanId: string;
  demo: boolean;
  onNavigate: (screen: string) => void;
}

const DOC_TYPES: DocumentType[] = [
  'application',
  'income_verification',
  'bank_statement',
  'tax_return',
  'appraisal',
  'title',
  'insurance',
  'id_document',
  'other',
];

const label = (s: string) =>
  s.replace(/_/g, ' ').replace(/\b\w/g, (c) => c.toUpperCase());

const docStages = LOAN_SCREENS.filter((s) => s !== 'documents' && s !== 'conditions');

const dateLabel = (iso: string) =>
  new Date(iso).toLocaleDateString(undefined, { month: 'short', day: 'numeric' });

const sizeLabel = (bytes: number | null) => {
  if (bytes === null) return '';
  if (bytes < 1024 * 1024) return `${Math.round(bytes / 1024)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
};

// The file's paperwork: what's on record, which stage it supports, who
// uploaded it. Document bytes live outside the database for now — these
// are the records the whole team works from.
export function DocumentsScreen(props: Props) {
  const [detail, { refetch }] = createResource(
    () => props.loanId,
    (loanId) => invoke<LoanDetail>('get_loan_detail', { loanId }),
  );
  const [title, setTitle] = createSignal('');
  const [docType, setDocType] = createSignal<DocumentType>('income_verification');
  const [screen, setScreen] = createSignal('');
  const [fileName, setFileName] = createSignal('');
  const [error, setError] = createSignal<string | null>(null);
  const [localDocs, setLocalDocs] = createSignal<Document[] | null>(null);

  const documents = () => localDocs() ?? (detail()?.documents ?? []);

  const addDoc = async () => {
    const t = title().trim();
    if (!t) return;
    if (props.demo) {
      setLocalDocs([
        {
          id: crypto.randomUUID(),
          loan_id: props.loanId,
          company_id: '',
          title: t,
          doc_type: docType(),
          screen: screen() || null,
          file_name: fileName() || null,
          mime_type: 'application/pdf',
          size_bytes: null,
          uploaded_by: null,
          uploaded_at: new Date().toISOString(),
          removed_at: null,
          uploaded_by_name: 'You (demo)',
        },
        ...(localDocs() ?? documents()),
      ]);
      resetForm();
      return;
    }
    try {
      await invoke('add_document', {
        loanId: props.loanId,
        title: t,
        docType: docType(),
        screen: screen() || null,
        fileName: fileName() || null,
      });
      resetForm();
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  const resetForm = () => {
    setTitle('');
    setFileName('');
    setDocType('income_verification');
    setScreen('');
  };

  const removeDoc = async (doc: Document) => {
    if (props.demo) {
      setLocalDocs((localDocs() ?? documents()).filter((d) => d.id !== doc.id));
      return;
    }
    try {
      await invoke('remove_document', { documentId: doc.id });
      setError(null);
      refetch();
    } catch (e) {
      setError(String(e));
    }
  };

  return (
    <div class="screen">
      <LoanHeader detail={detail()} />
      <PizzaTracker current="documents" detail={detail()} onNavigate={props.onNavigate} />

      <LoanColumns
        work={
          <>
            <div class="screen-title-row">
              <h2 class="screen-title">Documents on file</h2>
              <span class="save-status">{documents().length} on record</span>
            </div>
            <Show when={props.demo}>
              <div class="demo-note">Demo mode — edits stay on this screen and are not persisted.</div>
            </Show>

            <div class="doc-add">
              <input
                type="text"
                placeholder="Document title…"
                value={title()}
                onInput={(e) => setTitle(e.currentTarget.value)}
                onKeyDown={(e) => e.key === 'Enter' && addDoc()}
              />
              <select
                value={docType()}
                onChange={(e) => setDocType(e.currentTarget.value as DocumentType)}
                title="Document type"
              >
                <For each={DOC_TYPES}>
                  {(t) => <option value={t}>{label(t)}</option>}
                </For>
              </select>
              <select
                value={screen()}
                onChange={(e) => setScreen(e.currentTarget.value)}
                title="Supports stage"
              >
                <option value="">General</option>
                <For each={docStages}>
                  {(s) => <option value={s}>{SCREEN_LABELS[s]}</option>}
                </For>
              </select>
              <input
                type="text"
                class="doc-file"
                placeholder="File reference…"
                title="File name or reference — bytes stay outside the database for now"
                value={fileName()}
                onInput={(e) => setFileName(e.currentTarget.value)}
                onKeyDown={(e) => e.key === 'Enter' && addDoc()}
              />
              <button onClick={addDoc}>Add</button>
            </div>

            <table class="data-table">
              <thead>
                <tr>
                  <th>Document</th>
                  <th>Type</th>
                  <th>Supports</th>
                  <th>File</th>
                  <th>Uploaded</th>
                  <th />
                </tr>
              </thead>
              <tbody>
                <For
                  each={documents()}
                  fallback={
                    <tr>
                      <td colspan={6}>No documents on record yet.</td>
                    </tr>
                  }
                >
                  {(doc) => (
                    <tr>
                      <td>{doc.title}</td>
                      <td>{label(doc.doc_type)}</td>
                      <td>
                        {doc.screen
                          ? (SCREEN_LABELS[doc.screen as keyof typeof SCREEN_LABELS] ?? doc.screen)
                          : 'General'}
                      </td>
                      <td class="doc-file-cell">
                        {doc.file_name ?? <span class="loans-none">—</span>}
                        <Show when={doc.size_bytes}>
                          <span class="doc-size"> · {sizeLabel(doc.size_bytes)}</span>
                        </Show>
                      </td>
                      <td>
                        {dateLabel(doc.uploaded_at)}
                        <Show when={doc.uploaded_by_name}>
                          <span class="doc-size"> · {doc.uploaded_by_name}</span>
                        </Show>
                      </td>
                      <td>
                        <button
                          class="ghost small"
                          title="Remove from record"
                          onClick={() => removeDoc(doc)}
                        >
                          ✕
                        </button>
                      </td>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
            <Show when={error()}>
              <div class="task-error">{error()}</div>
            </Show>
          </>
        }
        review={
          <UnderwriterPanel
            loanId={props.loanId}
            screen="documents"
            review={detail()?.uw_reviews?.find((r) => r.screen === 'documents')}
            demo={props.demo}
          />
        }
      />

      <TaskPanel loanId={props.loanId} screen="documents" demo={props.demo} />
    </div>
  );
}
