use crate::tasks::Task;
use chrono::{DateTime, Utc};

fn due_label(t: &Task, now: DateTime<Utc>) -> String {
    match t.due_date {
        Some(due) => {
            let days = (due - now).num_days();
            if days < 0 {
                format!("OVERDUE by {} day(s)", -days)
            } else if days == 0 {
                "due today".to_string()
            } else {
                format!("due in {days} day(s)")
            }
        }
        None => "no due date".to_string(),
    }
}

fn task_line(t: &Task, now: DateTime<Utc>) -> String {
    format!(
        "  - [{}] {} — loan {}, screen {}, priority {}, {}, owner {}",
        t.status,
        t.title,
        t.loan_number,
        t.screen.as_deref().unwrap_or("general"),
        t.priority,
        due_label(t, now),
        t.assigned_to_name.as_deref().unwrap_or("UNASSIGNED"),
    )
}

/// The Tasks screen has no single loan either — the context is the
/// company's whole task list, the same facts the unified view shows.
pub fn build_context(tasks: &[Task], now: DateTime<Utc>) -> String {
    let open: Vec<&Task> = tasks.iter().filter(|t| t.status != "done").collect();
    let overdue: Vec<&Task> = open
        .iter()
        .filter(|t| t.due_date.is_some_and(|d| d < now))
        .copied()
        .collect();
    let unassigned: Vec<&Task> =
        open.iter().filter(|t| t.assigned_to.is_none()).copied().collect();
    let high: Vec<&Task> = open.iter().filter(|t| t.priority == "high").copied().collect();

    if open.is_empty() {
        return r#"You are an expert mortgage assistant inside the Orion loan
origination platform, answering on the Tasks screen.

CURRENT TASK LIST: all clear — every task is done.

YOUR ROLE:
- Say so plainly and encourage the user; don't invent work
- Be concise"#
            .to_string();
    }

    let lines = |title: &str, rows: &[&Task], now: DateTime<Utc>| -> String {
        if rows.is_empty() {
            format!("{title}: (none)\n")
        } else {
            let items: Vec<String> = rows.iter().map(|t| task_line(t, now)).collect();
            format!("{title}:\n{}", items.join("\n"))
        }
    };

    format!(
        r#"You are an expert mortgage assistant inside the Orion loan
origination platform, answering on the Tasks screen. The user sees
their company's task list and asks what to do about it.

{n_open} open task(s).

{all}
{overdue}
{high}
{unassigned}

YOUR ROLE:
- Recommend what to tackle first: overdue items, then high priority,
  then anything unassigned that blocks a loan's underwriting
- Always name the task, its loan number, and who it belongs to
- Suggest concrete next actions (assign it, chase the borrower, clear
  the condition that spawned it)
- Be concise — this is task triage, not a report"#,
        n_open = open.len(),
        all = lines("OPEN", &open, now),
        overdue = lines("OVERDUE", &overdue, now),
        high = lines("HIGH PRIORITY", &high, now),
        unassigned = lines("UNASSIGNED", &unassigned, now),
    )
}
