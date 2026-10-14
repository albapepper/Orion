use super::AppState;
use crate::ai::{AiResponse, Message};
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn ai_chat(
    state: State<'_, AppState>,
    loan_id: Option<String>,
    screen: String,
    messages: Vec<Message>,
    thinking: Option<bool>,
) -> Result<AiResponse, String> {
    let loan_id = loan_id
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(Uuid::parse_str)
        .transpose()
        .map_err(|e| e.to_string())?;

    // Plugin-driven path: every registered screen (dashboard, tasks, and all
    // loan tabs) delegates context building to its plugin. Unregistered
    // screens (demographic/AUS/closing/funding/overview-adjacent) fall through
    // to the generic assistant below.
    let plugin = state.plugins.get(&screen);
    let base_context = if plugin.id() == screen {
        let ctx = crate::harness::PluginContext {
            state: state.inner(),
            loan_id,
            screen: &screen,
        };
        plugin.build_context(&ctx).await.map_err(|e| e.to_string())?
    } else if loan_id.is_none() {
        format!(
            "You are a concise, expert mortgage assistant inside the Orion \
             loan origination platform. No loan is currently open. Current \
             screen: {screen}. Help the user navigate their pipeline and \
             answer general origination questions briefly."
        )
    } else {
        format!(
            "You are a concise, expert mortgage assistant inside the Orion \
             loan origination platform. Current screen: {screen}. Answer \
             questions relevant to this screen only."
        )
    };

    let system_context = crate::prompt::build(state.inner(), loan_id, &screen, &base_context).await;

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

// Metadata: which plugins (screens) are registered and the tools each owns.
#[derive(serde::Serialize)]
pub struct PluginInfo {
    pub id: String,
    pub tools: Vec<String>,
}

#[tauri::command]
pub async fn list_plugins(
    state: State<'_, AppState>,
) -> Result<Vec<PluginInfo>, String> {
    let registry = &state.plugins;
    let mut out = Vec::with_capacity(registry.ids().len());
    for id in registry.ids() {
        let plugin = registry.get(id);
        out.push(PluginInfo {
            id: id.to_string(),
            tools: plugin.tools().iter().map(|t| (*t).to_string()).collect(),
        });
    }
    Ok(out)
}
