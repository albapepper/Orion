use crate::loans::Asset;

pub fn build_context(borrower_name: &str, assets: &[Asset]) -> String {
    let total: f64 = assets.iter().map(|a| a.balance).sum();

    let lines: Vec<String> = if assets.is_empty() {
        vec!["  (no assets entered yet)".to_string()]
    } else {
        assets
            .iter()
            .map(|a| {
                let gift = if a.account_type == "gift" {
                    match (&a.gift_donor, &a.gift_relationship) {
                        (Some(d), _) => format!(", gift from {d} — needs gift letter"),
                        _ => ", gift: donor NOT RECORDED — needs gift letter".to_string(),
                    }
                } else {
                    String::new()
                };
                format!(
                    "  - {} at {}: ${:.0}{gift}{}",
                    a.account_type,
                    a.institution.as_deref().unwrap_or("unknown institution"),
                    a.balance,
                    a.notes
                        .as_deref()
                        .map(|n| format!(" ({n})"))
                        .unwrap_or_default(),
                )
            })
            .collect()
    };

    format!(
        r#"You are an expert mortgage assistant helping a loan officer
on the Assets screen.

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}

ASSETS (newest balances):
{lines}

  Total assets: ${total:.0}
  Note: roughly 2 months of PITI reserves are expected on most programs.

YOUR ROLE:
- Answer questions about reserves, sourcing, and seasoning
- Flag: large recent deposits (sourcing), unexplained gift funds, gift
  funds without a donor recorded, total assets short of closing costs
- Be concise — the user is in the middle of processing a loan

Do not discuss other screens unless directly related to these assets."#,
        lines = lines.join("\n"),
    )
}
