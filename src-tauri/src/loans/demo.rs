// Demo data shown when no database is configured yet, so the app is fully
// explorable on first run. Replaced by real rows once the wizard connects
// a database.

use super::{Asset, Borrower, BorrowerDependent, Employment, LoanIncome, LoanSummary, LoanTerms};
use chrono::{NaiveDate, Utc};
use uuid::Uuid;

pub const DEMO_LOAN_ID: &str = "00000000-0000-0000-0000-000000000001";
const DEMO_BORROWER_ID: &str = "00000000-0000-0000-0000-000000000101";
const DEMO_COMPANY_ID: &str = "00000000-0000-0000-0000-000000000009";

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
        },
        LoanSummary {
            id: id("00000000-0000-0000-0000-000000000002"),
            loan_number: "ORN-2026-0002".into(),
            status: "underwriting".into(),
            borrower_name: Some("Sam Whitfield".into()),
            assigned_to_name: None,
            updated_at: Utc::now(),
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
