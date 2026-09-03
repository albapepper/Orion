use super::AppState;
use crate::config::Config;
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
pub struct AppStatus {
    pub configured: bool,
    pub db_connected: bool,
    pub ai_provider: String,
    pub ai_model: String,
    pub company_name: Option<String>,
}

#[tauri::command]
pub async fn get_status(state: State<'_, AppState>) -> Result<AppStatus, String> {
    let config = state.config.read().await;
    let db = state.db.read().await.clone();
    let db_connected = db.is_some();
    // The company name rides the status so the nav tray can show who
    // this install belongs to.
    let company_name = match (&db, uuid::Uuid::parse_str(&config.app.company_id)) {
        (Some(pool), Ok(id)) => crate::db::queries::get_company(pool, id)
            .await
            .map_err(|e| e.to_string())?
            .map(|c| c.name),
        (None, _) => Some("Demo Company".to_string()),
        _ => None,
    };
    Ok(AppStatus {
        configured: config.is_configured(),
        db_connected,
        ai_provider: config.ai.provider.clone(),
        ai_model: config.ai.model.clone(),
        company_name,
    })
}

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<Config, String> {
    Ok(state.config.read().await.clone())
}

/// Persist config, rebuild the AI provider, and (re)connect the database.
#[tauri::command]
pub async fn save_config(state: State<'_, AppState>, config: Config) -> Result<(), String> {
    config.save().map_err(|e| e.to_string())?;

    *state.ai.write().await = crate::ai::build_provider(&config.ai);

    if !config.database.url.is_empty() {
        match crate::db::connect(&config.database.url).await {
            Ok(pool) => *state.db.write().await = Some(pool),
            Err(e) => {
                *state.config.write().await = config;
                return Err(format!("Config saved but database connection failed: {e}"));
            }
        }
    }

    *state.config.write().await = config;
    Ok(())
}

/// Wizard "Test Connection" for the database path.
#[tauri::command]
pub async fn test_db_connection(url: String) -> Result<String, String> {
    let pool = crate::db::connect(&url).await.map_err(|e| e.to_string())?;
    let version: (String,) = sqlx::query_as("SELECT version()")
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(version.0)
}

/// Wizard "Deploy Schema" — connect and run all migrations.
#[tauri::command]
pub async fn deploy_schema(state: State<'_, AppState>, url: String) -> Result<(), String> {
    let pool = crate::db::connect(&url).await.map_err(|e| e.to_string())?;
    crate::db::run_migrations(&pool)
        .await
        .map_err(|e| e.to_string())?;
    *state.db.write().await = Some(pool);
    Ok(())
}

/// Wizard "Test Connection" for the AI path — round-trips a tiny prompt.
#[tauri::command]
pub async fn test_ai_connection(state: State<'_, AppState>) -> Result<String, String> {
    let provider = state.provider().await;
    let messages = [crate::ai::Message {
        role: "user".into(),
        content: "Reply with the single word: ready".into(),
    }];
    let resp = provider
        .complete("You are a connection test.", &messages, false)
        .await
        .map_err(|e| e.to_string())?;
    Ok(resp.content)
}

// ── Setup wizard: company step (P5-01) ───────────────────────────────

#[tauri::command]
pub async fn list_companies(state: State<'_, AppState>) -> Result<Vec<crate::db::queries::Company>, String> {
    let pool = state.pool().await?;
    crate::db::queries::list_companies(&pool)
        .await
        .map_err(|e| e.to_string())
}

/// Create the tenant row and make it the configured company in the
/// same motion — the wizard's "Create company" button.
#[tauri::command]
pub async fn create_company(
    state: State<'_, AppState>,
    name: String,
) -> Result<crate::db::queries::Company, String> {
    let pool = state.pool().await?;
    let name = name.trim();
    if name.is_empty() {
        return Err("Company name is required".into());
    }
    let company = crate::db::queries::create_company(&pool, name)
        .await
        .map_err(|e| e.to_string())?;

    let mut cfg = state.config.read().await.clone();
    cfg.app.company_id = company.id.to_string();
    cfg.save().map_err(|e| e.to_string())?;
    *state.config.write().await = cfg;

    if let Some(user_id) = *state.current_user_id.lock().await {
        crate::audit::write(
            &pool,
            company.id,
            None,
            user_id,
            "company.create",
            "companies",
            company.id,
            None,
            Some(serde_json::json!({ "name": company.name })),
        )
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(company)
}

/// Point the config at an existing company (wizard "use existing").
#[tauri::command]
pub async fn set_company(state: State<'_, AppState>, company_id: String) -> Result<(), String> {
    let id = uuid::Uuid::parse_str(&company_id).map_err(|e| e.to_string())?;
    {
        let pool = state.pool().await?;
        crate::db::queries::get_company(&pool, id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Company not found")?;
    }
    let mut cfg = state.config.read().await.clone();
    cfg.app.company_id = id.to_string();
    cfg.save().map_err(|e| e.to_string())?;
    *state.config.write().await = cfg;
    Ok(())
}

/// Rename the company — P5-02's settings surface. Admin only.
#[tauri::command]
pub async fn rename_company(
    state: State<'_, AppState>,
    name: String,
) -> Result<crate::db::queries::Company, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let role = crate::db::queries::get_user_role(&pool, user_id)
        .await
        .map_err(|e| e.to_string())?;
    if role.as_deref() != Some("admin") {
        return Err("Only admins can rename the company".into());
    }
    let name = name.trim();
    if name.is_empty() {
        return Err("Company name is required".into());
    }

    let company = crate::db::queries::rename_company(&pool, company_id, name)
        .await
        .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        None,
        user_id,
        "company.rename",
        "companies",
        company.id,
        None,
        Some(serde_json::json!({ "name": company.name })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(company)
}
