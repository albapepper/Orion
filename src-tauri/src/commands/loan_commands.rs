use super::AppState;
use crate::loans::{demo, Borrower, BorrowerDependent, LoanIncome, LoanSummary, LoanTerms};
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
    pub dependents: Vec<BorrowerDependent>,
    pub employments: Vec<crate::loans::Employment>,
    pub income: Vec<LoanIncome>,
    pub assets: Vec<crate::loans::Asset>,
    pub liabilities: Vec<crate::loans::Liability>,
    pub property: Option<crate::loans::SubjectProperty>,
    pub reo: Vec<crate::loans::ReoProperty>,
    pub declarations: Vec<crate::loans::Declaration>,
    pub demographics: Vec<crate::loans::Demographics>,
    pub terms: Option<LoanTerms>,
    pub uw_reviews: Vec<crate::loans::UwReview>,
    pub documents: Vec<crate::documents::Document>,
    pub conditions: Vec<crate::conditions::Condition>,
    pub pricing: Vec<crate::pricing::LoanPricingOption>,
    pub aus: Vec<crate::aus::AusSubmission>,
    pub lock: Option<LockInfo>,
}

/// Audit payloads must never carry a full SSN — mask to last-4.
fn mask_ssn_in(payload: &mut serde_json::Value) {
    if let Some(obj) = payload.as_object_mut() {
        let masked = obj
            .get("ssn")
            .and_then(|v| v.as_str())
            .and_then(|s| s.get(s.len().saturating_sub(4)..))
            .map(|last4| format!("***-**-{last4}"));
        if let Some(m) = masked {
            obj.insert("ssn".into(), serde_json::Value::String(m));
        } else {
            obj.insert("ssn".into(), serde_json::Value::Null);
        }
    }
}

