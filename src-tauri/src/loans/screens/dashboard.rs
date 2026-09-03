use crate::loans::LoanSummary;
use chrono::{DateTime, Utc};

fn idle_days(summary: &LoanSummary, now: DateTime<Utc>) -> i64 {
    (now - summary.updated_at).num_days().max(0)
}

/// The dashboard has no loan of its own — the context is the whole
/// pipeline. Built from LoanSummary rows only, so it stays cheap no
/// matter how many loans the shop has. The AI reads the same facts the
/// dashboard shows: stage counts, flags, staleness, ownership.
pub fn build_context(loans: &[LoanSummary], now: DateTime<Utc>) -> String {
    let mut counts: Vec<(String, usize)> = Vec::new();
    for loan in loans {
        match counts.iter_mut().find(|(s, _)| *s == loan.status) {
            Some((_, n)) => *n += 1,
            None => counts.push((loan.status.clone(), 1)),
        }
    }
    let count_lines: String = if counts.is_empty() {
        "  (pipeline is empty)".to_string()
    } else {
        counts
            .iter()
            .map(|(status, n)| format!("  - {status}: {n}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let flagged: Vec<&LoanSummary> =
        loans.iter().filter(|l| l.flagged_reviews > 0).collect();
    let flagged_lines: String = if flagged.is_empty() {
        "  (none)".to_string()
    } else {
        flagged
            .iter()
            .map(|l| {
                format!(
                    "  - {} ({}): {} flagged review(s) by underwriting",
                    l.loan_number,
                    l.borrower_name.as_deref().unwrap_or("no borrower"),
                    l.flagged_reviews,
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let stale: Vec<(&LoanSummary, i64)> = loans
        .iter()
        .map(|l| (l, idle_days(l, now)))
        .filter(|(_, days)| *days >= 5)
        .collect();
    let stale_lines: String = if stale.is_empty() {
        "  (none — every active file touched in the last 5 days)".to_string()
    } else {
        stale
            .iter()
            .map(|(l, days)| {
                format!(
                    "  - {} ({}): idle {} days, status {}",
                    l.loan_number,
                    l.borrower_name.as_deref().unwrap_or("no borrower"),
                    days,
                    l.status,
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let unassigned: Vec<&LoanSummary> =
        loans.iter().filter(|l| l.assigned_to_name.is_none()).collect();
    let unassigned_lines: String = if unassigned.is_empty() {
        "  (none)".to_string()
    } else {
        unassigned
            .iter()
            .map(|l| {
                format!(
                    "  - {} ({}): status {}",
                    l.loan_number,
                    l.borrower_name.as_deref().unwrap_or("no borrower"),
                    l.status,
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        r#"You are an expert mortgage assistant inside the Orion loan
origination platform, answering on the Dashboard. No single loan is
open — the user asked what to work on next across their pipeline.

PIPELINE COUNTS BY STAGE:
{count_lines}

UNDERWRITING FLAGS (blocked items needing banker action):
{flagged_lines}

STALE FILES (untouched 5+ days):
{stale_lines}

UNASSIGNED LOANS:
{unassigned_lines}

YOUR ROLE:
- Recommend what to work on next as a short, prioritized list — most
  urgent first, one line per loan, always naming the loan number
- Priority order: flagged reviews first (they block underwriting),
  then stale files, then unassigned loans, then files furthest along
- For each recommendation say in a few words WHAT to do (e.g. clear
  the flagged assets review, follow up on the stale file)
- If the pipeline is clear, say so plainly — don't invent work
- Be concise — this is a morning triage, not a report"#,
        count_lines = count_lines,
        flagged_lines = flagged_lines,
        stale_lines = stale_lines,
        unassigned_lines = unassigned_lines,
    )
}
