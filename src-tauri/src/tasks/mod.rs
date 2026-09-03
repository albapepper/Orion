use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// A task on a loan. Joined with the assignee's name and the loan number
/// so the unified task view (and the AI) can show everything in one row
/// without a second round trip.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Task {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub screen: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub assigned_to_name: Option<String>,
    pub loan_number: String,
}

const TASK_COLUMNS: &str = r#"
        t.id, t.loan_id, t.company_id, t.title, t.description, t.assigned_to,
        t.screen, t.status, t.priority, t.due_date, t.created_by, t.created_at,
        t.completed_at, u.name AS assigned_to_name, l.loan_number
"#;

const TASK_FROM: &str = r#"
        FROM tasks t
        LEFT JOIN users u ON u.id = t.assigned_to
        JOIN loans l ON l.id = t.loan_id
"#;

/// Open first, then most urgent. The unified view's default order.
const TASK_ORDER: &str = r#"
        ORDER BY (t.status = 'done'),
                 (t.priority = 'high') DESC,
                 t.due_date ASC NULLS LAST,
                 t.created_at
"#;

pub async fn list_for_loan(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<Task>> {
    let rows = sqlx::query_as::<_, Task>(&format!(
        r#"SELECT {TASK_COLUMNS} {TASK_FROM}
           WHERE t.loan_id = $1
           {TASK_ORDER}"#
    ))
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Every task in the company, across all loans — the unified Tasks view.
pub async fn list_for_company(pool: &PgPool, company_id: Uuid) -> anyhow::Result<Vec<Task>> {
    let rows = sqlx::query_as::<_, Task>(&format!(
        r#"SELECT {TASK_COLUMNS} {TASK_FROM}
           WHERE t.company_id = $1
           {TASK_ORDER}"#
    ))
    .bind(company_id)
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
    description: Option<&str>,
    screen: Option<&str>,
    assigned_to: Option<Uuid>,
    due_date: Option<DateTime<Utc>>,
    priority: &str,
    created_by: Uuid,
) -> anyhow::Result<Task> {
    let row = sqlx::query_as::<_, Task>(&format!(
        r#"
        INSERT INTO tasks (loan_id, company_id, title, description, screen,
                           assigned_to, due_date, priority, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING {TASK_COLUMNS}
        "#
    ))
    .bind(loan_id)
    .bind(company_id)
    .bind(title)
    .bind(description)
    .bind(screen)
    .bind(assigned_to)
    .bind(due_date)
    .bind(priority)
    .bind(created_by)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn set_status(pool: &PgPool, task_id: Uuid, status: &str) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE tasks
        SET status = $2,
            completed_at = CASE WHEN $2 = 'done' THEN now() ELSE NULL END
        WHERE id = $1
        "#,
    )
    .bind(task_id)
    .bind(status)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn assign(pool: &PgPool, task_id: Uuid, assigned_to: Option<Uuid>) -> anyhow::Result<()> {
    sqlx::query("UPDATE tasks SET assigned_to = $2 WHERE id = $1")
        .bind(task_id)
        .bind(assigned_to)
        .execute(pool)
        .await?;
    Ok(())
}

/// company_id + loan_id for audit entries on status/assign changes.
pub async fn context_of(pool: &PgPool, task_id: Uuid) -> anyhow::Result<(Uuid, Uuid)> {
    let row: (Uuid, Uuid) =
        sqlx::query_as("SELECT company_id, loan_id FROM tasks WHERE id = $1")
            .bind(task_id)
            .fetch_one(pool)
            .await?;
    Ok(row)
}
