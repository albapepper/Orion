use super::AppState;
use crate::conditions::Condition;
use tauri::State;
use uuid::Uuid;

fn parse_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

// Due-style inputs arrive as plain YYYY-MM-DD from <input type="date">.
fn parse_clear_by(s: &str) -> Result<chrono::DateTime<chrono::Utc>, String> {
    let date = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| format!("Bad clear-by date: {e}"))?;
    let dt = date.and_hms_opt(0, 0, 0).ok_or("Bad clear-by date")?;
    Ok(dt.and_utc())
}

#[tauri::command]
pub async fn list_conditions(
    state: State<'_, AppState>,
    loan_id: String,
) -> Result<Vec<Condition>, String> {
    let db = state.db.read().await.clone();
    match db {
        Some(pool) => crate::conditions::list_for_loan(&pool, parse_uuid(&loan_id)?)
            .await
            .map_err(|e| e.to_string()),
        None => Ok(crate::loans::demo::conditions(parse_uuid(&loan_id)?)),
    }
}

#[tauri::command]
pub async fn create_condition(
    state: State<'_, AppState>,
    loan_id: String,
    title: String,
    detail: Option<String>,
    source: Option<String>,
    priority: Option<String>,
    clear_by: Option<String>,
) -> Result<Condition, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let loan_id = parse_uuid(&loan_id)?;

    let source = source.as_deref().unwrap_or("underwriter").to_string();
    if !matches!(source.as_str(), "underwriter" | "processor" | "system") {
        return Err("Source must be underwriter, processor, or system".into());
    }
    let priority = priority.as_deref().unwrap_or("normal").to_string();
    if !matches!(priority.as_str(), "low" | "normal" | "high") {
        return Err("Priority must be low, normal, or high".into());
    }
    let clear_by = match clear_by.as_deref() {
        Some(s) if !s.is_empty() => Some(parse_clear_by(s)?),
        _ => None,
    };

    let condition = crate::conditions::create(
        &pool,
        loan_id,
        company_id,
        &title,
        detail.as_deref(),
        &source,
        &priority,
        clear_by,
        user_id,
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "condition.create",
        "loan_conditions",
        condition.id,
        None,
        Some(serde_json::json!({
            "title": condition.title,
            "source": condition.source,
            "priority": condition.priority,
        })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(condition)
}

#[tauri::command]
pub async fn set_condition_status(
    state: State<'_, AppState>,
    condition_id: String,
    status: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let condition_id = parse_uuid(&condition_id)?;
    if !matches!(
        status.as_str(),
        "open" | "submitted" | "cleared" | "waived"
    ) {
        return Err("Status must be open, submitted, cleared, or waived".into());
    }

    let (company_id, loan_id) = crate::conditions::context_of(&pool, condition_id)
        .await
        .map_err(|e| e.to_string())?;
    crate::conditions::set_status(&pool, condition_id, &status, user_id)
        .await
        .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "condition.status",
        "loan_conditions",
        condition_id,
        None,
        Some(serde_json::json!({ "status": status })),
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_condition(
    state: State<'_, AppState>,
    condition_id: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let condition_id = parse_uuid(&condition_id)?;

    let (company_id, loan_id) = crate::conditions::context_of(&pool, condition_id)
        .await
        .map_err(|e| e.to_string())?;
    crate::conditions::remove(&pool, condition_id)
        .await
        .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "condition.remove",
        "loan_conditions",
        condition_id,
        None,
        Some(serde_json::json!({ "removed": true })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
