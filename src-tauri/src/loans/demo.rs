// Demo data shown when no database is configured yet, so the app is fully
// explorable on first run. Replaced by real rows once the wizard connects
// a database.

use super::{
    Asset, Borrower, BorrowerDependent, Declaration, Demographics, Employment, Liability,
    LoanIncome, LoanSummary, LoanTerms, ReoProperty, SubjectProperty, UserAccount, UwReview,
};
use chrono::{NaiveDate, Utc};
use uuid::Uuid;

pub const DEMO_LOAN_ID: &str = "00000000-0000-0000-0000-000000000001";
const DEMO_BORROWER_ID: &str = "00000000-0000-0000-0000-000000000101";
const DEMO_COMPANY_ID: &str = "00000000-0000-0000-0000-000000000009";
pub const DEMO_OFFICER_ID: &str = "00000000-0000-0000-0000-000000000d01";
pub const DEMO_UW_ID: &str = "00000000-0000-0000-0000-000000000d02";

/// The persona the app starts as in demo mode: a loan officer, so the
/// underwriter column starts locked and switching (Settings) is demonstrable.
pub fn users() -> Vec<UserAccount> {
    vec![
        UserAccount {
            id: id(DEMO_OFFICER_ID),
            email: "alex.kim@orion.demo".into(),
            name: "Alex Kim".into(),
            role: "loan_officer".into(),
            phone: Some("555-0102".into()),
            title: Some("Senior Loan Officer".into()),
            active: true,
        },
        UserAccount {
            id: id("00000000-0000-0000-0000-000000000d03"),
            email: "pri.nair@orion.demo".into(),
            name: "Priya Nair".into(),
            role: "processor".into(),
            phone: Some("555-0107".into()),
            title: Some("Processor".into()),
            active: true,
        },
        UserAccount {
            id: id(DEMO_UW_ID),
            email: "dana.reyes@orion.demo".into(),
            name: "Dana Reyes".into(),
            role: "underwriter".into(),
            phone: Some("555-0111".into()),
            title: Some("Underwriter".into()),
            active: true,
        },
    ]
}

pub fn default_user() -> UserAccount {
    users().into_iter().next().unwrap()
}

pub fn uw_reviews(loan_id: Uuid) -> Vec<UwReview> {
    vec![
        UwReview {
            id: id("00000000-0000-0000-0000-000000000c01"),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            screen: "income".into(),
            status: "verified".into(),
            note: Some("Base + bonus match paystubs; VOE on file.".into()),
            updated_by: Some(id(DEMO_UW_ID)),
            updated_at: Utc::now(),
            updated_by_name: Some("Dana Reyes".into()),
        },
        UwReview {
            id: id("00000000-0000-0000-0000-000000000c02"),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            screen: "assets".into(),
            status: "flagged".into(),
            note: Some(
                "Large deposit — $25k gift needs a signed gift letter and sourcing.".into(),
            ),
            updated_by: Some(id(DEMO_UW_ID)),
            updated_at: Utc::now(),
            updated_by_name: Some("Dana Reyes".into()),
        },
    ]
}

fn id(s: &str) -> Uuid {
    Uuid::parse_str(s).unwrap()
}

pub fn loan_summaries() -> Vec<LoanSummary> {    vec![
        LoanSummary {
            id: id(DEMO_LOAN_ID),
            loan_number: "ORN-2026-0001".into(),
            status: "processing".into(),
            borrower_name: Some("Jordan Ellis".into()),
            assigned_to_name: Some("Demo User".into()),
            assigned_to_email: Some("alex.kim@oriondemo.example".into()),
            assigned_to_phone: Some("555-0102".into()),
            assigned_to_title: Some("Senior Loan Officer".into()),
            updated_at: Utc::now(),
            flagged_reviews: 1,
        },
        LoanSummary {
            id: id("00000000-0000-0000-0000-000000000002"),
            loan_number: "ORN-2026-0002".into(),
            status: "underwriting".into(),
            borrower_name: Some("Sam Whitfield".into()),
            assigned_to_name: None,
            assigned_to_email: None,
            assigned_to_phone: None,
            assigned_to_title: None,
            // Stale on purpose — the dashboard queue and the AI summary
            // should both have a stale file to point at.
            updated_at: Utc::now() - chrono::Duration::days(9),
            flagged_reviews: 0,
        },
        LoanSummary {
            id: id("00000000-0000-0000-0000-000000000003"),
            loan_number: "ORN-2026-0003".into(),
            status: "application".into(),
            borrower_name: Some("Avery Chen".into()),
            assigned_to_name: Some("Demo User".into()),
            assigned_to_email: Some("alex.kim@oriondemo.example".into()),
            assigned_to_phone: Some("555-0102".into()),
            assigned_to_title: Some("Senior Loan Officer".into()),
            updated_at: Utc::now() - chrono::Duration::hours(5),
            flagged_reviews: 0,
        },
    ]
}

