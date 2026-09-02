// Runtime-checked sqlx queries. These can be migrated to the compile-time
// `query!` macros once a dev database + `sqlx prepare` workflow is in place.

use crate::loans::{Borrower, BorrowerDependent, Loan, LoanIncome, LoanSummary, LoanTerms};
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
        r#"
        SELECT id, loan_id, company_id, first_name, last_name, email, phone, is_primary,
               middle_name, suffix_name, ssn,
               birth_date, marital_status, citizenship_status
        FROM borrowers WHERE loan_id = $1 ORDER BY is_primary DESC, last_name
        "#,
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
pub async fn save_borrower(
    pool: &PgPool,
    borrower_id: Uuid,
    first_name: &str,
    last_name: &str,
    middle_name: Option<&str>,
    suffix_name: Option<&str>,
    ssn: Option<&str>,
    birth_date: Option<chrono::NaiveDate>,
    marital_status: Option<&str>,
    citizenship_status: Option<&str>,
    email: Option<&str>,
    phone: Option<&str>,
) -> anyhow::Result<Borrower> {
    let row = sqlx::query_as::<_, Borrower>(
        r#"
        UPDATE borrowers SET
            first_name = $2, last_name = $3, middle_name = $4, suffix_name = $5,
            ssn = $6, birth_date = $7, marital_status = $8, citizenship_status = $9,
            email = $10, phone = $11
        WHERE id = $1
        RETURNING id, loan_id, company_id, first_name, last_name, email, phone, is_primary,
                  middle_name, suffix_name, ssn,
                  birth_date, marital_status, citizenship_status
        "#,
    )
    .bind(borrower_id)
    .bind(first_name)
    .bind(last_name)
    .bind(middle_name)
    .bind(suffix_name)
    .bind(ssn)
    .bind(birth_date)
    .bind(marital_status)
    .bind(citizenship_status)
    .bind(email)
    .bind(phone)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn get_dependents(
    pool: &PgPool,
    loan_id: Uuid,
) -> anyhow::Result<Vec<BorrowerDependent>> {
    let rows = sqlx::query_as::<_, BorrowerDependent>(
        "SELECT id, borrower_id, loan_id, company_id, age
         FROM borrower_dependents WHERE loan_id = $1 ORDER BY borrower_id, created_at",
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

/// Dependents are edited as a list of ages: replace-all is the honest
/// model (MISMO carries a count + age list, not identified individuals).
pub async fn replace_dependents(
    pool: &PgPool,
    borrower_id: Uuid,
    loan_id: Uuid,
    company_id: Uuid,
    ages: &[Option<i32>],
) -> anyhow::Result<Vec<BorrowerDependent>> {
    let mut tx = pool.begin().await?;
    sqlx::query("DELETE FROM borrower_dependents WHERE borrower_id = $1")
        .bind(borrower_id)
        .execute(&mut *tx)
        .await?;
    let mut rows = Vec::with_capacity(ages.len());
    for age in ages {
        let row = sqlx::query_as::<_, BorrowerDependent>(
            r#"
            INSERT INTO borrower_dependents (borrower_id, loan_id, company_id, age)
            VALUES ($1, $2, $3, $4)
            RETURNING id, borrower_id, loan_id, company_id, age
            "#,
        )
        .bind(borrower_id)
        .bind(loan_id)
        .bind(company_id)
        .bind(age)
        .fetch_one(&mut *tx)
        .await?;
        rows.push(row);
    }
    tx.commit().await?;
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

const TERMS_COLUMNS: &str = "id, loan_id, company_id, loan_purpose, mortgage_type,
    amortization_type, loan_amount::FLOAT8 AS loan_amount, interest_rate::FLOAT8 AS interest_rate,
    term_months, lien_priority, property_value::FLOAT8 AS property_value, application_date";

pub async fn get_loan_terms(
    pool: &PgPool,
    loan_id: Uuid,
) -> anyhow::Result<Option<LoanTerms>> {
    let row = sqlx::query_as::<_, LoanTerms>(
        &format!("SELECT {TERMS_COLUMNS} FROM loan_terms WHERE loan_id = $1"),
    )
    .bind(loan_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_loan_terms(
    pool: &PgPool,
    loan_id: Uuid,
    company_id: Uuid,
    loan_purpose: &str,
    mortgage_type: &str,
    amortization_type: &str,
    loan_amount: Option<f64>,
    interest_rate: Option<f64>,
    term_months: Option<i32>,
    lien_priority: &str,
    property_value: Option<f64>,
    application_date: Option<chrono::NaiveDate>,
) -> anyhow::Result<LoanTerms> {
    let row = sqlx::query_as::<_, LoanTerms>(
        &format!(
            r#"
        INSERT INTO loan_terms
            (id, loan_id, company_id, loan_purpose, mortgage_type, amortization_type,
             loan_amount, interest_rate, term_months, lien_priority, property_value,
             application_date, updated_at)
        VALUES
            (gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, now())
        ON CONFLICT (loan_id) DO UPDATE SET
            loan_purpose = EXCLUDED.loan_purpose,
            mortgage_type = EXCLUDED.mortgage_type,
            amortization_type = EXCLUDED.amortization_type,
            loan_amount = EXCLUDED.loan_amount,
            interest_rate = EXCLUDED.interest_rate,
            term_months = EXCLUDED.term_months,
            lien_priority = EXCLUDED.lien_priority,
            property_value = EXCLUDED.property_value,
            application_date = EXCLUDED.application_date,
            updated_at = now()
        RETURNING {TERMS_COLUMNS}
        "#
        ),
    )
    .bind(loan_id)
    .bind(company_id)
    .bind(loan_purpose)
    .bind(mortgage_type)
    .bind(amortization_type)
    .bind(loan_amount)
    .bind(interest_rate)
    .bind(term_months)
    .bind(lien_priority)
    .bind(property_value)
    .bind(application_date)
    .fetch_one(pool)
    .await?;
    Ok(row)
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
