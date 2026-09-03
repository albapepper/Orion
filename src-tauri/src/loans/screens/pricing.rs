use crate::loans::LoanTerms;
use crate::pricing::LoanPricingOption;

/// Thousands-separated dollars (Rust format strings have no grouping).
/// Shared with the pricing AI so suggested and manual rows render alike.
pub(crate) fn money(v: f64) -> String {
    let s = format!("{v:.0}");
    let bytes = s.as_bytes();
    let mut grouped = String::with_capacity(s.len() + s.len() / 3);
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(*b as char);
    }
    format!("${grouped}")
}

/// The Pricing screen context: the deal shape plus the options built so
/// far. P4-03 layers constrained AI option suggestions on top of this.
pub fn build_context(
    borrower_name: &str,
    terms: Option<&LoanTerms>,
    options: &[LoanPricingOption],
) -> String {
    let terms_block = match terms {
        Some(t) => {
            let amount = t.loan_amount.map(money).unwrap_or_else(|| "not set".into());
            let rate = t
                .interest_rate
                .map(|r| format!("{r:.3}%"))
                .unwrap_or_else(|| "not set".into());
            let term = t
                .term_months
                .map(|m| format!("{m} months"))
                .unwrap_or_else(|| "not set".into());
            let value = t.property_value.map(money).unwrap_or_else(|| "not set".into());
            let ltv = match (t.loan_amount, t.property_value) {
                (Some(a), Some(v)) if v > 0.0 => format!(" ({:.1}% LTV)", a / v * 100.0),
                _ => String::new(),
            };
            format!(
                "  Loan amount: {amount}\n  Note rate on file: {rate}\n  Term: {term}\n  Property value: {value}{ltv}"
            )
        }
        None => "  (loan terms not filled in yet — math will be incomplete)".to_string(),
    };

    let options_block = if options.is_empty() {
        "  (no options built yet)".to_string()
    } else {
        options
            .iter()
            .map(|o| {
                format!(
                    "  - [{}] {} at {:.3}%, {} points{} — P&I {}, points cost {}",
                    if o.selected { "SELECTED" } else { "        " },
                    o.product.replace('_', " "),
                    o.rate,
                    o.points,
                    o.comp_plan
                        .as_deref()
                        .map(|c| format!(" ({c})"))
                        .unwrap_or_default(),
                    o.monthly_pi.map(money).unwrap_or_else(|| "?".into()),
                    o.points_cost.map(money).unwrap_or_else(|| "?".into()),
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        r#"You are an expert mortgage assistant inside the Orion loan
origination platform, answering on the Pricing screen for one loan.

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}
{terms_block}

PRICING OPTIONS BUILT ({n}):
{options_block}

YOUR ROLE:
- Compare the options on the table: monthly payment vs. upfront points
  cost, and note the selected option when relevant
- Explain trade-offs plainly (paying points lowers the payment; lender
  credits raise it but cut cash to close)
- Answer questions about how P&I and points math work
- Be concise"#,
        borrower_name = borrower_name,
        terms_block = terms_block,
        n = options.len(),
        options_block = options_block,
    )
}