pub fn borrowers(loan_id: Uuid) -> Vec<Borrower> {
    vec![Borrower {
        id: id(DEMO_BORROWER_ID),
        loan_id,
        company_id: id(DEMO_COMPANY_ID),
        first_name: "Jordan".into(),
        last_name: "Ellis".into(),
        email: Some("jordan.ellis@example.com".into()),
        phone: Some("555-0142".into()),
        is_primary: true,
        middle_name: Some("Alex".into()),
        suffix_name: None,
        ssn: Some("123-45-6789".into()),
        birth_date: NaiveDate::from_ymd_opt(1985, 3, 14),
        marital_status: Some("married".into()),
        citizenship_status: Some("us_citizen".into()),
        mailing_address_line: Some("412 Juniper Court".into()),
        mailing_city: Some("Bellingham".into()),
        mailing_state: Some("WA".into()),
        mailing_postal_code: Some("98225".into()),
    }]
}

pub fn dependents(loan_id: Uuid) -> Vec<BorrowerDependent> {
    vec![
        BorrowerDependent {
            id: id("00000000-0000-0000-0000-000000000401"),
            borrower_id: id(DEMO_BORROWER_ID),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            age: Some(12),
        },
        BorrowerDependent {
            id: id("00000000-0000-0000-0000-000000000402"),
            borrower_id: id(DEMO_BORROWER_ID),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            age: Some(7),
        },
    ]
}

pub fn income(loan_id: Uuid) -> Vec<LoanIncome> {
    vec![
        LoanIncome {
            id: id("00000000-0000-0000-0000-000000000201"),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            borrower_id: id(DEMO_BORROWER_ID),
            income_type: "base".into(),
            monthly_amount: 7250.0,
            prior_year_monthly: Some(6900.0),
            verified: true,
            employer_name: Some("Cascade Medical Group".into()),
            years_on_job: Some(4.5),
            notes: None,
        },
        LoanIncome {
            id: id("00000000-0000-0000-0000-000000000202"),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            borrower_id: id(DEMO_BORROWER_ID),
            income_type: "bonus".into(),
            monthly_amount: 850.0,
            prior_year_monthly: Some(700.0),
            verified: false,
            employer_name: Some("Cascade Medical Group".into()),
            years_on_job: Some(1.5),
            notes: Some("Annual bonus averaged over 12 months".into()),
        },
    ]
}

pub fn loan_terms(loan_id: Uuid) -> LoanTerms {
    LoanTerms {
        id: id("00000000-0000-0000-0000-000000000301"),
        loan_id,
        company_id: id(DEMO_COMPANY_ID),
        loan_purpose: "purchase".into(),
        mortgage_type: "conventional".into(),
        amortization_type: "fixed".into(),
        loan_amount: Some(560_000.0),
        interest_rate: Some(6.375),
        term_months: Some(360),
        lien_priority: "first".into(),
        property_value: Some(700_000.0),
        application_date: NaiveDate::from_ymd_opt(2026, 9, 1),
    }
}

