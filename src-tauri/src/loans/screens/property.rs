use crate::loans::SubjectProperty;

pub fn build_context(borrower_name: &str, property: &SubjectProperty) -> String {
    let address = match (
        property.address_line.as_deref(),
        property.city.as_deref(),
        property.state.as_deref(),
) {
        (Some(line), Some(city), Some(state)) => format!("{line}, {city}, {state}"),
        (Some(line), _, _) => line.to_string(),
        _ => "(address not entered yet)".to_string(),
    };

    let flags: Vec<String> = Vec::new();
    let flags_text = if flags.is_empty() {
        String::new()
    } else {
        format!("\n{}\n", flags.join("\n"))
    };

    format!(
        r#"You are an expert mortgage assistant helping a loan officer
on the Property screen.

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}
  Subject property: {address}
  Type: {} ({} unit{})
  Occupancy: {}
  Year built: {}
  Estimated value: {}

{flags_text}
YOUR ROLE:
- Answer questions about property eligibility and occupancy
- Flag: investment/second-home occupancy (affects LTV limits and
  pricing), condo review requirements, manufactured-home conditions,
  multi-unit rental income implications
- Be concise — the user is in the middle of processing a loan"#,
        property.property_type,
        property.units,
        if property.units == 1 { "" } else { "s" },
        property.occupancy,
        property
            .year_built
            .map(|y| y.to_string())
            .unwrap_or_else(|| "unknown".into()),
        property
            .estimated_value
            .map(|v| format!("${v:.0}"))
            .unwrap_or_else(|| "not entered".into()),
    )
}
