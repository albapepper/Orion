use super::AppState;
use crate::tasks::Task;
use tauri::State;
use uuid::Uuid;

fn parse_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

// Due dates arrive as plain YYYY-MM-DD from <input type="date">; stored
// as midnight UTC.
fn parse_due_date(s: &str) -> Result<chrono::DateTime<chrono::Utc>, String> {
    let date = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| format!("Bad due date: {e}"))?;
    let dt = date
        .and_hms_opt(0, 0, 0)
        .ok_or("Bad due date")?;
    Ok(dt.and_utc())
}

#[tauri::command]
pub async fn list_tasks(state: State<'_, AppState>, loan_id: String) -> Result<Vec<Task>, String> {
    let db = state.db.read().await.clone();
    match db {
        Some(pool) => crate::tasks::list_for_loan(&pool, parse_uuid(&loan_id)?)
            .await
            .map_err(|e| e.to_string()),
        None => Ok(crate::loans::demo::tasks(parse_uuid(&loan_id)?)),
    }
}

/// The unified Tasks view: every task on every loan in the company.
#[tauri::command]
pub async fn list_all_tasks(state: State<'_, AppState>) -> Result<Vec<Task>, String> {
    let db = state.db.read().await.clone();
    match db {
        Some(pool) => {
            let company_id = state.company_id().await?;
            crate::tasks::list_for_company(&pool, company_id)
                .await
                .map_err(|e| e.to_string())
        }
        None => Ok(crate::loans::demo::all_tasks()),
    }
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn create_task(
    state: State<'_, AppState>,
    loan_id: String,
    title: String,
    description: Option<String>,
    screen: Option<String>,
    assigned_to: Option<String>,
    due_date: Option<String>,
    priority: Option<String>,
) -> Result<Task, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;

    let loan_id = parse_uuid(&loan_id)?;
    let assigned_to = assigned_to.map(|s| parse_uuid(&s)).transpose()?;    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;

    let due = match due_date.as_deref() {
        Some(s) if !s.is_empty() => Some(parse_due_date(s)?),
        _ => None,
    };
    let priority = priority.as_deref().unwrap_or("normal").to_string();
    if !matches!(priority.as_str(), "low" | "normal" | "high") {
        return Err("Priority must be low, normal, or high".into());
    }

    let task = crate::tasks::create(
        &pool,
        loan_id,
        company_id,
        &title,
        description.as_deref(),
        screen.as_deref(),
        assigned_to,
        due,
        &priority,
        user_id,
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "task.create",
        "tasks",
        task.id,
        None,
        Some(serde_json::json!({
            "title": task.title,
            "priority": task.priority,
            "assigned_to": task.assigned_to,
        })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(task)
}

#[tauri::command]
pub async fn set_task_status(
    state: State<'_, AppState>,
    task_id: String,
    status: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let task_id = parse_uuid(&task_id)?;
    if !matches!(status.as_str(), "open" | "in_progress" | "done") {
        return Err("Status must be open, in_progress, or done".into());
    }

    let (company_id, loan_id) = crate::tasks::context_of(&pool, task_id)
        .await
        .map_err(|e| e.to_string())?;
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;
    crate::tasks::set_status(&pool, task_id, &status)
        .await
        .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "task.status",
        "tasks",
        task_id,
        None,
        Some(serde_json::json!({ "status": status })),
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn assign_task(
    state: State<'_, AppState>,
    task_id: String,
    assigned_to: Option<String>,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let task_id = parse_uuid(&task_id)?;
    let assigned_to = assigned_to.map(|s| parse_uuid(&s)).transpose()?;

    let (company_id, loan_id) = crate::tasks::context_of(&pool, task_id)
        .await
        .map_err(|e| e.to_string())?;
    crate::tasks::assign(&pool, task_id, assigned_to)
        .await
        .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "task.assign",
        "tasks",
        task_id,
        None,
        Some(serde_json::json!({ "assigned_to": assigned_to })),
    )
    .await
    .map_err(|e| e.to_string())
}
