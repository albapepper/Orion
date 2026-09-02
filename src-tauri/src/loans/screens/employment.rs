use crate::loans::Employment;
use chrono::{Datelike, NaiveDate};

fn today() -> NaiveDate {
    chrono::Local::now().date_naive()
}

/// Human tenure between two dates, e.g. "3yr 2mo".
fn tenure(from: NaiveDate, to: NaiveDate) -> String {
    let mut months = (to.year() - from.year()) * 12 + (to.month() as i32 - from.month() as i32);
    if to.day() < from.day() {
        months -= 1;
    }
    if months < 0 {
        return "started".into();
    }
    format!("{}yr {}mo", months / 12, months % 12)
}

/// Employment history context. Flags gaps between jobs so the officer
/// can explain them before an underwriter asks.
pub fn build_context(borrower_name: &str, employments: &[Employment]) -> String {
    if employments.is_empty() {
        return format!(
            r#"You are an expert mortgage assistant helping a loan officer
on the Employment screen.

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}
  Employment history: (none entered yet)

YOUR ROLE:
- Note that employment history is empty; 2-year history is required for
  most programs
- Be concise — the user is in the middle of processing a loan"#
        );
    }

    // Newest first (is_current first, then by start date desc) — same
    // order as the screen displays.
    let mut sorted: Vec<&Employment> = employments.iter().collect();
    sorted.sort_by(|a, b| {
        b.is_current
            .cmp(&a.is_current)
            .then(b.start_date.cmp(&a.start_date))
    });

    let lines: Vec<String> = sorted
        .iter()
        .map(|e| {
            let span = match (e.start_date, e.end_date) {
                (Some(s), Some(end)) => format!("{} → {} ({})", s, end, tenure(s, end)),
                (Some(s), None) => format!("{} → present ({})", s, tenure(s, today())),
                _ => "dates missing".to_string(),
            };
            format!(
                "  - {}{} ({}), monthly income: {}",
                e.employer_name.as_deref().unwrap_or("(unnamed employer)"),
                if e.self_employed { " — self-employed" } else { "" },
                span,
                e.monthly_income
                    .map(|m| format!("${m:.0}"))
                    .unwrap_or_else(|| "not entered".into()),
            )
        })
        .collect();

    // Gap check across the last two years of history.
    let mut gap_flags: Vec<String> = Vec::new();
    for pair in sorted.windows(2) {
        let (older, newer) = (pair[1], pair[0]);
        if let (Some(end), Some(next_start)) = (older.end_date, newer.start_date) {
            let gap_days = (next_start - end).num_days();
            if gap_days > 30 {
                gap_flags.push(format!(
                    "  - {} to {} ({} days) between {} and {}",
                    end,
                    next_start,
                    gap_days,
                    older.employer_name.as_deref().unwrap_or("previous job"),
                    newer.employer_name.as_deref().unwrap_or("next job"),
                ));
            }
        }
    }
    let missing_dates: Vec<String> = sorted
        .iter()
        .filter(|e| e.start_date.is_none())
        .map(|e| {
            format!(
                "  - {} has no start date",
                e.employer_name.as_deref().unwrap_or("an entry")
            )
        })
        .collect();

    let warnings = if gap_flags.is_empty() && missing_dates.is_empty() {
        "  (none — history looks continuous)".to_string()
    } else {
        [gap_flags, missing_dates].concat().join("\n")
    };

    let current = sorted[0];
    let line_of_work = current
        .months_in_line_of_work
        .map(|m| format!("  Time in line of work: {}yr {}mo", m / 12, m % 12))
        .unwrap_or_default();

    format!(
        r#"You are an expert mortgage assistant helping a loan officer
on the Employment screen.

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}

EMPLOYMENT HISTORY (newest first):
{history}

GAPS / FLAGS:
{warnings}
{line_of_work}

YOUR ROLE:
- Answer questions about this borrower's employment stability
- Help draft short explanations for any gaps (standard underwriting asks)
- Flag: short tenure at current job, missing dates, self-employment
  needing extra documentation
- Be concise — the user is in the middle of processing a loan"#,
        history = lines.join("\n"),
    )
}
