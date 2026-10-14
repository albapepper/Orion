//! P4-03 Pricing AI: constrained option suggestions.
//!
//! The loan officer asks in plain language — "give me options at 6.5%
//! with under $3K in points" — and Orion answers with real numbers.
//! The request is parsed into hard constraints (deterministically
//! first, AI as a fallback for unusual phrasing), the active rate
//! sheet is filtered server-side against the loan's own terms, and
//! every suggestion carries the same P&I / points-cost math the
//! manual screen uses. The model never invents rates or prices; it
//! can only narrow the search, so a bad answer degrades to "no caps
//! parsed", never to wrong numbers.

use crate::ai::{Message, Provider};
use crate::loans::LoanTerms;
use crate::pricing::{RateOption, RateSheet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The hard caps parsed out of the request. Anything left as None was
/// unconstrained — the suggestion engine treats None as "no limit".
#[derive(Debug, Default, Serialize, Clone)]
pub struct PricingConstraints {
    pub max_rate: Option<f64>,
    pub max_points: Option<f64>,
    pub max_points_cost: Option<f64>,
    pub product: Option<String>,
}

impl PricingConstraints {
    pub fn is_empty(&self) -> bool {
        self.max_rate.is_none()
            && self.max_points.is_none()
            && self.max_points_cost.is_none()
            && self.product.is_none()
    }
}

/// One suggestion row: a rate-sheet price computed against this loan.
/// Not yet saved anywhere — adding it goes through the normal,
/// audited, lock-checked pricing path.
#[derive(Debug, Serialize, Clone)]
pub struct PricingSuggestion {
    pub sheet_option_id: Uuid,
    pub product: String,
    pub rate: f64,
    pub points: f64,
    pub monthly_pi: Option<f64>,
    pub points_cost: Option<f64>,
    pub comp_plan: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PricingSuggestions {
    pub request: String,
    pub max_rate: Option<f64>,
    pub max_points: Option<f64>,
    pub max_points_cost: Option<f64>,
    pub product: Option<String>,
    /// What Orion understood, plus any caveats (no sheet, no loan
    /// amount, nothing matched). Shown above the results.
    pub notes: Vec<String>,
    pub matches: Vec<PricingSuggestion>,
}

// ── Constraint parsing ───────────────────────────────────────────────

/// Numeric tokens in a string: (start, end, value). Understands
/// thousands commas ("3,000") and decimals; trailing punctuation is
/// not part of the number.
fn number_spans(s: &str) -> Vec<(usize, usize, f64)> {
    let bytes = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            let mut end = i;
            let mut seen_dot = false;
            while end < bytes.len()
                && (bytes[end].is_ascii_digit()
                    || (bytes[end] == b'.' && !seen_dot)
                    || (bytes[end] == b',' && end + 1 < bytes.len() && bytes[end + 1].is_ascii_digit()))
            {
                if bytes[end] == b'.' {
                    seen_dot = true;
                }
                end += 1;
            }
            let mut e = end;
            while e > start && (bytes[e - 1] == b',' || bytes[e - 1] == b'.') {
                e -= 1;
            }
            let cleaned: String = s[start..e].chars().filter(|c| *c != ',').collect();
            if let Ok(v) = cleaned.parse::<f64>() {
                out.push((start, e, v));
            }
            i = e;
        } else {
            i += 1;
        }
    }
    out
}

/// Deterministic parse of the request — covers the phrasings LOs
/// actually type: "at 6.5%", "under 6.5%", "$3K in points",
/// "$3,000 in points", "under 1 point", "15 year", "ARM". A rate
/// mention reads as a cap (at-or-under); the first one wins.
pub fn parse_constraints(request: &str) -> PricingConstraints {
    let s = request.to_lowercase();
    let spans = number_spans(&s);
    let mut c = PricingConstraints::default();

    // Rate: a number followed by %.
    for (_, end, v) in &spans {
        if *v <= 0.0 || *v >= 20.0 {
            continue;
        }
        if s[*end..].trim_start().starts_with('%') {
            c.max_rate = Some(*v);
            break;
        }
    }

    // Money or bare k-quantities tied to points, and points percents.
    for (start, end, v) in &spans {
        let rest = &s[*end..];
        let before_is_dollar = s[..*start].trim_end().ends_with('$');
        let (mult, after_mult) = if let Some(r) = rest.strip_prefix('k') {
            (1_000.0, r)
        } else if let Some(r) = rest.strip_prefix('m') {
            (1_000_000.0, r)
        } else {
            (1.0, rest)
        };
        let mentions_points = after_mult.chars().take(30).collect::<String>().contains("point");
        if (before_is_dollar || mult > 1.0) && *v > 0.0 && mentions_points {
            // "$3K in points", "$3,000 points", "3k in points".
            let amount = v * mult;
            c.max_points_cost = Some(c.max_points_cost.map_or(amount, |m| m.max(amount)));
        } else if *v < 10.0 && c.max_points.is_none() {
            let after = after_mult.trim_start();
            if after.starts_with("point") || after.starts_with("pts") {
                // "under 1 point", "2 pts" — a percent, not dollars.
                c.max_points = Some(*v);
            }
        }
    }

    // Product keywords, in sheet-product vocabulary.
    if s.contains("15yr") || s.contains("15 year") || s.contains("15-year") {
        c.product = Some("15yr_fixed".into());
    } else if s.contains("arm") || s.contains("7/1") {
        c.product = Some("7_1_arm".into());
    } else if s.contains("30yr") || s.contains("30 year") || s.contains("30-year") {
        c.product = Some("30yr_fixed".into());
    }
    c
}

