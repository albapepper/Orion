use super::AppState;
use tauri::State;
use uuid::Uuid;

fn parse_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

/// The panel's data — a light read that doesn't drag the whole loan
/// detail along.
#[tauri::command]
pub async fn list_contacts(
    state: State<'_, AppState>,
    loan_id: String,
) -> Result<Vec<crate::loans::LoanContact>, String> {
    let db = state.db.read().await.clone();
    let loan_id = parse_uuid(&loan_id)?;
    match db {
        Some(pool) => crate::contacts::list_for_loan(&pool, loan_id)
            .await
            .map_err(|e| e.to_string()),
        None => Ok(crate::loans::demo::loan_contacts(loan_id)),
    }
}

/// Add a contact to the loan (P5-09). Lock-checked and audited.
#[tauri::command]
pub async fn add_loan_contact(
    state: State<'_, AppState>,
    loan_id: String,
    contact: crate::loans::LoanContact,
) -> Result<crate::loans::LoanContact, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;

    let name = contact.name.trim();
    if name.is_empty() {
        return Err("Contact name is required".into());
    }

    let saved = crate::contacts::insert(
        &pool,
        loan_id,
        company_id,
        &contact.role,
        name,
        contact.company.as_deref().filter(|c| !c.is_empty()),
        contact.email.as_deref().filter(|c| !c.is_empty()),
        contact.phone.as_deref().filter(|c| !c.is_empty()),
        contact.notes.as_deref().filter(|c| !c.is_empty()),
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "contact.add",
        "loan_contacts",
        saved.id,
        None,
        Some(serde_json::json!({ "name": saved.name, "role": saved.role })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[tauri::command]
pub async fn remove_loan_contact(
    state: State<'_, AppState>,
    contact_id: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let contact_id = parse_uuid(&contact_id)?;

    let row: Option<(Uuid, Uuid)> =
        sqlx::query_as("SELECT company_id, loan_id FROM loan_contacts WHERE id = $1")
            .bind(contact_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| e.to_string())?;
    let Some((company_id, loan_id)) = row else {
        return Ok(());
    };

    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;
    crate::contacts::remove(&pool, contact_id)
        .await
        .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "contact.remove",
        "loan_contacts",
        contact_id,
        None,
        Some(serde_json::json!({ "removed": true })),
    )
    .await
    .map_err(|e| e.to_string())
}
