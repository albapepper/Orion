use super::AppState;
use crate::tasks::Task;
use tauri::State;
use uuid::Uuid;

fn parse_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_tasks(state: State<'_, AppState>, loan_id: String) -> Result<Vec<Task>, String> {
    let db = state.db.read().await.clone();
    match db {
        Some(pool) => crate::tasks::list_for_loan(&pool, parse_uuid(&loan_id)?)
            .await
            .map_err(|e| e.to_string()),
        None => Ok(Vec::new()),
    }
}

#[tauri::command]
pub async fn create_task(
    state: State<'_, AppState>,
    loan_id: String,
    title: String,
    description: Option<String>,
    screen: Option<String>,
) -> Result<Task, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;

    crate::tasks::create(
        &pool,
        parse_uuid(&loan_id)?,
        company_id,
        &title,
        description.as_deref(),
        screen.as_deref(),
        user_id,
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_task_status(
    state: State<'_, AppState>,
    task_id: String,
    status: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    crate::tasks::set_status(&pool, parse_uuid(&task_id)?, &status)
        .await
        .map_err(|e| e.to_string())
}
