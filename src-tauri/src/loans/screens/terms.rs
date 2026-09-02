use crate::loans::LoanTerms;

/// Monthly principal & interest, standard amortization. None when the
/// inputs needed for the math aren't there yet.
fn monthly_principal_interest(
    loan_amount: f64,
    annual_rate: f64,
    term_months: i32,
) -> Option<f64> {
    if loan_amount <= 0.0 || term_months <= 0 {
        return None;
    }
    let r = annual_rate / 100.0 / 12.0;
    let n = term_months as f64;
    if r <= 0.0 {
        return Some(loan_amount / n);
    }
    Some(loan_amount * r * (1.0 + r).powf(n) / ((1.0 + r).powf(n) - 1.0))
}

pub fn build_context(borrower_name: &str, terms: &LoanTerms) -> String {
    let ltv = match (terms.loan_amount, terms.property_value) {
        (Some(amount), Some(value)) if value > 0.0 => {
            format!("  LTV: {:.1}%", amount / value * 100.0)
        }
        _ => "  LTV: (need loan amount and property value)".to_string(),
    };

    let payment = match (terms.loan_amount, terms.interest_rate, terms.term_months) {
        (Some(amount), Some(rate), Some(months)) => {
            match monthly_principal_interest(amount, rate, months) {
                Some(p) => format!("  Est. monthly P&I: ${p:.0}"),
                None => String::new(),
            }
        }
        _ => String::new(),
    };

    format!(
        r#"You are an expert mortgage assistant helping a loan officer
on the Loan Terms screen.

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}
  Purpose: {} ({})
  Amortization: {} over {} months
  Loan amount: {}
  Note rate: {}
  Lien position: {}
  Property value: {}
{ltv}
{payment}

YOUR ROLE:
- Answer questions about this loan's structure (purpose, product, sizing)
- Flag issues: LTV above program limits, rate/term combinations worth
  questioning, missing structure fields
- Be concise — the user is in the middle of processing a loan

Do not discuss other screens or borrowers unless directly related to
these loan terms."#,
        terms.loan_purpose,
        terms.mortgage_type,
        terms.amortization_type,
        terms
            .term_months
            .map(|m| format!("{m} months"))
            .unwrap_or_else(|| "?".into()),
        terms
            .loan_amount
            .map(|a| format!("${a:.0}"))
            .unwrap_or_else(|| "not entered".into()),
        terms
            .interest_rate
            .map(|r| format!("{r:.3}%"))
            .unwrap_or_else(|| "not entered".into()),
        terms.lien_priority,
        terms
            .property_value
            .map(|v| format!("${v:.0}"))
            .unwrap_or_else(|| "not entered".into()),
    )
}
