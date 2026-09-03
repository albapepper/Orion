use super::AppState;
use crate::documents::Document;
use tauri::State;
use uuid::Uuid;

fn parse_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_documents(
    state: State<'_, AppState>,
    loan_id: String,
) -> Result<Vec<Document>, String> {
    let db = state.db.read().await.clone();
    match db {
        Some(pool) => crate::documents::list_for_loan(&pool, parse_uuid(&loan_id)?)
            .await
            .map_err(|e| e.to_string()),
        None => Ok(crate::loans::demo::documents(parse_uuid(&loan_id)?)),
    }
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn add_document(
    state: State<'_, AppState>,
    loan_id: String,
    title: String,
    doc_type: Option<String>,
    screen: Option<String>,
    file_name: Option<String>,
    mime_type: Option<String>,
    size_bytes: Option<i64>,
) -> Result<Document, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let loan_id = parse_uuid(&loan_id)?;

    let doc = crate::documents::add(
        &pool,
        loan_id,
        company_id,
        &title,
        doc_type.as_deref().unwrap_or("other"),
        screen.as_deref(),
        file_name.as_deref(),
        mime_type.as_deref(),
        size_bytes,
        user_id,
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "document.add",
        "loan_documents",
        doc.id,
        None,
        Some(serde_json::json!({
            "title": doc.title,
            "doc_type": doc.doc_type,
            "screen": doc.screen,
        })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(doc)
}

#[tauri::command]
pub async fn remove_document(
    state: State<'_, AppState>,
    document_id: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let doc_id = parse_uuid(&document_id)?;

    let (company_id, loan_id) = crate::documents::context_of(&pool, doc_id)
        .await
        .map_err(|e| e.to_string())?;
    crate::documents::remove(&pool, doc_id)
        .await
        .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "document.remove",
        "loan_documents",
        doc_id,
        None,
        Some(serde_json::json!({ "removed": true })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
