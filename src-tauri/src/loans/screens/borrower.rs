use crate::loans::{Borrower, BorrowerDependent};
use chrono::Datelike;

/// Borrower profile context. SSN is deliberately excluded — sensitive
/// data never leaves the building via the AI path (see
/// docs/1003-fields.md cross-cutting rule 4).
pub fn build_context(borrower: &Borrower, dependents: &[BorrowerDependent]) -> String {
    let age = borrower.birth_date.map(|d| {
        let today = chrono::Local::now().date_naive();
        let mut years = today.year() - d.year();
        if (today.month(), today.day()) < (d.month(), d.day()) {
            years -= 1;
        }
        years
    });

    let ssn_status = if borrower.ssn.is_some() {
        "on file (not shown here)"
    } else {
        "missing — needed before credit pull"
    };

    let dependent_line = if dependents.is_empty() {
        "  Dependents: none".to_string()
    } else {
        let ages: Vec<String> = dependents
            .iter()
            .map(|d| d.age.map(|a| a.to_string()).unwrap_or_else(|| "unborn".into()))
            .collect();
        format!(
            "  Dependents: {} (ages: {})",
            dependents.len(),
            ages.join(", ")
        )
    };

    format!(
        r#"You are an expert mortgage assistant helping a loan officer
on the Borrower Info screen.

CURRENT LOAN CONTEXT:
  Borrower: {} {} {} {}
  Age: {}
  SSN: {ssn_status}
  Marital status: {}
  Citizenship: {}
  Contact: email {}, phone {}
{dependent_line}

YOUR ROLE:
- Answer questions about this borrower's profile
- Flag missing or inconsistent identity fields (e.g. SSN or DOB absent,
  marital status implying a co-borrower that isn't on the loan)
- Be concise — the user is in the middle of processing a loan

Never request or repeat the borrower's full SSN. Do not discuss other
screens unless directly related to this borrower profile."#,
        borrower.first_name,
        borrower.middle_name.as_deref().unwrap_or(""),
        borrower.last_name,
        borrower.suffix_name.as_deref().unwrap_or(""),
        age.map(|a| a.to_string()).unwrap_or_else(|| "unknown".into()),
        borrower
            .marital_status
            .as_deref()
            .unwrap_or("not entered"),
        borrower
            .citizenship_status
            .as_deref()
            .unwrap_or("not entered"),
        borrower.email.as_deref().unwrap_or("none"),
        borrower.phone.as_deref().unwrap_or("none"),
    )
}
