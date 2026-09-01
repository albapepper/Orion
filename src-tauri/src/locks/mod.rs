use sqlx::PgPool;
use uuid::Uuid;

pub async fn acquire_lock(
    pool: &PgPool,
    loan_id: Uuid,
    user_id: Uuid,
    screen: &str,
) -> anyhow::Result<bool> {
    // First user in gets the lock; re-entry by the same user updates their screen.
    let result = sqlx::query(
        r#"
        INSERT INTO loan_locks (loan_id, locked_by, screen)
        VALUES ($1, $2, $3)
        ON CONFLICT (loan_id) DO UPDATE SET screen = EXCLUDED.screen
        WHERE loan_locks.locked_by = EXCLUDED.locked_by
        "#,
    )
    .bind(loan_id)
    .bind(user_id)
    .bind(screen)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn release_lock(pool: &PgPool, loan_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM loan_locks WHERE loan_id = $1 AND locked_by = $2")
        .bind(loan_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_lock(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Option<LockInfo>> {
    let row = sqlx::query_as::<_, LockInfo>(
        r#"
        SELECT ll.loan_id, ll.locked_by, ll.screen, u.name AS user_name
        FROM loan_locks ll
        JOIN users u ON u.id = ll.locked_by
        WHERE ll.loan_id = $1
        "#,
    )
    .bind(loan_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct LockInfo {
    pub loan_id: Uuid,
    pub locked_by: Uuid,
    pub user_name: String,
    pub screen: Option<String>,
}
