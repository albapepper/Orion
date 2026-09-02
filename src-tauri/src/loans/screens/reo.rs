use crate::loans::ReoProperty;

/// Address as one line, or a stand-in when it's not filled in.
fn address_line(r: &ReoProperty) -> String {
    let street = r.address_line.as_deref().unwrap_or("(address not entered)");
    match (r.city.as_deref(), r.state.as_deref()) {
        (Some(c), Some(s)) => format!("{street}, {c}, {s}"),
        (Some(c), None) => format!("{street}, {c}"),
        _ => street.to_string(),
    }
}

pub fn build_context(borrower_name: &str, reo: &[ReoProperty]) -> String {
    let total_value: f64 = reo.iter().map(|r| r.market_value.unwrap_or(0.0)).sum();
    let total_payments: f64 = reo.iter().map(|r| r.mortgage_payment.unwrap_or(0.0)).sum();
    let gross_rent: f64 = reo.iter().map(|r| r.rental_income.unwrap_or(0.0)).sum();
    // Standard underwriting nets 25% vacancy/maintenance off gross rent.
    let net_rent: f64 = gross_rent * 0.75;

    let lines: Vec<String> = if reo.is_empty() {
        vec!["  (no other properties owned)".to_string()]
    } else {
        reo.iter()
            .map(|r| {
                format!(
                    "  - {} ({}): value ${:.0}, payment ${:.0}/mo, gross rent ${:.0}/mo — {}{}",
                    address_line(r),
                    r.property_type,
                    r.market_value.unwrap_or(0.0),
                    r.mortgage_payment.unwrap_or(0.0),
                    r.rental_income.unwrap_or(0.0),
                    r.disposition.replace('_', " "),
                    r.notes
                        .as_deref()
                        .map(|n| format!(" ({n})"))
                        .unwrap_or_default(),
                )
            })
            .collect()
    };

    format!(
        r#"You are an expert mortgage assistant helping a loan officer
on the REO (Real Estate Owned) screen.

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}

OTHER PROPERTIES OWNED:
{lines}

  Total REO value: ${total_value:.0}
  Total REO payments: ${total_payments:.0}/mo
  Gross rent: ${gross_rent:.0}/mo — net (75%): ${net_rent:.0}/mo

YOUR ROLE:
- Answer questions about rental income treatment (75% netting) and
  how pending sales / retained properties affect qualification
- Flag: pending-sale properties (need listing agreements + payoffs),
  negative cash flow, properties with no value or payment entered
- Be concise — the user is in the middle of processing a loan"#,
        lines = lines.join("\n"),
    )
}
