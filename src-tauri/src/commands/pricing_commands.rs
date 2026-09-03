use super::AppState;
use crate::pricing::{RateOption, RateSheet};
use tauri::State;
use uuid::Uuid;

fn parse_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

// Dates arrive as plain YYYY-MM-DD from <input type="date">.
fn parse_date(s: &str) -> Result<chrono::NaiveDate, String> {
    chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| format!("Bad date: {e}"))
}

#[tauri::command]
pub async fn list_rate_sheets(state: State<'_, AppState>) -> Result<Vec<RateSheet>, String> {
    let db = state.db.read().await.clone();
    match db {
        Some(pool) => {
            let company_id = state.company_id().await?;
            crate::pricing::list_sheets(&pool, company_id)
                .await
                .map_err(|e| e.to_string())
        }
        None => Ok(crate::loans::demo::rate_sheets()),
    }
}

#[tauri::command]
pub async fn list_rate_options(
    state: State<'_, AppState>,
    rate_sheet_id: String,
) -> Result<Vec<RateOption>, String> {
    let db = state.db.read().await.clone();
    match db {
        Some(pool) => crate::pricing::list_options(&pool, parse_uuid(&rate_sheet_id)?)
            .await
            .map_err(|e| e.to_string()),
        None => Ok(crate::loans::demo::rate_options(parse_uuid(&rate_sheet_id)?)),
    }
}

#[tauri::command]
pub async fn create_rate_sheet(
    state: State<'_, AppState>,
    lender: String,
    effective_date: Option<String>,
    expires_date: Option<String>,
) -> Result<RateSheet, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let effective = effective_date.as_deref().filter(|s| !s.is_empty()).map(parse_date).transpose()?;
    let expires = expires_date.as_deref().filter(|s| !s.is_empty()).map(parse_date).transpose()?;

    let sheet = crate::pricing::create_sheet(&pool, company_id, &lender, effective, expires)
        .await
        .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        None,
        user_id,
        "rate_sheet.create",
        "rate_sheets",
        sheet.id,
        None,
        Some(serde_json::json!({ "lender": sheet.lender })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(sheet)
}

#[tauri::command]
pub async fn set_rate_sheet_active(
    state: State<'_, AppState>,
    rate_sheet_id: String,
    active: bool,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let sheet_id = parse_uuid(&rate_sheet_id)?;

    crate::pricing::set_active(&pool, company_id, sheet_id, active)
        .await
        .map_err(|e| e.to_string())?;

    let user_id = state.current_user_id.lock().await;
    if let Some(user_id) = *user_id {
        crate::audit::write(
            &pool,
            company_id,
            None,
            user_id,
            "rate_sheet.active",
            "rate_sheets",
            sheet_id,
            None,
            Some(serde_json::json!({ "active": active })),
        )
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn add_rate_option(
    state: State<'_, AppState>,
    rate_sheet_id: String,
    product: Option<String>,
    rate: f64,
    points: f64,
    comp_plan: Option<String>,
) -> Result<RateOption, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let sheet_id = parse_uuid(&rate_sheet_id)?;

    let option = crate::pricing::add_option(
        &pool,
        sheet_id,
        company_id,
        product.as_deref().unwrap_or("30yr_fixed"),
        rate,
        points,
        comp_plan.as_deref(),
        None,
        None,
    )
    .await
    .map_err(|e| e.to_string())?;

    if let Some(user_id) = *state.current_user_id.lock().await {
        crate::audit::write(
            &pool,
            company_id,
            None,
            user_id,
            "rate_option.add",
            "rate_sheet_options",
            option.id,
            None,
            Some(serde_json::json!({
                "product": option.product,
                "rate": option.rate,
                "points": option.points,
            })),
        )
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(option)
}

#[tauri::command]
pub async fn remove_rate_option(
    state: State<'_, AppState>,
    rate_option_id: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    crate::pricing::remove_option(&pool, parse_uuid(&rate_option_id)?)
        .await
        .map_err(|e| e.to_string())
}