pub fn employments(loan_id: Uuid) -> Vec<Employment> {
    vec![
        Employment {
            id: id("00000000-0000-0000-0000-000000000501"),
            borrower_id: id(DEMO_BORROWER_ID),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            employer_name: Some("Cascade Medical Group".into()),
            position: Some("Physician Assistant".into()),
            self_employed: false,
            monthly_income: Some(7_250.0),
            start_date: NaiveDate::from_ymd_opt(2021, 11, 1),
            end_date: None,
            is_current: true,
            months_in_line_of_work: Some(96),
            business_phone: Some("555-0117".into()),
        },
        Employment {
            id: id("00000000-0000-0000-0000-000000000502"),
            borrower_id: id(DEMO_BORROWER_ID),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            employer_name: Some("Lakeside Family Clinic".into()),
            position: Some("Physician Assistant".into()),
            self_employed: false,
            monthly_income: Some(6_400.0),
            start_date: NaiveDate::from_ymd_opt(2018, 6, 1),
            end_date: NaiveDate::from_ymd_opt(2021, 10, 31),
            is_current: false,
            months_in_line_of_work: Some(96),
            business_phone: None,
        },
    ]
}

pub fn assets(loan_id: Uuid) -> Vec<Asset> {
    vec![
        Asset {
            id: id("00000000-0000-0000-0000-000000000601"),
            borrower_id: id(DEMO_BORROWER_ID),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            account_type: "checking".into(),
            institution: Some("Puget Sound Credit Union".into()),
            balance: 48_200.0,
            gift_donor: None,
            gift_relationship: None,
            notes: None,
        },
        Asset {
            id: id("00000000-0000-0000-0000-000000000602"),
            borrower_id: id(DEMO_BORROWER_ID),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            account_type: "savings".into(),
            institution: Some("Alaska USA".into()),
            balance: 92_500.0,
            gift_donor: None,
            gift_relationship: None,
            notes: None,
        },
        Asset {
            id: id("00000000-0000-0000-0000-000000000603"),
            borrower_id: id(DEMO_BORROWER_ID),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            account_type: "gift".into(),
            institution: None,
            balance: 25_000.0,
            gift_donor: Some("Ruth Ellis (mother)".into()),
            gift_relationship: Some("parent".into()),
            notes: Some("Gift letter on file".into()),
        },
    ]
}

pub fn liabilities(loan_id: Uuid) -> Vec<Liability> {
    vec![
        Liability {
            id: id("00000000-0000-0000-0000-000000000701"),
            borrower_id: id(DEMO_BORROWER_ID),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            liability_type: "mortgage".into(),
            creditor_name: Some("Meridian Home Loans".into()),
            monthly_payment: Some(1_480.0),
            unpaid_balance: Some(289_000.0),
            payoff_at_close: false,
            notes: Some("Primary residence being sold — will pay off at close".into()),
        },
        Liability {
            id: id("00000000-0000-0000-0000-000000000604"),
            borrower_id: id(DEMO_BORROWER_ID),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            liability_type: "installment".into(),
            creditor_name: Some("Cascade Auto Finance".into()),
            monthly_payment: Some(415.0),
            unpaid_balance: Some(11_200.0),
            payoff_at_close: false,
            notes: None,
        },
    ]
}

pub fn subject_property(loan_id: Uuid) -> SubjectProperty {
    SubjectProperty {
        id: id("00000000-0000-0000-0000-000000000801"),
        loan_id,
        company_id: id(DEMO_COMPANY_ID),
        address_line: Some("1417 Alder Ridge Rd".into()),
        city: Some("Bellingham".into()),
        state: Some("WA".into()),
        postal_code: Some("98225".into()),
        property_type: "detached".into(),
        occupancy: "primary".into(),
        year_built: Some(1998),
        units: 1,
        estimated_value: Some(700_000.0),
        notes: None,
    }
}

pub fn reo(loan_id: Uuid) -> Vec<ReoProperty> {
    vec![ReoProperty {
        id: id("00000000-0000-0000-0000-000000000901"),
        borrower_id: id(DEMO_BORROWER_ID),
        loan_id,
        company_id: id(DEMO_COMPANY_ID),
        address_line: Some("88 Harbor View Ln, Apt 4".into()),
        city: Some("Bellingham".into()),
        state: Some("WA".into()),
        postal_code: Some("98225".into()),
        property_type: "condo".into(),
        market_value: Some(415_000.0),
        mortgage_payment: Some(1_480.0),
        rental_income: Some(1_950.0),
        disposition: "pending_sale".into(),
        notes: Some("Closing mid-October".into()),
    }]
}

