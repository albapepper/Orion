pub mod demo;
pub mod screens;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Loan {
    pub id: Uuid,
    pub company_id: Uuid,
    pub loan_number: String,
    pub status: String,
    pub assigned_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Borrower {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_primary: bool,
    pub middle_name: Option<String>,
    pub suffix_name: Option<String>,
    /// Never logged to audit or sent to AI verbatim — masked last-4 only.
    pub ssn: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub marital_status: Option<String>,
    pub citizenship_status: Option<String>,
    // Mailing address (URLA p.1) — MISMO: PARTY/ADDRESSES.
    pub mailing_address_line: Option<String>,
    pub mailing_city: Option<String>,
    pub mailing_state: Option<String>,
    pub mailing_postal_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Employment {
    pub id: Uuid,
    pub borrower_id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub employer_name: Option<String>,
    pub position: Option<String>,
    pub self_employed: bool,
    pub monthly_income: Option<f64>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_current: bool,
    pub months_in_line_of_work: Option<i32>,
    pub business_phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Asset {
    pub id: Uuid,
    pub borrower_id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub account_type: String,
    pub institution: Option<String>,
    pub balance: f64,
    pub gift_donor: Option<String>,
    pub gift_relationship: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Liability {
    pub id: Uuid,
    pub borrower_id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub liability_type: String,
    pub creditor_name: Option<String>,
    pub monthly_payment: Option<f64>,
    pub unpaid_balance: Option<f64>,
    pub payoff_at_close: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct SubjectProperty {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub address_line: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub property_type: String,
    pub occupancy: String,
    pub year_built: Option<i32>,
    pub units: i32,
    pub estimated_value: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ReoProperty {
    pub id: Uuid,
    pub borrower_id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub address_line: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub property_type: String,
    pub market_value: Option<f64>,
    pub mortgage_payment: Option<f64>,
    pub rental_income: Option<f64>,
    pub disposition: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Declaration {
    pub id: Uuid,
    pub borrower_id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub question_key: String,
    pub answer: Option<bool>,
    pub explanation: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Demographics {
    pub id: Uuid,
    pub borrower_id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    /// All that apply, per MISMO/HMDA — or exactly ["not_provided"]
    /// when the borrower declines. Empty = not collected yet.
    pub ethnicity_choices: Vec<String>,
    /// Same contract as `ethnicity_choices`.
    pub race_choices: Vec<String>,
    pub sex: Option<String>,
    pub collection_basis: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct BorrowerDependent {
    pub id: Uuid,
    pub borrower_id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub age: Option<i32>,
}

/// The closing appointment (P5-05): when, where, who conducts it, and
/// the cost summary the borrower signs against.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct LoanClosing {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub closing_date: Option<NaiveDate>,
    pub closing_location: Option<String>,
    pub settlement_agent: Option<String>,
    pub closing_costs: Option<f64>,
    pub notes: Option<String>,
}

/// The disbursement (P5-07): the day money moved and how to trace it.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct LoanFunding {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub funded_date: Option<NaiveDate>,
    pub method: Option<String>,
    pub reference: Option<String>,
    pub amount: Option<f64>,
    pub notes: Option<String>,
}

/// A party on the deal (P5-09): realtor, title, escrow, insurance —
/// the humans a loan team actually calls.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct LoanContact {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub role: String,
    pub name: String,
    pub company: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct LoanIncome {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub borrower_id: Uuid,
    pub income_type: String,
    pub monthly_amount: f64,
    pub prior_year_monthly: Option<f64>,
    pub verified: bool,
    pub employer_name: Option<String>,
    pub years_on_job: Option<f64>,
    pub notes: Option<String>,
}

/// The editable shape of a borrower row (P3-17 added the mailing
/// address). Frontend payload and the UPDATE share this struct.
#[derive(Debug, Deserialize)]
pub struct BorrowerUpdate {
    pub first_name: String,
    pub last_name: String,
    pub middle_name: Option<String>,
    pub suffix_name: Option<String>,
    pub ssn: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub marital_status: Option<String>,
    pub citizenship_status: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub mailing_address_line: Option<String>,
    pub mailing_city: Option<String>,
    pub mailing_state: Option<String>,
    pub mailing_postal_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct LoanTerms {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub loan_purpose: String,
    pub mortgage_type: String,
    pub amortization_type: String,
    pub loan_amount: Option<f64>,
    pub interest_rate: Option<f64>,
    pub term_months: Option<i32>,
    pub lien_priority: String,
    pub property_value: Option<f64>,
    pub application_date: Option<NaiveDate>,
}

/// A loan row enriched with the primary borrower's name for list views.
/// `flagged_reviews` is the pipeline's attention flag — underwriter flags
/// surface on the dashboard without opening the loan.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct LoanSummary {
    pub id: Uuid,
    pub loan_number: String,
    pub status: String,
    pub borrower_name: Option<String>,
    pub assigned_to_name: Option<String>,
    // LO contact block (P3-17) — the originator section of the 1003.
    pub assigned_to_email: Option<String>,
    pub assigned_to_phone: Option<String>,
    pub assigned_to_title: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub flagged_reviews: i64,
}

/// Underwriter review of one loan screen: verification status + notes.
/// Everyone touching the loan can read it; only underwriters write it.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UwReview {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub screen: String,
    pub status: String,
    pub note: Option<String>,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by_name: Option<String>,
}

/// A signed-in identity. There is no password auth — the desktop app picks
/// a persona ("sign in as") so locks, audit, and field permissions bind to
/// a real user row.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UserAccount {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: String,
    // Contact block (P3-17) and team-management state (P5-03).
    pub phone: Option<String>,
    pub title: Option<String>,
    pub active: bool,
}
