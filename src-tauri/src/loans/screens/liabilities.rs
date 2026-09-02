use crate::loans::Liability;

pub fn build_context(
    borrower_name: &str,
    liabilities: &[Liability],
    total_monthly_income: f64,
) -> String {
    let active: Vec<&Liability> = liabilities.iter().filter(|l| !l.payoff_at_close).collect();
    let total_payments: f64 = active.iter().map(|l| l.monthly_payment.unwrap_or(0.0)).sum();
    let total_balances: f64 = liabilities.iter().map(|l| l.unpaid_balance.unwrap_or(0.0)).sum();

    let dti = if total_monthly_income > 0.0 {
        format!("{:.1}%", total_payments / total_monthly_income * 100.0)
    } else {
        "income not entered yet".to_string()
    };

    let lines: Vec<String> = if liabilities.is_empty() {
        vec!["  (no liabilities entered yet)".to_string()]
    } else {
        liabilities
            .iter()
            .map(|l| {
                let payoff = if l.payoff_at_close {
                    " — will pay off at close (excluded from DTI)"
                } else {
                    ""
                };
                format!(
                    "  - {} at {}: ${:.0}/mo, balance ${:.0}{payoff}{}",
                    l.liability_type,
                    l.creditor_name.as_deref().unwrap_or("unknown creditor"),
                    l.monthly_payment.unwrap_or(0.0),
                    l.unpaid_balance.unwrap_or(0.0),
                    l.notes
                        .as_deref()
                        .map(|n| format!(" ({n})"))
                        .unwrap_or_default(),
                )
            })
            .collect()
    };

    format!(
        r#"You are an expert mortgage assistant helping a loan officer
on the Liabilities screen.

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}
  Total monthly income (all sources): ${total_monthly_income:.0}

LIABILITIES:
{lines}

  Total monthly debt payments: ${total_payments:.0}
  Total unpaid balances: ${total_balances:.0}
  Estimated debt-to-income (excl. housing payment): {dti}

YOUR ROLE:
- Answer DTI questions: what counts, what can be excluded, payoffs
- Flag: high revolving balances, debts likely to be paid off at close,
  anything that looks like it needs a paydown before approval
- The housing payment (PITI) is NOT in this DTI — add it from the
  terms screen when the officer asks about total DTI
- Be concise — the user is in the middle of processing a loan"#,
        lines = lines.join("\n"),
    )
}