pub fn declarations(loan_id: Uuid) -> Vec<Declaration> {
    let decl = |key: &str, did: &str, answer: Option<bool>, explanation: Option<&str>| Declaration {
        id: id(did),
        borrower_id: id(DEMO_BORROWER_ID),
        loan_id,
        company_id: id(DEMO_COMPANY_ID),
        question_key: key.into(),
        answer,
        explanation: explanation.map(|e| e.into()),
    };
    vec![
        decl("primary_residence", "00000000-0000-0000-0000-000000000a01", Some(true), None),
        decl("us_citizen", "00000000-0000-0000-0000-000000000a02", Some(true), None),
        decl("bankruptcy_7yr", "00000000-0000-0000-0000-000000000a03", Some(false), None),
        decl("alimony_child_support", "00000000-0000-0000-0000-000000000a04", Some(true), Some("$450/month until 2031, per divorce decree")),

    ]
}
pub fn demographics(loan_id: Uuid) -> Vec<Demographics> {
    vec![Demographics {
        id: id("00000000-0000-0000-0000-000000000b01"),
        borrower_id: id(DEMO_BORROWER_ID),
        loan_id,
        company_id: id(DEMO_COMPANY_ID),
        ethnicity_choices: vec!["not_hispanic_or_latino".into()],
        race_choices: vec!["white".into(), "native_hawaiian_or_pacific_islander".into()],
        sex: Some("male".into()),
        collection_basis: Some("not_face_to_face".into()),
    }]
}

// ── Tasks ────────────────────────────────────────────────────────────

fn loan_number_for(loan_id: Uuid) -> &'static str {
    match loan_id {
        x if x == id("00000000-0000-0000-0000-000000000002") => "ORN-2026-0002",
        x if x == id("00000000-0000-0000-0000-000000000003") => "ORN-2026-0003",
        _ => "ORN-2026-0001",
    }
}

