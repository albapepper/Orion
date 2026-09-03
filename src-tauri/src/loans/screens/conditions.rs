use crate::conditions::Condition;
use chrono::{DateTime, Utc};

/// The Conditions screen context: what underwriting still needs before
/// this loan can clear — the blockers and their deadlines.
pub fn build_context(conditions: &[Condition], now: DateTime<Utc>) -> String {
    if conditions.is_empty() {
        return r#"You are an expert mortgage assistant inside the Orion loan
origination platform, answering on the Conditions screen.

CONDITIONS: none on file.

YOUR ROLE:
- Note that underwriting has listed no conditions yet
- If the user asks, briefly explain that conditions are the checklist
  underwriting requires before clearing the loan
- Be concise"#
            .to_string();
    }

    let line = |c: &Condition| {
        let due = match c.clear_by {
            Some(d) => {
                let days = (d - now).num_days();
                if days < 0 && c.status != "cleared" && c.status != "waived" {
                    format!("OVERDUE by {} day(s)", -days)
                } else if days < 0 {
                    format!("was due {} day(s) ago", -days)
                } else if days == 0 {
                    "due today".to_string()
                } else {
                    format!("due in {days} day(s)")
                }
            }
            None => "no deadline".to_string(),
        };
        format!(
            "  - [{}] {} — priority {}, from {}, {}, cleared by {}",
            c.status,
            c.title,
            c.priority,
            c.source,
            due,
            c.cleared_by_name.as_deref().unwrap_or("—"),
        )
    };

    let outstanding: Vec<&Condition> = conditions
        .iter()
        .filter(|c| c.status == "open" || c.status == "submitted")
        .collect();
    let done: Vec<&Condition> = conditions
        .iter()
        .filter(|c| c.status == "cleared" || c.status == "waived")
        .collect();

    let mut sections = String::new();
    if outstanding.is_empty() {
        sections.push_str("OUTSTANDING CONDITIONS: none — everything is cleared or waived.\n");
    } else {
        sections.push_str(&format!(
            "OUTSTANDING CONDITIONS ({}):\n{}\n",
            outstanding.len(),
            outstanding
                .into_iter()
                .map(line)
                .collect::<Vec<_>>()
                .join("\n"),
        ));
    }
    if !done.is_empty() {
        sections.push_str(&format!(
            "\nCLEARED / WAIVED ({}):\n{}\n",
            done.len(),
            done.into_iter().map(line).collect::<Vec<_>>().join("\n"),
        ));
    }

    format!(
        r#"You are an expert mortgage assistant inside the Orion loan
origination platform, answering on the Conditions screen. Conditions
are underwriting's checklist — outstanding ones block approval.

{sections}
YOUR ROLE:
- Point at what blocks approval first: overdue items, then high
  priority, then submitted-but-not-yet-reviewed items
- For each blocker suggest the concrete next step (chase the borrower,
  order the appraisal, confirm the payoff statement arrived)
- If nothing is outstanding, say the file is clear
- Be concise"#
    )
}
