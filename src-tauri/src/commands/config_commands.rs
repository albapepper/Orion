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
}

#[tauri::command]
pub async fn get_status(state: State<'_, AppState>) -> Result<AppStatus, String> {
    let config = state.config.read().await;
    let db_connected = state.db.read().await.is_some();
    Ok(AppStatus {
        configured: config.is_configured(),
        db_connected,
        ai_provider: config.ai.provider.clone(),
        ai_model: config.ai.model.clone(),
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
