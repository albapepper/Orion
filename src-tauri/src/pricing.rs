use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

/// A lender's rate sheet: the container. One is active at a time (the
/// newest active one wins if data is messy).
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RateSheet {
    pub id: Uuid,
    pub company_id: Uuid,
    pub lender: String,
    pub effective_date: Option<NaiveDate>,
    pub expires_date: Option<NaiveDate>,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// One priceable row on a sheet: a rate for a product with its points.
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct RateOption {
    pub id: Uuid,
    pub rate_sheet_id: Uuid,
    pub company_id: Uuid,
    pub product: String,
    pub rate: f64,
    pub points: f64,
    pub comp_plan: Option<String>,
    pub min_loan_amount: Option<f64>,
    pub max_loan_amount: Option<f64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list_sheets(pool: &PgPool, company_id: Uuid) -> anyhow::Result<Vec<RateSheet>> {
    let rows = sqlx::query_as::<_, RateSheet>(
        r#"
        SELECT id, company_id, lender, effective_date, expires_date, is_active, created_at
        FROM rate_sheets
        WHERE company_id = $1
        ORDER BY is_active DESC, effective_date DESC NULLS LAST, created_at DESC
        "#,
    )
    .bind(company_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn list_options(
    pool: &PgPool,
    rate_sheet_id: Uuid,
) -> anyhow::Result<Vec<RateOption>> {
    let rows = sqlx::query_as::<_, RateOption>(
        r#"
        SELECT id, rate_sheet_id, company_id, product, rate, points, comp_plan,
               min_loan_amount, max_loan_amount, created_at
        FROM rate_sheet_options
        WHERE rate_sheet_id = $1
        ORDER BY product, rate DESC
        "#,
    )
    .bind(rate_sheet_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_sheet(
    pool: &PgPool,
    company_id: Uuid,
    lender: &str,
    effective_date: Option<NaiveDate>,
    expires_date: Option<NaiveDate>,
) -> anyhow::Result<RateSheet> {
    let row = sqlx::query_as::<_, RateSheet>(
        r#"
        INSERT INTO rate_sheets (company_id, lender, effective_date, expires_date)
        VALUES ($1, $2, $3, $4)
        RETURNING id, company_id, lender, effective_date, expires_date, is_active, created_at
        "#,
    )
    .bind(company_id)
    .bind(lender)
    .bind(effective_date)
    .bind(expires_date)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

/// Exactly one active sheet at a time — activating one deactivates the rest.
pub async fn set_active(
    pool: &PgPool,
    company_id: Uuid,
    rate_sheet_id: Uuid,
    active: bool,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await?;
    if active {
        sqlx::query("UPDATE rate_sheets SET is_active = false WHERE company_id = $1")
            .bind(company_id)
            .execute(&mut *tx)
            .await?;
    }
    sqlx::query("UPDATE rate_sheets SET is_active = $3 WHERE id = $2 AND company_id = $1")
        .bind(company_id)
        .bind(rate_sheet_id)
        .bind(active)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn add_option(
    pool: &PgPool,
    rate_sheet_id: Uuid,
    company_id: Uuid,
    product: &str,
    rate: f64,
    points: f64,
    comp_plan: Option<&str>,
    min_loan_amount: Option<f64>,
    max_loan_amount: Option<f64>,
) -> anyhow::Result<RateOption> {
    let row = sqlx::query_as::<_, RateOption>(
        r#"
        INSERT INTO rate_sheet_options (rate_sheet_id, company_id, product, rate,
                                        points, comp_plan, min_loan_amount, max_loan_amount)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, rate_sheet_id, company_id, product, rate, points, comp_plan,
                  min_loan_amount, max_loan_amount, created_at
        "#,
    )
    .bind(rate_sheet_id)
    .bind(company_id)
    .bind(product)
    .bind(rate)
    .bind(points)
    .bind(comp_plan)
    .bind(min_loan_amount)
    .bind(max_loan_amount)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn remove_option(pool: &PgPool, option_id: Uuid) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM rate_sheet_options WHERE id = $1")
        .bind(option_id)
        .execute(pool)
        .await?;
    Ok(())
}
