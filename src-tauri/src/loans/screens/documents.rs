use crate::documents::Document;

/// The Documents screen context: what's in the file, organized by the
/// stage each document supports, so gaps are visible at a glance.
pub fn build_context(documents: &[Document]) -> String {
    if documents.is_empty() {
        return r#"You are an expert mortgage assistant inside the Orion loan
origination platform, answering on the Documents screen.

DOCUMENTS ON FILE: none yet.

YOUR ROLE:
- Note that the file has no documents and suggest the usual starting
  set (income verification, bank statements, ID, insurance)
- Be concise"#
            .to_string();
    }

    let lines: Vec<String> = documents
        .iter()
        .map(|d| {
            format!(
                "  - [{}] {} — supports stage {}, uploaded by {} on {}{}",
                d.doc_type,
                d.title,
                d.screen.as_deref().unwrap_or("general"),
                d.uploaded_by_name.as_deref().unwrap_or("unknown"),
                d.uploaded_at.format("%Y-%m-%d"),
                d.file_name
                    .as_deref()
                    .map(|f| format!(" ({f})"))
                    .unwrap_or_default(),
            )
        })
        .collect();

    format!(
        r#"You are an expert mortgage assistant inside the Orion loan
origination platform, answering on the Documents screen.

DOCUMENTS ON FILE ({}):
{lines}

YOUR ROLE:
- Answer questions about what's on file and what each document covers
- Flag gaps: a file usually needs income verification, bank statements,
  ID, and (if applicable) the appraisal, title, and insurance documents
- Note when a supporting document is missing for a stage the borrower
  has filled in
- Be concise"#,
        documents.len(),
        lines = lines.join("\n"),
    )
}
