use super::AppState;
use crate::mismo::MismoInput;
use serde::Serialize;
use tauri::State;
use uuid::Uuid;

fn parse_uuid(s: &str) -> Result<Uuid, String> {
    Uuid::parse_str(s).map_err(|e| e.to_string())
}

#[derive(Serialize)]
pub struct MismoExport {
    /// The full MISMO 3.4 document, ready for a GSE sandbox (P4-05/06).
    pub xml: String,
    /// Gaps between this loan and a submittable 1003 — advice shown
    /// next to the export, not a gate.
    pub missing: Vec<String>,
}

/// The LO contact block rides on the loan summary (P3-17). Borrowed
/// references stay valid: the summary outlives the serializer call.
fn originator_from_summary<'a>(detail: &'a super::loan_commands::LoanDetail) -> Option<crate::mismo::Originator<'a>> {
    detail.summary.assigned_to_name.as_deref().map(|name| crate::mismo::Originator {
        name,
        email: detail.summary.assigned_to_email.as_deref(),
        phone: detail.summary.assigned_to_phone.as_deref(),
        title: detail.summary.assigned_to_title.as_deref(),
    })
}

/// Serialize a complete loan to MISMO 3.4 XML (P4-04). Read-only on
/// the file itself, but exporting a 1003 is a compliance event, so the
/// real path writes an audit row — with no payload, just the event.
#[tauri::command]
pub async fn export_loan_mismo(
    state: State<'_, AppState>,
    loan_id: String,
) -> Result<MismoExport, String> {
    let loan_id = parse_uuid(&loan_id)?;
    // Reuse the detail loader: same fetches, same demo fallback, and
    // passing no screen keeps it from touching the lock state.
    let detail = super::loan_commands::get_loan_detail(state.clone(), loan_id.to_string(), None)
        .await?;

    let input = MismoInput {
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
        originator: originator_from_summary(&detail),
    };
    let export = MismoExport {
        xml: crate::mismo::serialize(&input),
        missing: crate::mismo::completeness(&input),
    };

    if let Some(pool) = state.db.read().await.clone() {
        let company_id = state.company_id().await?;
        if let Some(user_id) = *state.current_user_id.lock().await {
            crate::audit::write(
                &pool,
                company_id,
                Some(loan_id),
                user_id,
                "mismo.export",
                "loans",
                loan_id,
                None,
                None,
            )
            .await
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(export)
}

/// Write the exported XML to the user's Downloads folder. The XML and
/// name arrive from the frontend; the name is sanitized to a bare
/// file name so nothing escapes the target directory.
#[tauri::command]
pub async fn save_mismo_file(xml: String, file_name: String) -> Result<String, String> {
    // Reject anything path-shaped — only a flat file name gets through.
    let bare: String = file_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | ' '))
        .collect();
    let bare = bare.trim().trim_matches('.');
    if bare.is_empty() {
        return Err("Bad file name".into());
    }
    let base = if bare.ends_with(".xml") {
        bare.to_string()
    } else {
        format!("{bare}.xml")
    };

    let dir = dirs::download_dir()
        .or_else(dirs::home_dir)
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    let mut path = dir.join(&base);
    // Never overwrite an existing export.
    let mut n = 1;
    while path.exists() {
        let stem = base.trim_end_matches(".xml");
        path = dir.join(format!("{stem}-{n}.xml"));
        n += 1;
    }

    std::fs::write(&path, xml).map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().into_owned())
}
