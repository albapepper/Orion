use super::AppState;
use crate::aus::{AusEngine, AusSubmission};
use tauri::State;
use uuid::Uuid;

fn parse_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

// ── AUS: DU (P4-05) and LPA (P4-06) sandboxes ────────────────────────
// One shared flow, two engines: the serialized loan goes out, findings
// come back into the same table. With a configured endpoint the MISMO
// document goes to the sandbox and whatever comes back is parsed
// best-effort; without one, the mock engine scores the file's own
// numbers. Submission is a loan-affecting write: lock-checked and
// audited.

#[tauri::command]
pub async fn submit_du(state: State<'_, AppState>, loan_id: String) -> Result<AusSubmission, String> {
    submit(state, loan_id, AusEngine::Du).await
}

#[tauri::command]
pub async fn submit_lpa(
    state: State<'_, AppState>,
    loan_id: String,
) -> Result<AusSubmission, String> {
    submit(state, loan_id, AusEngine::Lpa).await
}

async fn submit(
    state: State<'_, AppState>,
    loan_id: String,
    engine: AusEngine,
) -> Result<AusSubmission, String> {
    let loan_id = parse_uuid(&loan_id)?;
    let db = state.db.read().await.clone();

    // Serialize whatever the file holds right now.
    let detail = super::loan_commands::get_loan_detail(state.clone(), loan_id.to_string(), None)
        .await?;
    let input = crate::mismo::MismoInput {
        loan_number: &detail.summary.loan_number,
        terms: detail.terms.as_ref(),
        borrowers: &detail.borrowers,
        dependents: &detail.dependents,
        employments: &detail.employments,
        income: &detail.income,
        assets: &detail.assets,
        liabilities: &detail.liabilities,
        property: detail.property.as_ref(),
        reo: &detail.reo,
        declarations: &detail.declarations,
        demographics: &detail.demographics,
        originator: detail.summary.assigned_to_name.as_deref().map(|name| {
            crate::mismo::Originator {
                name,
                email: detail.summary.assigned_to_email.as_deref(),
                phone: detail.summary.assigned_to_phone.as_deref(),
                title: detail.summary.assigned_to_title.as_deref(),
            }
        }),
    };
    let xml = crate::mismo::serialize(&input);

    let endpoint = match engine {
        AusEngine::Du => state.config.read().await.aus.du_endpoint.clone(),
        AusEngine::Lpa => state.config.read().await.aus.lpa_endpoint.clone(),
    };

    match (&db, endpoint.is_empty()) {
        // Demo mode: mock findings, nothing persisted.
        (None, _) => {
            let outcome = mock(&detail, engine);
            Ok(AusSubmission {
                id: Uuid::new_v4(),
                loan_id,
                company_id: Uuid::nil(),
                system: outcome.system.into(),
                case_id: outcome.case_id,
                recommendation: Some(outcome.recommendation),
                messages: serde_json::to_value(outcome.messages).map_err(|e| e.to_string())?,
                raw_response: None,
                submitted_by: None,
                submitted_at: chrono::Utc::now(),
            })
        }
        // Real mode, no endpoint: mock findings, persisted as a mock row.
        (Some(pool), true) => {
            let company_id = state.company_id().await?;
            let user_id = state
                .current_user_id
                .lock()
                .await
                .ok_or("No user signed in")?;
            crate::locks::ensure_can_write(pool, loan_id, user_id)
                .await
                .map_err(|e| e.to_string())?;

            let outcome = mock(&detail, engine);
            let sub = crate::aus::insert(
                pool,
                loan_id,
                company_id,
                outcome.system,
                outcome.case_id.as_deref(),
                Some(&outcome.recommendation),
                serde_json::to_value(outcome.messages).map_err(|e| e.to_string())?,
                None,
                user_id,
            )
            .await
            .map_err(|e| e.to_string())?;
            audit_submit(pool, company_id, loan_id, user_id, &sub.id, engine).await?;
            Ok(sub)
        }
        // Real mode with endpoint: POST the MISMO document.
        (Some(pool), false) => {
            let company_id = state.company_id().await?;
            let user_id = state
                .current_user_id
                .lock()
                .await
                .ok_or("No user signed in")?;
            crate::locks::ensure_can_write(pool, loan_id, user_id)
                .await
                .map_err(|e| e.to_string())?;

            let client = reqwest::Client::new();
            let resp = client
                .post(&endpoint)
                .header("Content-Type", "application/xml")
                .body(xml)
                .timeout(std::time::Duration::from_secs(120))
                .send()
                .await
                .map_err(|e| format!("{} endpoint unreachable: {e}", engine.label().to_uppercase()))?;
            let status = resp.status();
            let body = resp.text().await.map_err(|e| e.to_string())?;
            if !status.is_success() {
                return Err(format!(
                    "{} endpoint returned HTTP {status}: {body}",
                    engine.label().to_uppercase()
                ));
            }

            let (recommendation, messages) =
                crate::aus::parse_response(engine.label().to_uppercase().as_str(), &body);
            let sub = crate::aus::insert(
                pool,
                loan_id,
                company_id,
                engine.label(),
                None,
                recommendation.as_deref(),
                serde_json::to_value(messages).map_err(|e| e.to_string())?,
                Some(&body),
                user_id,
            )
            .await
            .map_err(|e| e.to_string())?;
            audit_submit(pool, company_id, loan_id, user_id, &sub.id, engine).await?;
            Ok(sub)
        }
    }
}

fn mock(detail: &super::loan_commands::LoanDetail, engine: AusEngine) -> crate::aus::MockOutcome {
    let facts = crate::aus::FileFacts {
        loan_number: detail.summary.loan_number.as_str(),
        terms: detail.terms.as_ref(),
        borrowers: &detail.borrowers,
        income: &detail.income,
        liabilities: &detail.liabilities,
        declarations: &detail.declarations,
    };
    crate::aus::mock_findings(engine, &facts)
}

async fn audit_submit(
    pool: &sqlx::PgPool,
    company_id: Uuid,
    loan_id: Uuid,
    user_id: Uuid,
    submission_id: &Uuid,
    engine: AusEngine,
) -> Result<(), String> {
    crate::audit::write(
        pool,
        company_id,
        Some(loan_id),
        user_id,
        &format!("aus.submit_{}", engine.label()),
        "aus_submissions",
        *submission_id,
        None,
        None,
    )
    .await
    .map_err(|e| e.to_string())
}
