use super::AppState;
use crate::aus::AusSubmission;
use tauri::State;
use uuid::Uuid;

fn parse_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

// ── AUS: DU sandbox (P4-05); LPA (P4-06) rides the same table ────────

/// Submit the serialized loan to DU. With a configured endpoint the
/// MISMO document goes to the sandbox and whatever comes back is
/// parsed best-effort; without one, the mock engine scores the file's
/// own numbers. Submission is a loan-affecting write: lock-checked
/// and audited.
#[tauri::command]
pub async fn submit_du(state: State<'_, AppState>, loan_id: String) -> Result<AusSubmission, String> {
    let loan_id = parse_uuid(&loan_id)?;
    let db = state.db.read().await.clone();

    // Serialize whatever the file holds right now.
    let detail = super::loan_commands::get_loan_detail(state.clone(), loan_id.to_string(), None)
        .await?;
    let input = crate::mismo::MismoInput {
        loan_number: detail.summary.loan_number.as_str(),
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
    };
    let xml = crate::mismo::serialize(&input);

    let endpoint = state.config.read().await.aus.du_endpoint.clone();

    match (&db, endpoint.is_empty()) {
        // Demo mode: mock findings, nothing persisted.
        (None, _) => {
            let facts = crate::aus::FileFacts {
                loan_number: detail.summary.loan_number.as_str(),
                terms: detail.terms.as_ref(),
                borrowers: &detail.borrowers,
                income: &detail.income,
                liabilities: &detail.liabilities,
                declarations: &detail.declarations,
            };
            let (recommendation, case_id, messages) = crate::aus::mock_findings(&facts);
            Ok(AusSubmission {
                id: Uuid::new_v4(),
                loan_id,
                company_id: Uuid::nil(),
                system: "du-mock".into(),
                case_id,
                recommendation: Some(recommendation),
                messages: serde_json::to_value(messages).map_err(|e| e.to_string())?,
                raw_response: None,
                submitted_by: None,
                submitted_at: chrono::Utc::now(),
            })
        }
        // Real mode, no endpoint: mock findings, persisted as du-mock.
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

            let facts = crate::aus::FileFacts {
                loan_number: detail.summary.loan_number.as_str(),
                terms: detail.terms.as_ref(),
                borrowers: &detail.borrowers,
                income: &detail.income,
                liabilities: &detail.liabilities,
                declarations: &detail.declarations,
            };
            let (recommendation, case_id, messages) = crate::aus::mock_findings(&facts);
            let sub = crate::aus::insert(
                pool,
                loan_id,
                company_id,
                "du-mock",
                case_id.as_deref(),
                Some(&recommendation),
                serde_json::to_value(messages).map_err(|e| e.to_string())?,
                None,
                user_id,
            )
            .await
            .map_err(|e| e.to_string())?;
            audit_submit(pool, &state, company_id, loan_id, user_id, &sub.id).await?;
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
                .map_err(|e| format!("DU endpoint unreachable: {e}"))?;
            let status = resp.status();
            let body = resp.text().await.map_err(|e| e.to_string())?;
            if !status.is_success() {
                return Err(format!("DU endpoint returned HTTP {status}: {body}"));
            }

            let (recommendation, messages) = crate::aus::parse_response("DU", &body);
            let sub = crate::aus::insert(
                pool,
                loan_id,
                company_id,
                "du",
                None,
                recommendation.as_deref(),
                serde_json::to_value(messages).map_err(|e| e.to_string())?,
                Some(&body),
                user_id,
            )
            .await
            .map_err(|e| e.to_string())?;
            audit_submit(pool, &state, company_id, loan_id, user_id, &sub.id).await?;
            Ok(sub)
        }
    }
}

async fn audit_submit(
    pool: &sqlx::PgPool,
    _state: &State<'_, AppState>,
    company_id: Uuid,
    loan_id: Uuid,
    user_id: Uuid,
    submission_id: &Uuid,
) -> Result<(), String> {
    crate::audit::write(
        pool,
        company_id,
        Some(loan_id),
        user_id,
        "aus.submit_du",
        "aus_submissions",
        *submission_id,
        None,
        None,
    )
    .await
    .map_err(|e| e.to_string())
}
