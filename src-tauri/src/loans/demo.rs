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
        },
        UserAccount {
            id: id("00000000-0000-0000-0000-000000000d03"),
            email: "pri.nair@orion.demo".into(),
            name: "Priya Nair".into(),
            role: "processor".into(),
        },
        UserAccount {
            id: id(DEMO_UW_ID),
            email: "dana.reyes@orion.demo".into(),
            name: "Dana Reyes".into(),
            role: "underwriter".into(),
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

pub fn loan_summaries() -> Vec<LoanSummary> {
    vec![
        LoanSummary {
            id: id(DEMO_LOAN_ID),
            loan_number: "ORN-2026-0001".into(),
            status: "processing".into(),
            borrower_name: Some("Jordan Ellis".into()),
            assigned_to_name: Some("Demo User".into()),
            updated_at: Utc::now(),
            flagged_reviews: 1,
        },
        LoanSummary {
            id: id("00000000-0000-0000-0000-000000000002"),
            loan_number: "ORN-2026-0002".into(),
            status: "underwriting".into(),
            borrower_name: Some("Sam Whitfield".into()),
            assigned_to_name: None,
            // Stale on purpose — the dashboard queue and the AI summary
            // should both have a stale file to point at.
            updated_at: Utc::now() - chrono::Duration::days(9),
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
        ethnicity: None,
        race: None,
        sex: None,
        collection_basis: Some("not_face_to_face".into()),
    }]
}
