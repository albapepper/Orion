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
    *state.current_user_id.lock().await = Some(user_id);
    Ok(account)
}
