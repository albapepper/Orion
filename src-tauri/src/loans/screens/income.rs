use crate::loans::LoanIncome;

pub fn build_context(
    borrower_name: &str,
    income_records: &[LoanIncome],
    loan_type: &str,
) -> String {
    let total_monthly: f64 = income_records.iter().map(|i| i.monthly_amount).sum();

    let income_detail: String = if income_records.is_empty() {
        "  (no income records entered yet)".to_string()
    } else {
        income_records
            .iter()
            .map(|i| {
                // 2-yr average matters for variable income (bonus,
                // overtime, commission); AUS expects it there.
                let two_year = match i.prior_year_monthly {
                    Some(prior) => format!(
                        ", 2-yr avg: ${:.0}/mo",
                        (i.monthly_amount + prior) / 2.0
                    ),
                    None if matches!(
                        i.income_type.as_str(),
                        "bonus" | "overtime" | "commission"
                    ) =>
                    {
                        ", 2-yr avg: MISSING — needed for variable income".to_string()
                    }
                    None => String::new(),
                };
                format!(
                    "  - {} ({}): ${:.0}/month{two_year}, employer: {}{}",
                    i.income_type,
                    if i.years_on_job.unwrap_or(0.0) >= 2.0 {
                        "2yr history ✓"
                    } else {
                        "< 2yr — verify"
                    },
                    i.monthly_amount,
                    i.employer_name.as_deref().unwrap_or("not listed"),
                    if i.verified { ", verified" } else { "" },
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        r#"You are an expert mortgage assistant helping a loan officer
on the Income Analysis screen.

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}
  Loan type: {loan_type}

INCOME BREAKDOWN:
{income_detail}

  Total qualifying income: ${total_monthly:.0}/month

YOUR ROLE:
- Answer questions about this borrower's income qualification
- Flag potential issues (employment gaps, income type eligibility,
  missing prior-year figures for variable income)
- Be concise — the user is in the middle of processing a loan

Do not discuss other screens, other borrowers, or general mortgage
topics unless directly related to this income analysis."#
    )
}
