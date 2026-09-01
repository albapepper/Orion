use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

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
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

pub async fn list_for_loan(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<Task>> {
    let rows = sqlx::query_as::<_, Task>(
        r#"
        SELECT id, loan_id, company_id, title, description, assigned_to,
               screen, status, created_by, created_at, completed_at
        FROM tasks WHERE loan_id = $1
        ORDER BY (status = 'done'), created_at
        "#,
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create(
    pool: &PgPool,
    loan_id: Uuid,
    company_id: Uuid,
    title: &str,
    description: Option<&str>,
    screen: Option<&str>,
    created_by: Uuid,
) -> anyhow::Result<Task> {
    let row = sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (loan_id, company_id, title, description, screen, created_by)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, loan_id, company_id, title, description, assigned_to,
                  screen, status, created_by, created_at, completed_at
        "#,
    )
    .bind(loan_id)
    .bind(company_id)
    .bind(title)
    .bind(description)
    .bind(screen)
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
