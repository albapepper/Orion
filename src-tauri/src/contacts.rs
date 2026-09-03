//! P5-09: Loan contacts — the deal's vendors and partners (realtor,
//! title, escrow, insurance). Loan-scoped rows, deliberately free of
//! borrower linkage: these people serve the file.

use crate::loans::LoanContact;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_for_loan(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<LoanContact>> {
    let rows = sqlx::query_as::<_, LoanContact>(
        "SELECT id, loan_id, company_id, role, name, company, email, phone, notes
         FROM loan_contacts WHERE loan_id = $1
         ORDER BY role, name",
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
    role: &str,
    name: &str,
    company: Option<&str>,
    email: Option<&str>,
    phone: Option<&str>,
    notes: Option<&str>,
) -> anyhow::Result<LoanContact> {
    let row = sqlx::query_as::<_, LoanContact>(
        "INSERT INTO loan_contacts (loan_id, company_id, role, name, company, email, phone, notes)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         RETURNING id, loan_id, company_id, role, name, company, email, phone, notes",
    )
    .bind(loan_id)
    .bind(company_id)
    .bind(role)
    .bind(name)
    .bind(company)
    .bind(email)
    .bind(phone)
    .bind(notes)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn remove(pool: &PgPool, id: Uuid) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM loan_contacts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
