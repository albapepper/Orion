//! P4-05: Automated underwriting (AUS) — submit the serialized loan to
//! DU, store and surface findings. P4-06's LPA rides the same table.
//!
//! Two modes, decided by config: a configured `du_endpoint` means real
//! sandbox submission (the MISMO document goes out, whatever comes back
//! is parsed best-effort and stored raw); an empty endpoint means mock
//! mode — findings are derived deterministically from the file's own
//! numbers with DU-flavored rules, so the workflow is demoable before
//! GSE sandbox credentials land. Mock mode is honest about being mock:
//! the submission row carries `system = "du-mock"`.

use crate::loans::{Borrower, Liability, LoanIncome, LoanTerms};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// One findings message from the engine. DU organizes output as
/// recommendation + a list of findings; severity is Orion's rough cut
/// (error = blocked evaluation, warning = needs attention, info = note).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AusMessage {
    pub severity: String, // "error" | "warning" | "info"
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct AusSubmission {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    /// "du" (live) or "du-mock" / "lpa-mock" (simulated).
    pub system: String,
    pub case_id: Option<String>,
    pub recommendation: Option<String>,
    pub messages: serde_json::Value,
    pub raw_response: Option<String>,
    pub submitted_by: Option<Uuid>,
    pub submitted_at: chrono::DateTime<Utc>,
}

pub async fn list_for_loan(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<AusSubmission>> {
    let rows = sqlx::query_as::<_, AusSubmission>(
        r#"
        SELECT id, loan_id, company_id, system, case_id, recommendation,
               messages, raw_response, submitted_by, submitted_at
        FROM aus_submissions
        WHERE loan_id = $1
        ORDER BY submitted_at DESC
        "#,
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
pub async fn insert(
    pool: &PgPool,
    loan_id: Uuid,
    company_id: Uuid,
    system: &str,
    case_id: Option<&str>,
    recommendation: Option<&str>,
    messages: serde_json::Value,
    raw_response: Option<&str>,
    submitted_by: Uuid,
) -> anyhow::Result<AusSubmission> {
    let row = sqlx::query_as::<_, AusSubmission>(
        r#"
        INSERT INTO aus_submissions
            (loan_id, company_id, system, case_id, recommendation, messages,
             raw_response, submitted_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, loan_id, company_id, system, case_id, recommendation,
                  messages, raw_response, submitted_by, submitted_at
        "#,
    )
    .bind(loan_id)
    .bind(company_id)
    .bind(system)
    .bind(case_id)
    .bind(recommendation)
    .bind(messages)
    .bind(raw_response)
    .bind(submitted_by)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

// ── Mock engine ──────────────────────────────────────────────────────

/// What the mock engine reads: the file's own numbers, nothing else.
/// Kept to the fields the DU-ish rules actually use.
pub struct FileFacts<'a> {
    pub loan_number: &'a str,
    pub terms: Option<&'a LoanTerms>,
    pub borrowers: &'a [Borrower],
    pub income: &'a [LoanIncome],
    pub liabilities: &'a [Liability],
    pub declarations: &'a [crate::loans::Declaration],
}

/// Which engine a submission targets. DU and LPA share one table and
/// one mock engine, but their vocabularies differ: DU speaks of debt
/// ratios and Refer; LPA (P4-06) speaks of obligation ratios and
/// issues Accept or Caution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AusEngine {
    Du,
    Lpa,
}

impl AusEngine {
    /// `system` value for live submissions.
    pub fn label(self) -> &'static str {
        match self {
            AusEngine::Du => "du",
            AusEngine::Lpa => "lpa",
        }
    }

    /// `system` value when the built-in mock scored the file.
    pub fn mock_label(self) -> &'static str {
        match self {
            AusEngine::Du => "du-mock",
            AusEngine::Lpa => "lpa-mock",
        }
    }

    fn case_prefix(self) -> &'static str {
        match self {
            AusEngine::Du => "DU",
            AusEngine::Lpa => "LPA",
        }
    }
}

/// What the mock engine concluded.
pub struct MockOutcome {
    pub system: &'static str,
    pub recommendation: String,
    pub case_id: Option<String>,
    pub messages: Vec<AusMessage>,
}

