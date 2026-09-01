// Runtime-checked sqlx queries. These can be migrated to the compile-time
// `query!` macros once a dev database + `sqlx prepare` workflow is in place.

use crate::loans::{Borrower, Loan, LoanIncome, LoanSummary};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_loans(pool: &PgPool, company_id: Uuid) -> anyhow::Result<Vec<LoanSummary>> {
    let rows = sqlx::query_as::<_, LoanSummary>(
        r#"
        SELECT l.id, l.loan_number, l.status, l.updated_at,
               (b.first_name || ' ' || b.last_name) AS borrower_name,
               u.name AS assigned_to_name
        FROM loans l
        LEFT JOIN borrowers b ON b.loan_id = l.id AND b.is_primary
        LEFT JOIN users u ON u.id = l.assigned_to
        WHERE l.company_id = $1
        ORDER BY l.updated_at DESC
        "#,
    )
    .bind(company_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_loan(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Option<Loan>> {
    let row = sqlx::query_as::<_, Loan>("SELECT * FROM loans WHERE id = $1")
        .bind(loan_id)
        .fetch_optional(pool)
        .await?;
    Ok(row)
}

pub async fn get_borrowers(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<Borrower>> {
    let rows = sqlx::query_as::<_, Borrower>(
        "SELECT id, loan_id, company_id, first_name, last_name, email, phone, is_primary
         FROM borrowers WHERE loan_id = $1 ORDER BY is_primary DESC, last_name",
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_income(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<LoanIncome>> {
    let rows = sqlx::query_as::<_, LoanIncome>(
        r#"
        SELECT id, loan_id, company_id, borrower_id, income_type,
               monthly_amount::FLOAT8 AS monthly_amount,
               employer_name,
               years_on_job::FLOAT8 AS years_on_job,
               notes
        FROM loan_income WHERE loan_id = $1 ORDER BY monthly_amount DESC
        "#,
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_income(
    pool: &PgPool,
    id: Option<Uuid>,
    loan_id: Uuid,
    company_id: Uuid,
    borrower_id: Uuid,
    income_type: &str,
    monthly_amount: f64,
    employer_name: Option<&str>,
    years_on_job: Option<f64>,
    notes: Option<&str>,
) -> anyhow::Result<LoanIncome> {
    let row = sqlx::query_as::<_, LoanIncome>(
        r#"
        INSERT INTO loan_income
            (id, loan_id, company_id, borrower_id, income_type,
             monthly_amount, employer_name, years_on_job, notes, updated_at)
        VALUES
            (COALESCE($1, gen_random_uuid()), $2, $3, $4, $5, $6, $7, $8, $9, now())
        ON CONFLICT (id) DO UPDATE SET
            income_type = EXCLUDED.income_type,
            monthly_amount = EXCLUDED.monthly_amount,
            employer_name = EXCLUDED.employer_name,
            years_on_job = EXCLUDED.years_on_job,
            notes = EXCLUDED.notes,
            updated_at = now()
        RETURNING id, loan_id, company_id, borrower_id, income_type,
                  monthly_amount::FLOAT8 AS monthly_amount,
                  employer_name,
                  years_on_job::FLOAT8 AS years_on_job,
                  notes
        "#,
    )
    .bind(id)
    .bind(loan_id)
    .bind(company_id)
    .bind(borrower_id)
    .bind(income_type)
    .bind(monthly_amount)
    .bind(employer_name)
    .bind(years_on_job)
    .bind(notes)
    .fetch_one(pool)
    .await?;
    Ok(row)
}
