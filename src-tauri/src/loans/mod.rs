pub mod demo;
pub mod screens;

use chrono::{DateTime, Utc};
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