/// Deterministic engine-flavored findings from the file. The same file
/// always produces the same answer — no randomness, so an LO can't
/// resubmit their way to a better outcome in demo mode. The math is
/// shared; only the vocabulary differs between DU and LPA.
pub fn mock_findings(engine: AusEngine, facts: &FileFacts) -> MockOutcome {
    let mut messages: Vec<AusMessage> = Vec::new();

    // Case id: derived from the loan number + today's date so it looks
    // and behaves like a real case file reference.
    let case_id = format!(
        "{}-{}-{}",
        engine.case_prefix(),
        Utc::now().format("%y%m%d"),
        hash_short(facts.loan_number)
    );

    let Some(t) = facts.terms else {
        messages.push(AusMessage {
            severity: "error".into(),
            text: "Loan terms not entered — the engine cannot evaluate a file with no deal shape."
                .into(),
        });
        return outcome(engine, "Unable to Determine", case_id, messages);
    };

    let Some(amount) = t.loan_amount else {
        messages.push(AusMessage {
            severity: "error".into(),
            text: "Loan amount missing — Unable to Determine.".into(),
        });
        return outcome(engine, "Unable to Determine", case_id, messages);
    };

    // Identity: a usable borrower is table stakes for either engine.
    let primary = facts
        .borrowers
        .iter()
        .find(|b| b.is_primary)
        .or(facts.borrowers.first());
    let mut hard_error = false;
    match primary {
        None => {
            messages.push(AusMessage {
                severity: "error".into(),
                text: "No borrower on the loan — Unable to Determine.".into(),
            });
            hard_error = true;
        }
        Some(b) => {
            if b.ssn.is_none() {
                messages.push(AusMessage {
                    severity: "error".into(),
                    text: "Borrower SSN missing — credit could not be evaluated.".into(),
                });
                hard_error = true;
            }
        }
    }

    // Disclosures both engines treat as refer-level risk.
    let risky_keys = [
        "bankruptcy_7yr",
        "foreclosure_7yr",
        "obligated_foreclosure_7yr",
        "delinquent_federal",
    ];
    let disclosed_risk = facts
        .declarations
        .iter()
        .any(|d| risky_keys.contains(&d.question_key.as_str()) && d.answer == Some(true));
    if disclosed_risk {
        messages.push(AusMessage {
            severity: "warning".into(),
            text: match engine {
                AusEngine::Du => {
                    "Bankruptcy, foreclosure, or federal debt disclosed — refer for review."
                }
                AusEngine::Lpa => {
                    "Bankruptcy, foreclosure, or federal debt disclosed — caution."
                }
            }
            .into(),
        });
    }

    // Qualifying ratios from the file's own math. DU calls the number
    // a debt ratio; Freddie's feedback certificate says obligation
    // ratio — same division, different dialect.
    let total_income: f64 = facts.income.iter().map(|i| i.monthly_amount).sum();
    let total_debt: f64 = facts
        .liabilities
        .iter()
        .filter(|l| !l.payoff_at_close)
        .filter_map(|l| l.monthly_payment)
        .sum();
    let mut refer = disclosed_risk;
    let mut caution = false;
    let ratio_name = match engine {
        AusEngine::Du => "Total debt ratio",
        AusEngine::Lpa => "Total obligation ratio",
    };
    if total_income > 0.0 {
        let dti = (total_debt / total_income) * 100.0;
        if dti > 50.0 {
            messages.push(AusMessage {
                severity: "warning".into(),
                text: match engine {
                    AusEngine::Du => {
                        format!("{ratio_name} {dti:.1}% exceeds the 50.0% limit — Refer with Caution.")
                    }
                    AusEngine::Lpa => {
                        format!("{ratio_name} {dti:.1}% exceeds the 50.0% limit — Caution.")
                    }
                },
            });
            refer = true;
            caution = true;
        } else if dti > 43.0 {
            messages.push(AusMessage {
                severity: "warning".into(),
                text: format!(
                    "{ratio_name} {dti:.1}% exceeds 43.0% — manual underwriting review advised."
                ),
            });
            refer = true;
        } else {
            messages.push(AusMessage {
                severity: "info".into(),
                text: format!("{ratio_name} {dti:.1}% — within tolerance."),
            });
        }
    } else {
        messages.push(AusMessage {
            severity: "warning".into(),
            text: "No qualifying income on file — ratios could not be computed.".into(),
        });
        refer = true;
    }

    // Collateral: LTV from the entered values.
    if let Some(value) = t.property_value {
        if value > 0.0 {
            let ltv = amount_pct(amount, value);
            if ltv > 97.0 {
                messages.push(AusMessage {
                    severity: "warning".into(),
                    text: match engine {
                        AusEngine::Du => {
                            format!("LTV {ltv:.1}% exceeds 97.0% — not eligible for approval.")
                        }
                        AusEngine::Lpa => {
                            format!("LTV {ltv:.1}% exceeds 97.0% — not eligible for acceptance.")
                        }
                    },
                });
                refer = true;
                caution = true;
            } else if ltv > 80.0 {
                messages.push(AusMessage {
                    severity: "info".into(),
                    text: format!("LTV {ltv:.1}% — mortgage insurance applies above 80%."),
                });
            }
        }
    }

    // DU has a four-way recommendation ladder; LPA compresses the
    // refer tiers into a single Caution.
    let recommendation: String = match engine {
        AusEngine::Du => {
            if hard_error {
                "Unable to Determine".into()
            } else if caution {
                "Refer with Caution".into()
            } else if refer {
                "Refer".into()
            } else {
                messages.push(AusMessage {
                    severity: "info".into(),
                    text: "Approve/Eligible — the file meets automated underwriting criteria."
                        .into(),
                });
                "Approve/Eligible".into()
            }
        }
        AusEngine::Lpa => {
            if hard_error {
                "Unable to Determine".into()
            } else if caution || refer {
                "Caution".into()
            } else {
                messages.push(AusMessage {
                    severity: "info".into(),
                    text: "Accept — the file meets LPA criteria for acceptance.".into(),
                });
                "Accept".into()
            }
        }
    };
    outcome(engine, &recommendation, case_id, messages)
}

