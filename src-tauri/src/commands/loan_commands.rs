use super::AppState;
use crate::loans::{demo, Borrower, LoanIncome, LoanSummary, LoanTerms};
use crate::locks::LockInfo;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

fn parse_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_loans(state: State<'_, AppState>) -> Result<Vec<LoanSummary>, String> {
    let db = state.db.read().await.clone();
    match db {
        Some(pool) => {
            let company_id = state.company_id().await?;
            crate::db::queries::list_loans(&pool, company_id)
                .await
                .map_err(|e| e.to_string())
        }
        None => Ok(demo::loan_summaries()),
    }
}

#[derive(Serialize)]
pub struct LoanDetail {
    pub summary: LoanSummary,
    pub borrowers: Vec<Borrower>,
    pub income: Vec<LoanIncome>,
    pub terms: Option<LoanTerms>,
    pub lock: Option<LockInfo>,
}

#[tauri::command]
pub async fn get_loan_detail(
    state: State<'_, AppState>,
    loan_id: String,
) -> Result<LoanDetail, String> {
    let loan_id = parse_uuid(&loan_id)?;
    let db = state.db.read().await.clone();

    match db {
        Some(pool) => {
            let company_id = state.company_id().await?;
            let summaries = crate::db::queries::list_loans(&pool, company_id)
                .await
                .map_err(|e| e.to_string())?;
            let summary = summaries
                .into_iter()
                .find(|l| l.id == loan_id)
                .ok_or("Loan not found")?;
            let borrowers = crate::db::queries::get_borrowers(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let income = crate::db::queries::get_income(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let terms = crate::db::queries::get_loan_terms(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let lock = crate::locks::get_lock(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            Ok(LoanDetail {
                summary,
                borrowers,
                income,
                terms,
                lock,
            })
        }
        None => {
            let summary = demo::loan_summaries()
                .into_iter()
                .find(|l| l.id == loan_id)
                .ok_or("Loan not found")?;
            Ok(LoanDetail {
                summary,
                borrowers: demo::borrowers(loan_id),
                income: demo::income(loan_id),
                terms: Some(demo::loan_terms(loan_id)),
                lock: None,
            })
        }
    }
}

#[derive(Deserialize)]
pub struct IncomeInput {
    pub id: Option<String>,
    pub borrower_id: String,
    pub income_type: String,
    pub monthly_amount: f64,
    pub employer_name: Option<String>,
    pub years_on_job: Option<f64>,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn save_income(
    state: State<'_, AppState>,
    loan_id: String,
    income: IncomeInput,
) -> Result<LoanIncome, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;

    let record_id = income.id.as_deref().map(parse_uuid).transpose()?;
    let borrower_id = parse_uuid(&income.borrower_id)?;

    let saved = crate::db::queries::upsert_income(
        &pool,
        record_id,
        loan_id,
        company_id,
        borrower_id,
        &income.income_type,
        income.monthly_amount,
        income.employer_name.as_deref(),
        income.years_on_job,
        income.notes.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "income.update",
        "loan_income",
        saved.id,
        None,
        Some(serde_json::to_value(&saved).map_err(|e| e.to_string())?),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[tauri::command]
pub async fn acquire_loan_lock(
    state: State<'_, AppState>,
    loan_id: String,
    screen: String,
) -> Result<bool, String> {
    let pool = state.pool().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    crate::locks::acquire_lock(&pool, loan_id, user_id, &screen)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn release_loan_lock(state: State<'_, AppState>, loan_id: String) -> Result<(), String> {
    let pool = state.pool().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    crate::locks::release_lock(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_loan_lock(
    state: State<'_, AppState>,
    loan_id: String,
) -> Result<Option<LockInfo>, String> {
    let pool = state.pool().await?;
    let loan_id = parse_uuid(&loan_id)?;
    crate::locks::get_lock(&pool, loan_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_loan_terms(
    state: State<'_, AppState>,
    loan_id: String,
) -> Result<Option<LoanTerms>, String> {
    let loan_id = parse_uuid(&loan_id)?;
    let db = state.db.read().await.clone();
    match db {
        Some(pool) => crate::db::queries::get_loan_terms(&pool, loan_id)
            .await
            .map_err(|e| e.to_string()),
        None => Ok(Some(demo::loan_terms(loan_id))),
    }
}

#[derive(Deserialize)]
pub struct TermsInput {
    pub loan_purpose: String,
    pub mortgage_type: String,
    pub amortization_type: String,
    pub loan_amount: Option<f64>,
    pub interest_rate: Option<f64>,
    pub term_months: Option<i32>,
    pub lien_priority: String,
    pub property_value: Option<f64>,
    pub application_date: Option<NaiveDate>,
}

#[tauri::command]
pub async fn save_loan_terms(
    state: State<'_, AppState>,
    loan_id: String,
    terms: TermsInput,
) -> Result<LoanTerms, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;

    let saved = crate::db::queries::upsert_loan_terms(
        &pool,
        loan_id,
        company_id,
        &terms.loan_purpose,
        &terms.mortgage_type,
        &terms.amortization_type,
        terms.loan_amount,
        terms.interest_rate,
        terms.term_months,
        &terms.lien_priority,
        terms.property_value,
        terms.application_date,
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "terms.update",
        "loan_terms",
        saved.id,
        None,
        Some(serde_json::to_value(&saved).map_err(|e| e.to_string())?),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}