#[tauri::command]
pub async fn get_loan_detail(
    state: State<'_, AppState>,
    loan_id: String,
    screen: Option<String>,
) -> Result<LoanDetail, String> {
    let loan_id = parse_uuid(&loan_id)?;
    let db = state.db.read().await.clone();

    // Loading a loan screen IS the presence signal: it acquires the
    // lock (or renews ours and updates which screen we're on). The
    // returned `lock` tells the screen whether it may write.
    if let Some(pool) = &db {
        if let Some(screen) = screen.as_deref() {
            let user_id = *state.current_user_id.lock().await;
            if let Some(user_id) = user_id {
                let _ = crate::locks::acquire_lock(pool, loan_id, user_id, screen).await;
            }
        }
    }

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
            let dependents = crate::db::queries::get_dependents(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let employments = crate::db::queries::get_employments(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let assets = crate::db::queries::get_assets(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let liabilities = crate::db::queries::get_liabilities(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let property = crate::db::queries::get_subject_property(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let reo = crate::db::queries::get_reo(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let declarations = crate::db::queries::get_declarations(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let demographics = crate::db::queries::get_demographics(&pool, loan_id)
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
            let uw_reviews = crate::db::queries::get_uw_reviews(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let documents = crate::documents::list_for_loan(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let conditions = crate::conditions::list_for_loan(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let pricing = crate::pricing::list_loan_options(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            let aus = crate::aus::list_for_loan(&pool, loan_id)
                .await
                .map_err(|e| e.to_string())?;
            Ok(LoanDetail {
                summary,
                borrowers,
                dependents,
                employments,
                income,
                assets,
                liabilities,
                property,
                reo,
                declarations,
                demographics,
                terms,
                uw_reviews,
                documents,
                conditions,
                pricing,
                aus,
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
                dependents: demo::dependents(loan_id),
                employments: demo::employments(loan_id),
                income: demo::income(loan_id),
                assets: demo::assets(loan_id),
                liabilities: demo::liabilities(loan_id),
                property: Some(demo::subject_property(loan_id)),
                reo: demo::reo(loan_id),
                declarations: demo::declarations(loan_id),
                demographics: demo::demographics(loan_id),
                terms: Some(demo::loan_terms(loan_id)),
                uw_reviews: demo::uw_reviews(loan_id),
                documents: demo::documents(loan_id),
                conditions: demo::conditions(loan_id),
                pricing: demo::loan_pricing(loan_id),
                aus: demo::aus_submissions(loan_id),
                // Demo presence: loan 2 pretends Dana (the underwriter
                // persona) is holding it in Assets — switch personas in
                // Settings to see both sides of the lock.
                lock: (loan_id == demo::loan_id("00000000-0000-0000-0000-000000000002"))
                    .then(|| crate::locks::LockInfo {
                        loan_id,
                        locked_by: demo::underwriter_id(),
                        user_name: "Dana Reyes".into(),
                        screen: Some("assets".into()),
                    }),
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
    pub prior_year_monthly: Option<f64>,
    pub verified: bool,
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
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;

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
        income.prior_year_monthly,
        income.verified,
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
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;

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

#[derive(Deserialize)]
pub struct BorrowerInput {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub suffix_name: Option<String>,
    pub ssn: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub marital_status: Option<String>,
    pub citizenship_status: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[tauri::command]
pub async fn save_borrower(
    state: State<'_, AppState>,
    borrower_id: String,
    borrower: BorrowerInput,
) -> Result<Borrower, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let borrower_id = parse_uuid(&borrower_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    let borrower_loan: Option<(Uuid,)> =
        sqlx::query_as("SELECT loan_id FROM borrowers WHERE id = $1")
            .bind(borrower_id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| e.to_string())?;
    if let Some((loan_uuid,)) = borrower_loan {
        crate::locks::ensure_can_write(&pool, loan_uuid, user_id)
            .await
            .map_err(|e| e.to_string())?;
    }

    let saved = crate::db::queries::save_borrower(
        &pool,
        borrower_id,
        &borrower.first_name,
        &borrower.last_name,
        borrower.middle_name.as_deref(),
        borrower.suffix_name.as_deref(),
        borrower.ssn.as_deref(),
        borrower.birth_date,
        borrower.marital_status.as_deref(),
        borrower.citizenship_status.as_deref(),
        borrower.email.as_deref(),
        borrower.phone.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    let mut payload = serde_json::to_value(&saved).map_err(|e| e.to_string())?;
    mask_ssn_in(&mut payload);

    crate::audit::write(
        &pool,
        company_id,
        Some(saved.loan_id),
        user_id,
        "borrower.update",
        "borrowers",
        saved.id,
        None,
        Some(payload),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[derive(Deserialize)]
pub struct DependentsInput {
    pub borrower_id: String,
    /// One entry per dependent; age optional (URLA allows "not yet born").
    pub ages: Vec<Option<i32>>,
}

#[tauri::command]
pub async fn save_dependents(
    state: State<'_, AppState>,
    loan_id: String,
    input: DependentsInput,
) -> Result<Vec<BorrowerDependent>, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let borrower_id = parse_uuid(&input.borrower_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;

    let saved = crate::db::queries::replace_dependents(
        &pool,
        borrower_id,
        loan_id,
        company_id,
        &input.ages,
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "dependents.replace",
        "borrower_dependents",
        borrower_id,
        None,
        Some(serde_json::json!({ "dependent_count": saved.len() })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[derive(Deserialize)]
pub struct EmploymentInput {
    pub id: Option<String>,
    pub borrower_id: String,
    pub employer_name: Option<String>,
    pub position: Option<String>,
    pub self_employed: bool,
    pub monthly_income: Option<f64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_current: bool,
    pub months_in_line_of_work: Option<i32>,
    pub business_phone: Option<String>,
}

#[tauri::command]
pub async fn save_employment(
    state: State<'_, AppState>,
    loan_id: String,
    employment: EmploymentInput,
) -> Result<crate::loans::Employment, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let borrower_id = parse_uuid(&employment.borrower_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;
    let record_id = employment.id.as_deref().map(parse_uuid).transpose()?;

    let saved = crate::db::queries::upsert_employment(
        &pool,
        record_id,
        borrower_id,
        loan_id,
        company_id,
        employment.employer_name.as_deref(),
        employment.position.as_deref(),
        employment.self_employed,
        employment.monthly_income,
        employment.start_date,
        employment.end_date,
        employment.is_current,
        employment.months_in_line_of_work,
        employment.business_phone.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        if record_id.is_some() {
            "employment.update"
        } else {
            "employment.create"
        },
        "borrower_employments",
        saved.id,
        None,
        Some(serde_json::to_value(&saved).map_err(|e| e.to_string())?),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[tauri::command]
pub async fn delete_employment(
    state: State<'_, AppState>,
    employment_id: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let employment_id = parse_uuid(&employment_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;

    // Audit captures what was removed, before it's gone.
    let before = crate::db::queries::get_employment(&pool, employment_id)
        .await
        .map_err(|e| e.to_string())?;
    if let Some(owned) = &before {
        crate::locks::ensure_can_write(&pool, owned.loan_id, user_id)
            .await
            .map_err(|e| e.to_string())?;
    }

    crate::db::queries::delete_employment(&pool, employment_id)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(removed) = before {
        crate::audit::write(
            &pool,
            company_id,
            Some(removed.loan_id),
            user_id,
            "employment.delete",
            "borrower_employments",
            removed.id,
            Some(serde_json::to_value(&removed).map_err(|e| e.to_string())?),
            None,
        )
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[derive(Deserialize)]
pub struct AssetInput {
    pub id: Option<String>,
    pub borrower_id: String,
    pub account_type: String,
    pub institution: Option<String>,
    pub balance: f64,
    pub gift_donor: Option<String>,
    pub gift_relationship: Option<String>,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn save_asset(
    state: State<'_, AppState>,
    loan_id: String,
    asset: AssetInput,
) -> Result<crate::loans::Asset, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let borrower_id = parse_uuid(&asset.borrower_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;
    let record_id = asset.id.as_deref().map(parse_uuid).transpose()?;

    let saved = crate::db::queries::upsert_asset(
        &pool,
        record_id,
        borrower_id,
        loan_id,
        company_id,
        &asset.account_type,
        asset.institution.as_deref(),
        asset.balance,
        asset.gift_donor.as_deref(),
        asset.gift_relationship.as_deref(),
        asset.notes.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        if record_id.is_some() { "asset.update" } else { "asset.create" },
        "loan_assets",
        saved.id,
        None,
        Some(serde_json::to_value(&saved).map_err(|e| e.to_string())?),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[tauri::command]
pub async fn delete_asset(
    state: State<'_, AppState>,
    asset_id: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let asset_id = parse_uuid(&asset_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;

    let before = crate::db::queries::get_asset(&pool, asset_id)
        .await
        .map_err(|e| e.to_string())?;
    if let Some(owned) = &before {
        crate::locks::ensure_can_write(&pool, owned.loan_id, user_id)
            .await
            .map_err(|e| e.to_string())?;
    }

    crate::db::queries::delete_asset(&pool, asset_id)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(removed) = before {
        crate::audit::write(
            &pool,
            company_id,
            Some(removed.loan_id),
            user_id,
            "asset.delete",
            "loan_assets",
            removed.id,
            Some(serde_json::to_value(&removed).map_err(|e| e.to_string())?),
            None,
        )
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[derive(Deserialize)]
pub struct LiabilityInput {
    pub id: Option<String>,
    pub borrower_id: String,
    pub liability_type: String,
    pub creditor_name: Option<String>,
    pub monthly_payment: Option<f64>,
    pub unpaid_balance: Option<f64>,
    pub payoff_at_close: bool,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn save_liability(
    state: State<'_, AppState>,
    loan_id: String,
    liability: LiabilityInput,
) -> Result<crate::loans::Liability, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let borrower_id = parse_uuid(&liability.borrower_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;
    let record_id = liability.id.as_deref().map(parse_uuid).transpose()?;

    let saved = crate::db::queries::upsert_liability(
        &pool,
        record_id,
        borrower_id,
        loan_id,
        company_id,
        &liability.liability_type,
        liability.creditor_name.as_deref(),
        liability.monthly_payment,
        liability.unpaid_balance,
        liability.payoff_at_close,
        liability.notes.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        if record_id.is_some() { "liability.update" } else { "liability.create" },
        "loan_liabilities",
        saved.id,
        None,
        Some(serde_json::to_value(&saved).map_err(|e| e.to_string())?),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[tauri::command]
pub async fn delete_liability(
    state: State<'_, AppState>,
    liability_id: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let liability_id = parse_uuid(&liability_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;

    let before = crate::db::queries::get_liability(&pool, liability_id)
        .await
        .map_err(|e| e.to_string())?;
    if let Some(owned) = &before {
        crate::locks::ensure_can_write(&pool, owned.loan_id, user_id)
            .await
            .map_err(|e| e.to_string())?;
    }
    crate::db::queries::delete_liability(&pool, liability_id)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(removed) = before {
        crate::audit::write(
            &pool,
            company_id,
            Some(removed.loan_id),
            user_id,
            "liability.delete",
            "loan_liabilities",
            removed.id,
            Some(serde_json::to_value(&removed).map_err(|e| e.to_string())?),
            None,
        )
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[derive(Deserialize)]
pub struct PropertyInput {
    pub address_line: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub property_type: String,
    pub occupancy: String,
    pub year_built: Option<i32>,
    pub units: i32,
    pub estimated_value: Option<f64>,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn save_subject_property(
    state: State<'_, AppState>,
    loan_id: String,
    property: PropertyInput,
) -> Result<crate::loans::SubjectProperty, String> {
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

    let saved = crate::db::queries::upsert_subject_property(
        &pool,
        loan_id,
        company_id,
        property.address_line.as_deref(),
        property.city.as_deref(),
        property.state.as_deref(),
        property.postal_code.as_deref(),
        &property.property_type,
        &property.occupancy,
        property.year_built,
        property.units,
        property.estimated_value,
        property.notes.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "property.update",
        "loan_subject_property",
        saved.id,
        None,
        Some(serde_json::to_value(&saved).map_err(|e| e.to_string())?),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[derive(Deserialize)]
pub struct ReoInput {
    pub id: Option<String>,
    pub borrower_id: String,
    pub address_line: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub property_type: String,
    pub market_value: Option<f64>,
    pub mortgage_payment: Option<f64>,
    pub rental_income: Option<f64>,
    pub disposition: String,
    pub notes: Option<String>,
}

#[tauri::command]
pub async fn save_reo(
    state: State<'_, AppState>,
    loan_id: String,
    record: ReoInput,
) -> Result<crate::loans::ReoProperty, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let borrower_id = parse_uuid(&record.borrower_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;
    let record_id = record.id.as_deref().map(parse_uuid).transpose()?;

    let saved = crate::db::queries::upsert_reo(
        &pool,
        record_id,
        borrower_id,
        loan_id,
        company_id,
        record.address_line.as_deref(),
        record.city.as_deref(),
        record.state.as_deref(),
        record.postal_code.as_deref(),
        &record.property_type,
        record.market_value,
        record.mortgage_payment,
        record.rental_income,
        &record.disposition,
        record.notes.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        if record_id.is_some() { "reo.update" } else { "reo.create" },
        "loan_properties_reo",
        saved.id,
        None,
        Some(serde_json::to_value(&saved).map_err(|e| e.to_string())?),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[tauri::command]
pub async fn delete_reo(
    state: State<'_, AppState>,
    reo_id: String,
) -> Result<(), String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let reo_id = parse_uuid(&reo_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;

    let before = crate::db::queries::get_reo_by_id(&pool, reo_id)
        .await
        .map_err(|e| e.to_string())?;
    if let Some(owned) = &before {
        crate::locks::ensure_can_write(&pool, owned.loan_id, user_id)
            .await
            .map_err(|e| e.to_string())?;
    }
    crate::db::queries::delete_reo(&pool, reo_id)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(removed) = before {
        crate::audit::write(
            &pool,
            company_id,
            Some(removed.loan_id),
            user_id,
            "reo.delete",
            "loan_properties_reo",
            removed.id,
            Some(serde_json::to_value(&removed).map_err(|e| e.to_string())?),
            None,
        )
        .await
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[derive(Deserialize)]
pub struct DeclarationInput {
    pub borrower_id: String,
    pub question_key: String,
    pub answer: Option<bool>,
    pub explanation: Option<String>,
}

#[tauri::command]
pub async fn save_declaration(
    state: State<'_, AppState>,
    loan_id: String,
    declaration: DeclarationInput,
) -> Result<crate::loans::Declaration, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let borrower_id = parse_uuid(&declaration.borrower_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;

    let saved = crate::db::queries::upsert_declaration(
        &pool,
        borrower_id,
        loan_id,
        company_id,
        &declaration.question_key,
        declaration.answer,
        declaration.explanation.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "declaration.update",
        "loan_declarations",
        saved.id,
        None,
        Some(serde_json::to_value(&saved).map_err(|e| e.to_string())?),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[derive(Deserialize)]
pub struct DemographicsInput {
    pub borrower_id: String,
    pub ethnicity: Option<String>,
    pub race: Option<String>,
    pub sex: Option<String>,
    pub collection_basis: Option<String>,
}

#[tauri::command]
pub async fn save_demographics(
    state: State<'_, AppState>,
    loan_id: String,
    input: DemographicsInput,
) -> Result<crate::loans::Demographics, String> {
    let pool = state.pool().await?;
    let company_id = state.company_id().await?;
    let loan_id = parse_uuid(&loan_id)?;
    let borrower_id = parse_uuid(&input.borrower_id)?;
    let user_id = state
        .current_user_id
        .lock()
        .await
        .ok_or("No user signed in")?;
    crate::locks::ensure_can_write(&pool, loan_id, user_id)
        .await
        .map_err(|e| e.to_string())?;

    let saved = crate::db::queries::upsert_demographics(
        &pool,
        borrower_id,
        loan_id,
        company_id,
        input.ethnicity.as_deref(),
        input.race.as_deref(),
        input.sex.as_deref(),
        input.collection_basis.as_deref(),
    )
    .await
    .map_err(|e| e.to_string())?;

    // HMDA data is sensitive: the audit entry records THAT a change
    // happened, never the values (cross-cutting rule 4).
    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "demographics.update",
        "borrower_demographics",
        saved.id,
        None,
        Some(serde_json::json!({ "record": "updated", "fields": null })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}

#[derive(Deserialize)]
pub struct UwReviewInput {
    pub screen: String,
    pub status: String,
    pub note: Option<String>,
}

/// The underwriter column of a loan screen. Field locking is dynamic and
/// enforced here — the UI hides the editors, but the backend refuses the
/// write unless the signed-in user is an underwriter or admin.
#[tauri::command]
pub async fn save_uw_review(
    state: State<'_, AppState>,
    loan_id: String,
    review: UwReviewInput,
) -> Result<crate::loans::UwReview, String> {
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

    let role = crate::db::queries::get_user_role(&pool, user_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Signed-in user no longer exists")?;
    if role != "underwriter" && role != "admin" {
        return Err("Only underwriters can update the review column".into());
    }

    let saved = crate::db::queries::upsert_uw_review(
        &pool,
        loan_id,
        company_id,
        &review.screen,
        &review.status,
        review.note.as_deref(),
        user_id,
    )
    .await
    .map_err(|e| e.to_string())?;

    // The note content stays out of the audit trail — status change is
    // what matters for the record.
    crate::audit::write(
        &pool,
        company_id,
        Some(loan_id),
        user_id,
        "uw_review.update",
        "loan_uw_reviews",
        saved.id,
        None,
        Some(serde_json::json!({ "screen": review.screen, "status": review.status })),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(saved)
}
