use crate::loans::Declaration;
use std::collections::HashMap;

/// The URLA §5 declaration questions, keyed the same way as
/// `loan_declarations.question_key`. Shared between the UI and the AI
/// context; the question text is the source of truth for wording.
pub const URLA_QUESTIONS: &[(&str, &str)] = &[
    (
        "primary_residence",
        "Is this the borrower's primary residence?",
    ),
    (
        "bankruptcy_7yr",
        "Has the borrower declared bankruptcy in the last 7 years?",
    ),
    (
        "foreclosure_7yr",
        "Has the borrower had property foreclosed upon in the last 7 years?",
    ),
    (
        "lawsuit_7yr",
        "Has the borrower been a party to a lawsuit in the last 7 years?",
    ),
    (
        "obligated_foreclosure_7yr",
        "Has the borrower been obligated on a loan that resulted in foreclosure, short sale, or deed-in-lieu in the last 7 years?",
    ),
    (
        "delinquent_federal",
        "Is the borrower presently delinquent on any federal debt?",
    ),
    (
        "alimony_child_support",
        "Is the borrower obligated to pay alimony, child support, or separate maintenance?",
    ),
    (
        "co_signer",
        "Is the borrower a co-signer or guarantor on any debt not listed?",
    ),
    ("us_citizen", "Is the borrower a U.S. citizen?"),
    (
        "permanent_resident",
        "Is the borrower a permanent resident alien?",
    ),
];

pub fn question_text(key: &str) -> &str {
    URLA_QUESTIONS
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, text)| *text)
        .unwrap_or(key)
}

/// Declarations context: yes-answers need explanations; "no" on the
/// citizenship/residency pair can conflict with the borrower profile.
pub fn build_context(
    borrower_name: &str,
    declarations: &[Declaration],
) -> String {
    let by_key: HashMap<&str, &Declaration> = declarations
        .iter()
        .map(|d| (d.question_key.as_str(), d))
        .collect();

    let answered: Vec<String> = declarations
        .iter()
        .map(|d| {
            let answer = match d.answer {
                Some(true) => "YES",
                Some(false) => "no",
                None => "not answered",
            };
            let expl = d
                .explanation
                .as_deref()
                .map(|e| format!(" — explanation: \"{e}\""))
                .unwrap_or_default();
            format!(
                "  - {}: {}{expl}",
                question_text(&d.question_key),
                answer
            )
        })
        .collect();

    let mut flags: Vec<String> = declarations
        .iter()
        .filter(|d| d.answer == Some(true))
        .map(|d| {
            if d.explanation.as_deref().unwrap_or("").is_empty() {
                format!("  - YES on \"{}\" with no explanation", question_text(&d.question_key))
            } else {
                String::new()
            }
        })
        .filter(|s| !s.is_empty())
        .collect();

    let answered_by = |key: &str| by_key.get(key).and_then(|d| d.answer);
    if answered_by("us_citizen") == Some(false) && answered_by("permanent_resident") == Some(false) {
        flags.push("  - Neither citizen nor permanent resident — residency status unclear".into());
    }

    let flags_text = if flags.is_empty() {
        "  (none)".to_string()
    } else {
        flags.join("\n")
    };

    format!(
        r#"You are an expert mortgage assistant helping a loan officer
on the Declarations screen (URLA section 5).

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}

DECLARATIONS:
{answered}

FLAGS:
{flags_text}

YOUR ROLE:
- Help draft concise underwriter-ready explanations for YES answers
- Flag unanswered questions (all must be answered before submission)
- Never restate sensitive details beyond what is on screen
- Be concise — the user is in the middle of processing a loan"#,
        answered = answered.join("\n"),
    )
}