fn demo_user_name(user_id: Uuid) -> Option<String> {
    match user_id {
        x if x == id(DEMO_OFFICER_ID) => Some("Alex Kim".into()),
        x if x == id(DEMO_UW_ID) => Some("Dana Reyes".into()),
        x if x == id("00000000-0000-0000-0000-000000000d03") => Some("Priya Nair".into()),
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
fn task(
    suffix: &str,
    loan_id: Uuid,
    title: &str,
    screen: Option<&str>,
    assigned_to: Option<Uuid>,
    status: &str,
    priority: &str,
    due_days: Option<i64>,
) -> crate::tasks::Task {
    crate::tasks::Task {
        id: id(&format!("00000000-0000-0000-0000-2000000000{suffix}")),
        loan_id,
        company_id: id(DEMO_COMPANY_ID),
        title: title.into(),
        description: None,
        assigned_to,
        screen: screen.map(Into::into),
        status: status.into(),
        priority: priority.into(),
        due_date: due_days.map(|d| Utc::now() + chrono::Duration::days(d)),
        created_by: id(DEMO_OFFICER_ID),
        created_at: Utc::now(),
        completed_at: (status == "done").then(Utc::now),
        assigned_to_name: assigned_to.and_then(demo_user_name),
        loan_number: loan_number_for(loan_id).into(),
    }
}

/// Per-loan task lists for the TaskPanel. Loan 1 carries the interesting
/// mix (overdue + unassigned, due soon, done); the other loans get one
/// row each so the unified view has spread.
pub fn tasks(loan_id: Uuid) -> Vec<crate::tasks::Task> {
    let officer = id(DEMO_OFFICER_ID);
    let uw = id(DEMO_UW_ID);
    let processor = id("00000000-0000-0000-0000-000000000d03");
    match loan_number_for(loan_id) {
        "ORN-2026-0002" => vec![task(
            "01", loan_id, "Assign a processor", None, None, "open", "low", None,
        )],
        "ORN-2026-0003" => vec![task(
            "02",
            loan_id,
            "Schedule intake call",
            None,
            Some(officer),
            "open",
            "normal",
            Some(5),
        )],
        _ => vec![
            task(
                "03",
                loan_id,
                "Gift letter for the $25k deposit",
                Some("assets"),
                None,
                "open",
                "high",
                Some(-1),
            ),
            task(
                "04",
                loan_id,
                "VOE for current employer",
                Some("employment"),
                Some(processor),
                "open",
                "normal",
                Some(2),
            ),
            task(
                "05",
                loan_id,
                "Pull preliminary credit",
                Some("credit"),
                Some(uw),
                "done",
                "normal",
                None,
            ),
        ],
    }
}

/// The unified Tasks view in demo mode: every demo loan's tasks.
pub fn all_tasks() -> Vec<crate::tasks::Task> {
    [
        DEMO_LOAN_ID,
        "00000000-0000-0000-0000-000000000002",
        "00000000-0000-0000-0000-000000000003",
    ]
    .iter()
    .flat_map(|s| tasks(id(s)))
    .collect()
}

// ── Documents & conditions ───────────────────────────────────────────

pub fn documents(loan_id: Uuid) -> Vec<crate::documents::Document> {
    let officer = id(DEMO_OFFICER_ID);
    let base = crate::documents::Document {
        id: id("00000000-0000-0000-0000-300000000001"),
        loan_id,
        company_id: id(DEMO_COMPANY_ID),
        title: String::new(),
        doc_type: "other".into(),
        screen: None,
        file_name: None,
        mime_type: Some("application/pdf".into()),
        size_bytes: Some(184_320),
        uploaded_by: Some(officer),
        uploaded_at: Utc::now() - chrono::Duration::days(1),
        removed_at: None,
        uploaded_by_name: Some("Alex Kim".into()),
    };
    let doc = move |suffix: &str, title: &str, doc_type: &str, screen: Option<&str>| {
        let mut d = base.clone();
        d.id = id(&format!("00000000-0000-0000-0000-3000000000{suffix}"));
        d.title = title.into();
        d.doc_type = doc_type.into();
        d.screen = screen.map(Into::into);
        d
    };
    match loan_number_for(loan_id) {
        "ORN-2026-0002" => vec![doc("01", "Purchase agreement", "application", Some("property"))],
        "ORN-2026-0003" => vec![],
        _ => vec![
            doc("02", "Paystubs — last 30 days", "income_verification", Some("income")),
            doc("03", "Bank statement — June", "bank_statement", Some("assets")),
            doc("04", "W-2s 2024 & 2025", "tax_return", Some("income")),
        ],
    }
}

pub fn conditions(loan_id: Uuid) -> Vec<crate::conditions::Condition> {
    let officer = id(DEMO_OFFICER_ID);
    let uw = id(DEMO_UW_ID);
    let base = crate::conditions::Condition {
        id: id("00000000-0000-0000-0000-400000000001"),
        loan_id,
        company_id: id(DEMO_COMPANY_ID),
        title: String::new(),
        detail: None,
        source: "underwriter".into(),
        status: "open".into(),
        priority: "normal".into(),
        clear_by: None,
        cleared_at: None,
        cleared_by: None,
        created_by: Some(uw),
        created_at: Utc::now() - chrono::Duration::days(2),
        updated_at: Utc::now() - chrono::Duration::days(2),
        cleared_by_name: None,
    };
    let cond = move |suffix: &str, title: &str, status: &str, priority: &str, due_days: Option<i64>| {
        let mut c = base.clone();
        c.id = id(&format!("00000000-0000-0000-0000-4000000000{suffix}"));
        c.title = title.into();
        c.status = status.into();
        c.priority = priority.into();
        c.clear_by = due_days.map(|d| Utc::now() + chrono::Duration::days(d));
        c.cleared_at = matches!(status, "cleared" | "waived").then(|| Utc::now() - chrono::Duration::days(1));
        c.cleared_by = matches!(status, "cleared" | "waived").then_some(officer);
        c.cleared_by_name = matches!(status, "cleared" | "waived").then(|| "Alex Kim".to_string());
        c
    };
    match loan_number_for(loan_id) {
        "ORN-2026-0002" => vec![
            cond("01", "Signed gift letter", "open", "high", Some(1)),
            cond("02", "Updated VOE within 90 days", "submitted", "normal", Some(4)),
        ],
        "ORN-2026-0003" => vec![],
        _ => vec![
            cond(
                "02",
                "Gift letter + sourcing for $25k deposit",
                "open",
                "high",
                Some(-2),
            ),
            cond("03", "Homeowners insurance binder", "open", "normal", Some(6)),
            cond("04", "Appraisal received", "cleared", "normal", None),
        ],
    }
}

/// The demo underwriter's user id — used by the fake presence lock.
pub fn underwriter_id() -> Uuid {
    id(DEMO_UW_ID)
}

/// Parse one of the fixed demo ids.
pub fn loan_id(s: &str) -> Uuid {
    id(s)
}

// ── Closing & funding ────────────────────────────────────────────────

/// Loan 1 has a closing on the books; the rest haven't gotten there.
pub fn loan_closing(loan_id: Uuid) -> Option<crate::loans::LoanClosing> {
    if loan_number_for(loan_id) != "ORN-2026-0001" {
        return None;
    }
    Some(crate::loans::LoanClosing {
        id: id("00000000-0000-0000-0000-800000000001"),
        loan_id,
        company_id: id(DEMO_COMPANY_ID),
        closing_date: NaiveDate::from_ymd_opt(2026, 9, 24),
        closing_location: Some("Puget Sound Title — Bellingham office".into()),
        settlement_agent: Some("Marcy Oduya".into()),
        closing_costs: Some(11_240.0),
        notes: Some("Borrower + spouse sign; wire remainder of down payment 24h ahead.".into()),
    })
}

/// Loan 1 has its deal team on file; the rest are still forming.
pub fn loan_contacts(loan_id: Uuid) -> Vec<crate::loans::LoanContact> {
    let contact = |suffix: &str, role: &str, name: &str, company: &str, phone: &str| {
        crate::loans::LoanContact {
            id: id(&format!("00000000-0000-0000-0000-9000000000{suffix}")),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            role: role.into(),
            name: name.into(),
            company: Some(company.into()),
            email: None,
            phone: Some(phone.into()),
            notes: None,
        }
    };
    match loan_number_for(loan_id) {
        "ORN-2026-0003" => vec![],
        _ => vec![
            contact(
                "01",
                "realtor",
                "Tessa Moreland",
                "Northbay Realty",
                "555-0134",
            ),
            contact(
                "02",
                "title",
                "Marcy Oduya",
                "Puget Sound Title",
                "555-0178",
            ),
        ],
    }
}

/// No demo loan has funded yet — the pipeline stops one short so the
/// funding screen still has a job to do in demos.
pub fn loan_funding(_loan_id: Uuid) -> Option<crate::loans::LoanFunding> {
    None
}

// ── AUS submissions ──────────────────────────────────────────────────

/// Demo runs the real mock engines over the demo file, so the findings
/// on screen always match the loan's actual numbers. Loan 1 starts
/// with both engines' takes on file.
pub fn aus_submissions(loan_id: Uuid) -> Vec<crate::aus::AusSubmission> {
    let terms = loan_terms(loan_id);
    let borrowers = borrowers(loan_id);
    let income = income(loan_id);
    let liabilities = liabilities(loan_id);
    let declarations = declarations(loan_id);
    let engine_submission = |engine: crate::aus::AusEngine, sid: &str| {
        let facts = crate::aus::FileFacts {
            loan_number: loan_number_for(loan_id),
            terms: Some(&terms),
            borrowers: &borrowers,
            income: &income,
            liabilities: &liabilities,
            declarations: &declarations,
        };
        let outcome = crate::aus::mock_findings(engine, &facts);
        crate::aus::AusSubmission {
            id: id(sid),
            loan_id,
            company_id: id(DEMO_COMPANY_ID),
            system: outcome.system.into(),
            case_id: outcome.case_id,
            recommendation: Some(outcome.recommendation),
            messages: serde_json::to_value(
                outcome
                    .messages
                    .iter()
                    .map(|m| serde_json::to_value(m).expect("message serializes"))
                    .collect::<Vec<_>>(),
            )
            .expect("messages serialize"),
            raw_response: None,
            submitted_by: Some(id(DEMO_OFFICER_ID)),
            submitted_at: Utc::now(),
        }
    };
    vec![
        engine_submission(
            crate::aus::AusEngine::Du,
            "00000000-0000-0000-0000-700000000001",
        ),
        engine_submission(
            crate::aus::AusEngine::Lpa,
            "00000000-0000-0000-0000-700000000002",
        ),
    ]
}

// ── Rate sheets ──────────────────────────────────────────────────────

pub fn rate_sheets() -> Vec<crate::pricing::RateSheet> {
    let company = id(DEMO_COMPANY_ID);
    vec![crate::pricing::RateSheet {
        id: id("00000000-0000-0000-0000-500000000001"),
        company_id: company,
        lender: "Premier Lending Group".into(),
        effective_date: NaiveDate::from_ymd_opt(2026, 9, 1),
        expires_date: NaiveDate::from_ymd_opt(2026, 9, 30),
        is_active: true,
        created_at: Utc::now(),
    }]
}

pub fn rate_options(rate_sheet_id: Uuid) -> Vec<crate::pricing::RateOption> {
    let company = id(DEMO_COMPANY_ID);
    let sheet = if rate_sheet_id == id("00000000-0000-0000-0000-500000000001") {
        rate_sheet_id
    } else {
        id("00000000-0000-0000-0000-500000000001")
    };
    let opt = |suffix: &str, product: &str, rate: f64, points: f64, comp: &str| {
        crate::pricing::RateOption {
            id: id(&format!("00000000-0000-0000-0000-5000000001{suffix}")),
            rate_sheet_id: sheet,
            company_id: company,
            product: product.into(),
            rate,
            points,
            comp_plan: Some(comp.into()),
            min_loan_amount: None,
            max_loan_amount: None,
            created_at: Utc::now(),
        }
    };
    vec![
        opt("01", "30yr_fixed", 6.375, 1.5, "PLG-Standard"),
        opt("02", "30yr_fixed", 6.500, 1.0, "PLG-Standard"),
        opt("03", "30yr_fixed", 6.625, 0.5, "PLG-Standard"),
        opt("04", "30yr_fixed", 6.750, 0.0, "PLG-Standard"),
        opt("05", "15yr_fixed", 5.875, 1.0, "PLG-Standard"),
        opt("06", "7_1_arm", 6.125, 0.75, "PLG-Arm"),
    ]
}

// ── Loan pricing options ─────────────────────────────────────────────

pub fn loan_pricing(loan_id: Uuid) -> Vec<crate::pricing::LoanPricingOption> {
    let company = id(DEMO_COMPANY_ID);
    let quote = |suffix: &str, product: &str, rate: f64, points: f64, pi: f64, selected: bool| {
        crate::pricing::LoanPricingOption {
            id: id(&format!("00000000-0000-0000-0000-6000000000{suffix}")),
            loan_id,
            company_id: company,
            product: product.into(),
            rate,
            points,
            monthly_pi: Some(pi),
            points_cost: Some(400_000.0 * points / 100.0),
            comp_plan: Some("PLG-Standard".into()),
            selected,
            created_by: Some(id(DEMO_OFFICER_ID)),
            created_at: Utc::now(),
        }
    };
    match loan_number_for(loan_id) {
        "ORN-2026-0002" => vec![quote(
            "01",
            "30yr_fixed",
            6.625,
            0.5,
            2_557.0,
            false,
        )],
        "ORN-2026-0003" => vec![],
        _ => vec![
            quote("02", "30yr_fixed", 6.375, 1.5, 2_491.0, false),
            quote("03", "30yr_fixed", 6.500, 1.0, 2_528.0, true),
            quote("04", "15yr_fixed", 5.875, 1.0, 3_362.0, false),
        ],
    }
}
