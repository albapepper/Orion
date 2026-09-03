use sqlx::PgPool;
use uuid::Uuid;

/// How long a lock survives without renewal. Every loan-screen load
/// renews it, so a lock dies ~this long after its holder closes Orion
/// or walks away.
const LOCK_TTL_MINUTES: i64 = 10;

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct LockInfo {
    pub loan_id: Uuid,
    pub locked_by: Uuid,
    pub user_name: String,
    pub screen: Option<String>,
}

/// First user in gets the lock; the same user re-acquiring renews it and
/// updates their screen (presence). A lock past its TTL is stale and gets
/// stolen. Returns true when the caller holds the lock.
pub async fn acquire_lock(
    pool: &PgPool,
    loan_id: Uuid,
    user_id: Uuid,
    screen: &str,
) -> anyhow::Result<bool> {
    let result = sqlx::query(
        r#"
        INSERT INTO loan_locks (loan_id, locked_by, screen)
        VALUES ($1, $2, $3)
        ON CONFLICT (loan_id) DO UPDATE SET
            locked_by = EXCLUDED.locked_by,
            screen = EXCLUDED.screen,
            locked_at = now()
        WHERE loan_locks.locked_by = EXCLUDED.locked_by
           OR loan_locks.locked_at < now() - make_interval(mins => $4)
        "#,
    )
    .bind(loan_id)
    .bind(user_id)
    .bind(screen)
    .bind(LOCK_TTL_MINUTES)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Extend the caller's own lock (heartbeat). No-op if someone else holds it.
pub async fn renew_lock(pool: &PgPool, loan_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
    sqlx::query("UPDATE loan_locks SET locked_at = now() WHERE loan_id = $1 AND locked_by = $2")
        .bind(loan_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn release_lock(pool: &PgPool, loan_id: Uuid, user_id: Uuid) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM loan_locks WHERE loan_id = $1 AND locked_by = $2")
        .bind(loan_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// The live lock, if any — stale locks read as free.
pub async fn get_lock(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Option<LockInfo>> {
    let row = sqlx::query_as::<_, LockInfo>(
        r#"
        SELECT ll.loan_id, ll.locked_by, ll.screen, u.name AS user_name
        FROM loan_locks ll
        JOIN users u ON u.id = ll.locked_by
        WHERE ll.loan_id = $1
          AND ll.locked_at > now() - make_interval(mins => $2)
        "#,
    )
    .bind(loan_id)
    .bind(LOCK_TTL_MINUTES)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// The server-side half of field locking: every save path calls this
/// before touching loan data. When another user holds the live lock,
/// the write is refused — the UI's read-only mode is the courtesy,
/// this is the enforcement.
pub async fn ensure_can_write(
    pool: &PgPool,
    loan_id: Uuid,
    user_id: Uuid,
) -> anyhow::Result<()> {
    if let Some(lock) = get_lock(pool, loan_id).await? {
        if lock.locked_by != user_id {
            return Err(anyhow::anyhow!(
                "{} is editing this loan — wait for them to finish",
                lock.user_name
            ));
        }
    }
    Ok(())
}