#[derive(Deserialize)]
struct AiConstraints {
    max_rate: Option<f64>,
    max_points: Option<f64>,
    max_points_cost: Option<f64>,
    product: Option<String>,
}

/// AI parse for phrasing the deterministic pass can't pin down. The
/// model only extracts constraints — it never sees the sheet — and
/// its answer is sanity-checked before use. Gaps the regex did find
/// stay authoritative.
pub async fn parse_constraints_ai(
    provider: &std::sync::Arc<Provider>,
    request: &str,
) -> anyhow::Result<PricingConstraints> {
    let system = r#"You extract mortgage pricing constraints from a loan officer's request.
Reply with ONLY a JSON object, no prose, using these keys (null when the request does not constrain them):
{"max_rate": <highest acceptable interest rate, e.g. 6.5>,
 "max_points": <highest acceptable points as a percent, e.g. 1.0>,
 "max_points_cost": <highest acceptable total dollar cost of points, e.g. 3000>,
 "product": "30yr_fixed" | "15yr_fixed" | "7_1_arm" | null}
Examples:
"give me options at 6.5% with under $3K in points" -> {"max_rate":6.5,"max_points":null,"max_points_cost":3000,"product":null}
"15 year loans under an inch of points" -> {"max_rate":null,"max_points":1.0,"max_points_cost":null,"product":"15yr_fixed"}"#;
    let messages = vec![Message {
        role: "user".into(),
        content: request.to_string(),
    }];
    let resp = provider.complete(system, &messages, false).await?;

    // The model may wrap the JSON in prose or code fences — grab the
    // outermost brace block and parse that.
    let start = resp
        .content
        .find('{')
        .ok_or_else(|| anyhow::anyhow!("no JSON in reply"))?;
    let end = resp
        .content
        .rfind('}')
        .ok_or_else(|| anyhow::anyhow!("unterminated JSON in reply"))?;
    let parsed: AiConstraints = serde_json::from_str(&resp.content[start..=end])?;

    // Fill the gaps the deterministic parse missed; keep its results
    // where they exist (deterministic beats model on conflicts).
    let mut c = parse_constraints(request);
    if c.max_rate.is_none() {
        c.max_rate = parsed.max_rate.filter(|r| *r > 0.0 && *r < 20.0);
    }
    if c.max_points.is_none() {
        c.max_points = parsed.max_points.filter(|p| *p > 0.0 && *p < 10.0);
    }
    if c.max_points_cost.is_none() {
        c.max_points_cost = parsed.max_points_cost.filter(|v| *v > 0.0);
    }
    if c.product.is_none() {
        c.product = parsed.product.filter(|p| {
            matches!(p.as_str(), "30yr_fixed" | "15yr_fixed" | "7_1_arm")
        });
    }
    Ok(c)
}

// ── The suggestion engine ────────────────────────────────────────────

