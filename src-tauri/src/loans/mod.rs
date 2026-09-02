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
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct BorrowerDependent {
    pub id: Uuid,
    pub borrower_id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub age: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct LoanIncome {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub company_id: Uuid,
    pub borrower_id: Uuid,
    pub income_type: String,
    pub monthly_amount: f64,
    pub employer_name: Option<String>,
    pub years_on_job: Option<f64>,
    pub notes: Option<String>,
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
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct LoanSummary {
    pub id: Uuid,
    pub loan_number: String,
    pub status: String,
    pub borrower_name: Option<String>,
    pub assigned_to_name: Option<String>,
    pub updated_at: DateTime<Utc>,
}
