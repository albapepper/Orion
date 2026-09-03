use super::AppState;
use crate::ai::{AiResponse, Message};
use crate::loans::demo;
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

    // Build screen-appropriate context from current loan data. The
    // dashboard and tasks screens are pipeline-level — no open loan.
    let system_context = if screen == "dashboard" {
        build_dashboard_context(&state)
            .await
            .map_err(|e| e.to_string())?
    } else if screen == "tasks" {
        build_tasks_context(&state)
            .await
            .map_err(|e| e.to_string())?
    } else {
        match loan_id {
            Some(id) => build_context_for_screen(&state, id, &screen)
                .await
                .map_err(|e| e.to_string())?,
            None => format!(
                "You are a concise, expert mortgage assistant inside the Orion \
                 loan origination platform. No loan is currently open. Current \
                 screen: {screen}. Help the user navigate their pipeline and \
                 answer general origination questions briefly."
            ),
        }
    };

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

async fn build_dashboard_context(state: &State<'_, AppState>) -> anyhow::Result<String> {
    let db = state.db.read().await.clone();
    let loans = match &db {
        Some(pool) => {
            let company_id = state
                .company_id()
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
            crate::db::queries::list_loans(pool, company_id).await?
        }
        None => demo::loan_summaries(),
    };
    Ok(crate::loans::screens::dashboard::build_context(
        &loans,
        chrono::Utc::now(),
    ))
}

async fn build_tasks_context(state: &State<'_, AppState>) -> anyhow::Result<String> {
    let db = state.db.read().await.clone();
    let tasks = match &db {
        Some(pool) => {
            let company_id = state
                .company_id()
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
            crate::tasks::list_for_company(pool, company_id).await?
        }
        None => demo::all_tasks(),
    };
    Ok(crate::loans::screens::tasks::build_context(
        &tasks,
        chrono::Utc::now(),
    ))
}