pub fn suggest(
    request: &str,
    terms: Option<&LoanTerms>,
    sheet: Option<&RateSheet>,
    options: &[RateOption],
    constraints: PricingConstraints,
    parse_note: Option<String>,
) -> PricingSuggestions {
    let mut notes: Vec<String> = Vec::new();
    if let Some(n) = parse_note {
        notes.push(n);
    }

    // Empty constraints means nothing usable was parsed — say so and
    // show the sheet rather than pretending the request was understood.
    if constraints.is_empty() {
        notes.push(
            "Couldn't find any caps in that — showing every sheet row that \
             fits the loan. Try e.g. \"options at 6.5% with under $3K in points\"."
                .into(),
        );
    }

    let amount = terms.as_ref().and_then(|t| t.loan_amount);
    let term = terms.as_ref().and_then(|t| t.term_months);

    let mut out = PricingSuggestions {
        request: request.to_string(),
        max_rate: constraints.max_rate,
        max_points: constraints.max_points,
        max_points_cost: constraints.max_points_cost,
        product: constraints.product.clone(),
        notes,
        matches: Vec::new(),
    };

    let Some(sheet) = sheet else {
        out.notes
            .push("No active rate sheet — set one up in Rate sheets (⌃y) first.".into());
        return out;
    };
    out.notes.push(format!(
        "Priced against {}'s active sheet{}.",
        sheet.lender,
        amount
            .map(|a| format!(" — loan amount {}", crate::loans::screens::pricing::money(a)))
            .unwrap_or_default(),
    ));
    if options.is_empty() {
        out.notes
            .push("The active sheet has no rate rows to price against.".into());
        return out;
    }

    // A sheet row can be amount-gated (min/max loan size); respect
    // that against the loan's own amount before applying the caps.
    let fits_amount = |o: &RateOption| {
        o.min_loan_amount.is_none_or(|min| amount.is_none_or(|a| a >= min))
            && o.max_loan_amount.is_none_or(|max| amount.is_none_or(|a| a <= max))
    };

    for o in options {
        if let Some(p) = &constraints.product {
            if &o.product != p {
                continue;
            }
        }
        if let Some(cap) = constraints.max_rate {
            if o.rate > cap + 1e-9 {
                continue;
            }
        }
        if let Some(cap) = constraints.max_points {
            if o.points > cap + 1e-9 {
                continue;
            }
        }
        let points_cost = amount.map(|a| o.points / 100.0 * a);
        if let (Some(cap), Some(pc)) = (constraints.max_points_cost, points_cost) {
            if pc > cap + 0.005 {
                continue;
            }
        } else if constraints.max_points_cost.is_some() && points_cost.is_none() {
            // Cap exists but the loan has no amount to price against —
            // say so once and let rate/points caps do the work.
            if !out
                .notes
                .iter()
                .any(|n| n.contains("No loan amount on file"))
            {
                out.notes.push(
                    "No loan amount on file — points cost can't be computed, \
                     so that cap wasn't applied."
                        .into(),
                );
            }
        }
        if !fits_amount(o) {
            continue;
        }
        out.matches.push(PricingSuggestion {
            sheet_option_id: o.id,
            product: o.product.clone(),
            rate: o.rate,
            points: o.points,
            monthly_pi: amount
                .zip(term)
                .and_then(|(a, m)| crate::pricing::monthly_pi(a, o.rate, m)),
            points_cost,
            comp_plan: o.comp_plan.clone(),
            tags: Vec::new(),
        });
    }

    // Best rate first; tags flag the stand-outs so the LO can scan.
    out.matches
        .sort_by(|a, b| a.rate.total_cmp(&b.rate).then(a.points.total_cmp(&b.points)));
    if !out.matches.is_empty() {
        out.matches[0].tags.push("lowest rate".into());
        if let Some((i, _)) = out
            .matches
            .iter()
            .enumerate()
            .filter(|(_, m)| m.monthly_pi.is_some())
            .min_by(|(_, a), (_, b)| a.monthly_pi.unwrap().total_cmp(&b.monthly_pi.unwrap()))
        {
            if !out.matches[i].tags.contains(&"lowest rate".to_string()) {
                out.matches[i].tags.push("lowest payment".into());
            }
        }
        if let Some((i, _)) = out
            .matches
            .iter()
            .enumerate()
            .filter(|(_, m)| m.points_cost.is_some())
            .min_by(|(_, a), (_, b)| {
                a.points_cost.unwrap().total_cmp(&b.points_cost.unwrap())
            })
        {
            if !out.matches[i].tags.contains(&"lowest rate".to_string())
                && !out.matches[i].tags.contains(&"lowest payment".to_string())
            {
                out.matches[i].tags.push("least cash to close".into());
            }
        }
    } else if constraints.max_rate.is_some()
        || constraints.max_points.is_some()
        || constraints.max_points_cost.is_some()
    {
        // Nothing fit. Point at the row that came closest so the LO
        // knows whether the caps were nearly achievable or hopeless.
        let scored = options.iter().filter_map(|o| {
            if let Some(p) = &constraints.product {
                if &o.product != p {
                    return None;
                }
            }
            let pc = amount.map(|a| o.points / 100.0 * a);
            let mut worst = 1.0f64;
            if let Some(cap) = constraints.max_rate {
                worst = worst.max(o.rate / cap);
            }
            if let Some(cap) = constraints.max_points {
                worst = worst.max(o.points / cap);
            }
            if let (Some(cap), Some(cost)) = (constraints.max_points_cost, pc) {
                worst = worst.max(cost / cap);
            }
            Some((worst, o, pc))
        });
        if let Some((_, o, pc)) = scored.min_by(|a, b| a.0.total_cmp(&b.0)) {
            out.notes.push(format!(
                "Nothing fit those caps — closest was {} at {:.3}% with {} \
                 points ({} in points cost).",
                o.product.replace('_', " "),
                o.rate,
                o.points,
                pc.map(crate::loans::screens::pricing::money)
                    .unwrap_or_else(|| "?".into()),
            ));
        } else if let Some(product) = &constraints.product {
            out.notes.push(format!(
                "No {} rows on the active sheet.",
                product.replace('_', " ")
            ));
        }
    }

    out
}

