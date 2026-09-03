use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// A document on a loan file. Records the what/when/who — the bytes
/// themselves live outside the database for now.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Document {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub title: String,
    pub doc_type: String,
    pub screen: Option<String>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i64>,
    pub uploaded_by: Option<Uuid>,
    pub uploaded_at: DateTime<Utc>,
    pub removed_at: Option<DateTime<Utc>>,
    pub uploaded_by_name: Option<String>,
}

const DOC_COLUMNS: &str = r#"
        d.id, d.loan_id, d.company_id, d.title, d.doc_type, d.screen,
        d.file_name, d.mime_type, d.size_bytes, d.uploaded_by,
        d.uploaded_at, d.removed_at, u.name AS uploaded_by_name
"#;

pub async fn list_for_loan(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<Document>> {
    let rows = sqlx::query_as::<_, Document>(&format!(
        r#"
        SELECT {DOC_COLUMNS}
        FROM loan_documents d
        LEFT JOIN users u ON u.id = d.uploaded_by
        WHERE d.loan_id = $1 AND d.removed_at IS NULL
        ORDER BY d.uploaded_at DESC
        "#
    ))
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
pub async fn add(
    pool: &PgPool,
    loan_id: Uuid,
    company_id: Uuid,
    title: &str,
    doc_type: &str,
    screen: Option<&str>,
    file_name: Option<&str>,
    mime_type: Option<&str>,
    size_bytes: Option<i64>,
    uploaded_by: Uuid,
) -> anyhow::Result<Document> {
    let row = sqlx::query_as::<_, Document>(&format!(
        r#"
        INSERT INTO loan_documents (loan_id, company_id, title, doc_type,
                                    screen, file_name, mime_type, size_bytes, uploaded_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING {DOC_COLUMNS}
        "#
    ))
    .bind(loan_id)
    .bind(company_id)
    .bind(title)
    .bind(doc_type)
    .bind(screen)
    .bind(file_name)
    .bind(mime_type)
    .bind(size_bytes)
    .bind(uploaded_by)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

/// Soft delete — the row stays so the audit trail stays honest.
pub async fn remove(pool: &PgPool, doc_id: Uuid) -> anyhow::Result<()> {
    sqlx::query("UPDATE loan_documents SET removed_at = now() WHERE id = $1")
        .bind(doc_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// company_id + loan_id for audit entries.
pub async fn context_of(pool: &PgPool, doc_id: Uuid) -> anyhow::Result<(Uuid, Uuid)> {
    let row: (Uuid, Uuid) =
        sqlx::query_as("SELECT company_id, loan_id FROM loan_documents WHERE id = $1")
            .bind(doc_id)
            .fetch_one(pool)
            .await?;
    Ok(row)
}