async fn build_context_for_screen(
    state: &State<'_, AppState>,
    loan_id: Uuid,
    screen: &str,
) -> anyhow::Result<String> {
    let db = state.db.read().await.clone();

    let borrower_name = |borrowers: &[crate::loans::Borrower]| {
        borrowers
            .iter()
            .find(|b| b.is_primary)
            .or(borrowers.first())
            .map(|b| format!("{} {}", b.first_name, b.last_name))
            .unwrap_or_else(|| "Unknown borrower".to_string())
    };

    match screen {
        "income" => {
            let (income, name) = match &db {
                Some(pool) => {
                    let income = crate::db::queries::get_income(pool, loan_id).await?;
                    let name = borrower_name(&crate::db::queries::get_borrowers(pool, loan_id).await?);
                    (income, name)
                }
                None => (demo::income(loan_id), borrower_name(&demo::borrowers(loan_id))),
            };
            Ok(crate::loans::screens::income::build_context(
                &name,
                &income,
                "conventional",
            ))
        }
        "terms" => {
            let terms = match &db {
                Some(pool) => {
                    crate::db::queries::get_loan_terms(pool, loan_id)
                        .await?
                        .unwrap_or_else(|| demo::loan_terms(loan_id))
                }
                None => demo::loan_terms(loan_id),
            };
            let name = match &db {
                Some(pool) => {
                    borrower_name(&crate::db::queries::get_borrowers(pool, loan_id).await?)
                }
                None => borrower_name(&demo::borrowers(loan_id)),
            };
            Ok(crate::loans::screens::terms::build_context(&name, &terms))
        }
        "borrower" => {
            let borrower = match &db {
                Some(pool) => {
                    let mut borrowers = crate::db::queries::get_borrowers(pool, loan_id).await?;
                    if borrowers.is_empty() {
                        borrowers = demo::borrowers(loan_id);
                    }
                    borrowers
                        .into_iter()
                        .find(|b| b.is_primary)
                        .unwrap_or_else(|| demo::borrowers(loan_id)[0].clone())
                }
                None => demo::borrowers(loan_id)[0].clone(),
            };
            let dependents = match &db {
                Some(pool) => crate::db::queries::get_dependents(pool, loan_id).await?,
                None => demo::dependents(loan_id),
            };
            Ok(crate::loans::screens::borrower::build_context(
                &borrower,
                &dependents,
            ))
        }
        "employment" => {
            let (name, employments) = match &db {
                Some(pool) => {
                    let borrowers = crate::db::queries::get_borrowers(pool, loan_id).await?;
                    let name = borrower_name(&borrowers);
                    let employments = crate::db::queries::get_employments(pool, loan_id).await?;
                    (name, employments)
                }
                None => (
                    borrower_name(&demo::borrowers(loan_id)),
                    demo::employments(loan_id),
                ),
            };
            Ok(crate::loans::screens::employment::build_context(
                &name,
                &employments,
            ))
        }
        "assets" => {
            let (name, assets) = match &db {
                Some(pool) => {
                    let borrowers = crate::db::queries::get_borrowers(pool, loan_id).await?;
                    let name = borrower_name(&borrowers);
                    let assets = crate::db::queries::get_assets(pool, loan_id).await?;
                    (name, assets)
                }
                None => (
                    borrower_name(&demo::borrowers(loan_id)),
                    demo::assets(loan_id),
                ),
            };
            Ok(crate::loans::screens::assets::build_context(&name, &assets))
        }
        "property" => {
            let (name, property) = match &db {
                Some(pool) => {
                    let borrowers = crate::db::queries::get_borrowers(pool, loan_id).await?;
                    let name = borrower_name(&borrowers);
                    let property = crate::db::queries::get_subject_property(pool, loan_id)
                        .await?
                        .unwrap_or_else(|| demo::subject_property(loan_id));
                    (name, property)
                }
                None => (
                    borrower_name(&demo::borrowers(loan_id)),
                    demo::subject_property(loan_id),
                ),
            };
            Ok(crate::loans::screens::property::build_context(&name, &property))
        }
        "declarations" => {
            let (name, declarations) = match &db {
                Some(pool) => {
                    let borrowers = crate::db::queries::get_borrowers(pool, loan_id).await?;
                    let name = borrower_name(&borrowers);
                    let declarations =
                        crate::db::queries::get_declarations(pool, loan_id).await?;
                    (name, declarations)
                }
                None => (
                    borrower_name(&demo::borrowers(loan_id)),
                    demo::declarations(loan_id),
                ),
            };
            Ok(crate::loans::screens::declarations::build_context(
                &name,
                &declarations,
            ))
        }
        "reo" => {
            let (name, reo) = match &db {
                Some(pool) => {
                    let borrowers = crate::db::queries::get_borrowers(pool, loan_id).await?;
                    let name = borrower_name(&borrowers);
                    let reo = crate::db::queries::get_reo(pool, loan_id).await?;
                    (name, reo)
                }
                None => (
                    borrower_name(&demo::borrowers(loan_id)),
                    demo::reo(loan_id),
                ),
            };
            Ok(crate::loans::screens::reo::build_context(&name, &reo))
        }
        "liabilities" => {
            let (name, liabilities, income) = match &db {
                Some(pool) => {
                    let borrowers = crate::db::queries::get_borrowers(pool, loan_id).await?;
                    let name = borrower_name(&borrowers);
                    let liabilities =
                        crate::db::queries::get_liabilities(pool, loan_id).await?;
                    let income = crate::db::queries::get_income(pool, loan_id).await?;
                    (name, liabilities, income)
                }
                None => (
                    borrower_name(&demo::borrowers(loan_id)),
                    demo::liabilities(loan_id),
                    demo::income(loan_id),
                ),
            };
            let total_monthly_income: f64 = income.iter().map(|i| i.monthly_amount).sum();
            Ok(crate::loans::screens::liabilities::build_context(
                &name,
                &liabilities,
                total_monthly_income,
            ))
        }
        _ => Ok(format!(
            "You are a concise, expert mortgage assistant inside the Orion \
             loan origination platform. Current screen: {screen}. Answer \
             questions relevant to this screen only."
        )),
    }
}
