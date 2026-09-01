use sqlx::PgPool;
use uuid::Uuid;

/// Append one row to the immutable audit log. Every data mutation calls this.
#[allow(clippy::too_many_arguments)]
pub async fn write(
    pool: &PgPool,
    company_id: Uuid,
    loan_id: Option<Uuid>,
    user_id: Uuid,
    action: &str,
    table_name: &str,
    record_id: Uuid,
    old_value: Option<serde_json::Value>,
    new_value: Option<serde_json::Value>,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO audit_log
            (company_id, loan_id, user_id, action, table_name, record_id, old_value, new_value)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(company_id)
    .bind(loan_id)
    .bind(user_id)
    .bind(action)
    .bind(table_name)
    .bind(record_id)
    .bind(old_value)
    .bind(new_value)
    .execute(pool)
    .await?;
    Ok(())
}
