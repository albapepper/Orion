use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// An underwriting condition: what must arrive or be resolved before the
/// loan can clear. Outstanding conditions are the pipeline's blockers.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Condition {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub title: String,
    pub detail: Option<String>,
    pub source: String,
    pub status: String,
    pub priority: String,
    pub clear_by: Option<DateTime<Utc>>,
    pub cleared_at: Option<DateTime<Utc>>,
    pub cleared_by: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cleared_by_name: Option<String>,
}

const COND_COLUMNS: &str = r#"
        c.id, c.loan_id, c.company_id, c.title, c.detail, c.source, c.status,
        c.priority, c.clear_by, c.cleared_at, c.cleared_by, c.created_by,
        c.created_at, c.updated_at, u.name AS cleared_by_name
"#;

const COND_FROM: &str = r#"
        FROM loan_conditions c
        LEFT JOIN users u ON u.id = c.cleared_by
"#;

pub async fn list_for_loan(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<Condition>> {
    let rows = sqlx::query_as::<_, Condition>(&format!(
        r#"
        SELECT {COND_COLUMNS} {COND_FROM}
        WHERE c.loan_id = $1
        ORDER BY (c.status IN ('open', 'submitted')) DESC,
                 (c.priority = 'high') DESC,
                 c.clear_by ASC NULLS LAST,
                 c.created_at
        "#
    ))
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
pub async fn create(
    pool: &PgPool,
    loan_id: Uuid,
    company_id: Uuid,
    title: &str,
    detail: Option<&str>,
    source: &str,
    priority: &str,
    clear_by: Option<DateTime<Utc>>,
    created_by: Uuid,
) -> anyhow::Result<Condition> {
    let row = sqlx::query_as::<_, Condition>(&format!(
        r#"
        INSERT INTO loan_conditions (loan_id, company_id, title, detail,
                                     source, priority, clear_by, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING {COND_COLUMNS}
        "#
    ))
    .bind(loan_id)
    .bind(company_id)
    .bind(title)
    .bind(detail)
    .bind(source)
    .bind(priority)
    .bind(clear_by)
    .bind(created_by)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn set_status(
    pool: &PgPool,
    condition_id: Uuid,
    status: &str,
    cleared_by: Uuid,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE loan_conditions
        SET status = $2,
            cleared_at = CASE WHEN $2 IN ('cleared', 'waived') THEN now() ELSE NULL END,
            cleared_by = CASE WHEN $2 IN ('cleared', 'waived') THEN $3 ELSE NULL END,
            updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(condition_id)
    .bind(status)
    .bind(cleared_by)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn remove(pool: &PgPool, condition_id: Uuid) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM loan_conditions WHERE id = $1")
        .bind(condition_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// company_id + loan_id for audit entries.
pub async fn context_of(pool: &PgPool, condition_id: Uuid) -> anyhow::Result<(Uuid, Uuid)> {
    let row: (Uuid, Uuid) =
        sqlx::query_as("SELECT company_id, loan_id FROM loan_conditions WHERE id = $1")
            .bind(condition_id)
            .fetch_one(pool)
            .await?;
    Ok(row)
}
