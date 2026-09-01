use super::AppState;
use crate::ai::{AiResponse, Message};
use crate::loans::demo;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn ai_chat(
    state: State<'_, AppState>,
    loan_id: String,
    screen: String,
    messages: Vec<Message>,
    thinking: Option<bool>,
) -> Result<AiResponse, String> {
    let loan_id = Uuid::parse_str(&loan_id).map_err(|e| e.to_string())?;

    // Build screen-appropriate context from current loan data
    let system_context = build_context_for_screen(&state, loan_id, &screen)
        .await
        .map_err(|e| e.to_string())?;

    let thinking = match thinking {
        Some(t) => t,
        None => state.config.read().await.ai.thinking,
    };

    // Call AI provider (local or cloud, doesn't matter here)
    let provider = state.provider().await;
    provider
        .complete(&system_context, &messages, thinking)
        .await
        .map_err(|e| e.to_string())
}

async fn build_context_for_screen(
    state: &State<'_, AppState>,
    loan_id: Uuid,
    screen: &str,
) -> anyhow::Result<String> {
    let db = state.db.read().await.clone();

    match screen {
        "income" => {
            let (income, borrower_name) = match db {
                Some(pool) => {
                    let income = crate::db::queries::get_income(&pool, loan_id).await?;
                    let borrowers = crate::db::queries::get_borrowers(&pool, loan_id).await?;
                    let name = borrowers
                        .iter()
                        .find(|b| b.is_primary)
                        .or(borrowers.first())
                        .map(|b| format!("{} {}", b.first_name, b.last_name))
                        .unwrap_or_else(|| "Unknown borrower".to_string());
                    (income, name)
                }
                None => {
                    let borrowers = demo::borrowers(loan_id);
                    let name = borrowers
                        .first()
                        .map(|b| format!("{} {}", b.first_name, b.last_name))
                        .unwrap_or_default();
                    (demo::income(loan_id), name)
                }
            };
            Ok(crate::loans::screens::income::build_context(
                &borrower_name,
                &income,
                "conventional",
            ))
        }
        _ => Ok(format!(
            "You are a concise, expert mortgage assistant inside the Orion \
             loan origination platform. Current screen: {screen}. Answer \
             questions relevant to this screen only."
        )),
    }
}
