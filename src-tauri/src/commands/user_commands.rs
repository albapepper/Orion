// Identity commands. There are no passwords — the desktop app "signs in
// as" a real user row, which is what locks, audit entries, and dynamic
// field permissions bind to.

use super::AppState;
use crate::loans::{demo, UserAccount};
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn list_users(state: State<'_, AppState>) -> Result<Vec<UserAccount>, String> {
    let db = state.db.read().await.clone();
    match db {
        Some(pool) => {
            let company_id = state.company_id().await?;
            crate::db::queries::list_users(&pool, company_id)
                .await
                .map_err(|e| e.to_string())
        }
        None => Ok(demo::users()),
    }
}

#[tauri::command]
pub async fn whoami(state: State<'_, AppState>) -> Result<Option<UserAccount>, String> {
    let user_id = *state.current_user_id.lock().await;
    let db = state.db.read().await.clone();
    match (user_id, db) {
        (Some(id), Some(pool)) => crate::db::queries::get_user(&pool, id)
            .await
            .map_err(|e| e.to_string()),
        // Demo mode always has a persona so the app is explorable on first run.
        (None, None) => Ok(Some(demo::default_user())),
        (Some(id), None) => Ok(demo::users().into_iter().find(|u| u.id == id)),
        // Connected but nobody signed in yet — Settings offers the picker.
        (None, Some(_)) => Ok(None),
    }
}

#[tauri::command]
pub async fn sign_in_as(
    state: State<'_, AppState>,
    user_id: String,
) -> Result<UserAccount, String> {
    let user_id = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    let db = state.db.read().await.clone();
    let account = match db {
        Some(pool) => crate::db::queries::get_user(&pool, user_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("No such user")?,
        None => demo::users()
            .into_iter()
            .find(|u| u.id == user_id)
            .ok_or("No such user")?,
    };
    if !account.active {
        return Err("This user has been deactivated".into());
    }
    *state.current_user_id.lock().await = Some(user_id);
    Ok(account)
}

// ── Team management (P5-03) — admin only ─────────────────────────────

/// The one server-side gate on team changes: only admins manage the
/// team. Returns the admin's id for the audit row.
async fn require_admin(
    state: &State<'_, AppState>,
    pool: &sqlx::PgPool,
) -> Result<Uuid, String> {
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let role = crate::db::queries::get_user_role(pool, user_id)
        .await
        .map_err(|e| e.to_string())?;
    match role.as_deref() {
        Some("admin") => Ok(user_id),
        _ => Err("Only admins can manage the team".into()),
    }
}

const ROLES: &[&str] = &["loan_officer", "processor", "underwriter", "admin"];

fn check_role(role: &str) -> Result<(), String> {
    if ROLES.contains(&role) {
        Ok(())
    } else {
        Err(format!("Unknown role: {role}"))
    }
}

#[tauri::command]
pub async fn create_user(
    state: State<'_, AppState>,
    email: String,
    name: String,
    role: String,
    phone: Option<String>,
    title: Option<String>,
) -> Result<UserAccount, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let admin_id = require_admin(&state, &pool).await?;
    check_role(&role)?;
    let email = email.trim().to_lowercase();
    if !email.contains('@') {
        return Err("A valid email is required".into());
    }
    let name = name.trim();
    if name.is_empty() {
        return Err("A name is required".into());
    }

    let account = crate::db::queries::create_user(
        &pool,
        company_id,
        &email,
        name,
        &role,
        phone.as_deref().filter(|p| !p.is_empty()),
        title.as_deref().filter(|t| !t.is_empty()),
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        None,
        admin_id,
        "user.create",
        "users",
        account.id,
        None,
        Some(serde_json::json!({ "email": account.email, "role": account.role })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(account)
}

#[tauri::command]
pub async fn update_user(
    state: State<'_, AppState>,
    user_id: String,
    name: String,
    role: String,
    phone: Option<String>,
    title: Option<String>,
) -> Result<UserAccount, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let admin_id = require_admin(&state, &pool).await?;
    let user_id = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    check_role(&role)?;

    let account = crate::db::queries::update_user(
        &pool,
        user_id,
        name.trim(),
        &role,
        phone.as_deref().filter(|p| !p.is_empty()),
        title.as_deref().filter(|t| !t.is_empty()),
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        None,
        admin_id,
        "user.update",
        "users",
        account.id,
        None,
        Some(serde_json::json!({ "role": account.role })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(account)
}

#[tauri::command]
pub async fn set_user_active(
    state: State<'_, AppState>,
    user_id: String,
    active: bool,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let admin_id = require_admin(&state, &pool).await?;
    let user_id = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;
    if user_id == admin_id && !active {
        return Err("You can't deactivate yourself".into());
    }

    crate::db::queries::set_user_active(&pool, user_id, active)
        .await
        .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        None,
        admin_id,
        "user.active",
        "users",
        user_id,
        None,
        Some(serde_json::json!({ "active": active })),
    )
    .await
    .map_err(|e| e.to_string())
}