fn outcome(
    engine: AusEngine,
    recommendation: &str,
    case_id: String,
    messages: Vec<AusMessage>,
) -> MockOutcome {
    MockOutcome {
        system: engine.mock_label(),
        recommendation: recommendation.to_string(),
        case_id: Some(case_id),
        messages,
    }
}

fn amount_pct(part: f64, whole: f64) -> f64 {
    if whole <= 0.0 {
        0.0
    } else {
        part / whole * 100.0
    }
}

/// Stable short hash (FNV-1a) for mock case ids — deterministic, not
/// cryptographic; this is demo scaffolding, not a real case file.
fn hash_short(s: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{:08X}", h & 0xFFFFFFFF)
}

// ── Live response parsing ────────────────────────────────────────────

/// Best-effort parse of a live engine response. GSE sandbox response
/// shapes vary by integration channel; we extract what we can
/// (recommendation, findings messages) and always keep the raw text.
/// Anything unparseable still gets stored — findings display marks it
/// for manual review rather than guessing.
pub fn parse_response(system: &str, body: &str) -> (Option<String>, Vec<AusMessage>) {
    let mut messages = Vec::new();

    // Recommendation: first <Recommendation>...</Recommendation> or a
    // LPA-style <AUSRecommendation> element.
    let recommendation = extract_tag(body, "Recommendation")
        .or_else(|| extract_tag(body, "AUSRecommendation"))
        .map(|s| s.trim().to_string());

    // Findings: any <FindingsMessage>/<MessageText> elements.
    for tag in ["FindingsMessage", "MessageText", "Finding"] {
        for text in extract_tags(body, tag) {
            let t = text.trim().to_string();
            if !t.is_empty() {
                messages.push(AusMessage {
                    severity: severity_of(&t),
                    text: t,
                });
            }
        }
        if !messages.is_empty() {
            break;
        }
    }

    if recommendation.is_none() && messages.is_empty() {
        messages.push(AusMessage {
            severity: "warning".into(),
            text: format!(
                "{system} response could not be parsed — raw response stored for review."
            ),
        });
    }
    (recommendation, messages)
}

fn severity_of(text: &str) -> String {
    let l = text.to_lowercase();
    if l.contains("unable") || l.contains("error") || l.contains("reject") {
        "error".into()
    } else if l.contains("caution") || l.contains("refer") || l.contains("ineligible") {
        "warning".into()
    } else {
        "info".into()
    }
}

fn extract_tag(body: &str, tag: &str) -> Option<String> {
    extract_tags(body, tag).into_iter().next()
}

/// Tag content extractor without an XML parser: matches `<Tag ...>...
/// </Tag>` blocks, tolerating attributes and namespaces.
fn extract_tags(body: &str, tag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(start) = find_tag(rest, tag) {
        let after_open = &rest[start.1..];
        let Some(close_rel) = after_open.find(&format!("</{tag}")) else {
            break;
        };
        out.push(after_open[..close_rel].to_string());
        rest = &after_open[close_rel..];
    }
    out
}

/// Finds `<Tag` or `<ns:Tag`, returns (end of tag name scan, offset of `>`+1).
fn find_tag(body: &str, tag: &str) -> Option<(usize, usize)> {
    let mut search = 0;
    while let Some(pos) = body[search..].find('<') {
        let abs = search + pos;
        let after = &body[abs + 1..];
        let name = after
            .split(['>', ' ', '\t', '\n'])
            .next()
            .unwrap_or("");
        let name = name.rsplit(':').next().unwrap_or(name);
        if name.eq_ignore_ascii_case(tag) {
            let gt = body[abs..].find('>')? + abs + 1;
            return Some((abs, gt));
        }
        search = abs + 1;
    }
    None
}

