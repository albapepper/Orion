// Demo data shown when no database is configured yet, so the app is fully
// explorable on first run. Replaced by real rows once the wizard connects
// a database.

use super::{Borrower, LoanIncome, LoanSummary};
use chrono::Utc;
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
    }]
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
            employer_name: Some("Cascade Medical Group".into()),
            years_on_job: Some(1.5),
            notes: Some("Annual bonus averaged over 12 months".into()),
        },
    ]
}
