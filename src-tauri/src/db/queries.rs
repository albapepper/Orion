// Runtime-checked sqlx queries. These can be migrated to the compile-time
// `query!` macros once a dev database + `sqlx prepare` workflow is in place.

use crate::loans::{
    Asset, Borrower, BorrowerDependent, Employment, Liability, Loan, LoanIncome, LoanSummary,
    LoanTerms, ReoProperty, UwReview,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_users(
    pool: &PgPool,
    company_id: Uuid,
) -> anyhow::Result<Vec<crate::loans::UserAccount>> {
    let rows = sqlx::query_as::<_, crate::loans::UserAccount>(
        "SELECT id, email, name, role FROM users WHERE company_id = $1 ORDER BY name",
    )
    .bind(company_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_user(
    pool: &PgPool,
    user_id: Uuid,
) -> anyhow::Result<Option<crate::loans::UserAccount>> {
    let row = sqlx::query_as::<_, crate::loans::UserAccount>(
        "SELECT id, email, name, role FROM users WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn get_user_role(pool: &PgPool, user_id: Uuid) -> anyhow::Result<Option<String>> {
    let role: Option<(String,)> = sqlx::query_as("SELECT role FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    Ok(role.map(|r| r.0))
}

const UW_REVIEW_COLUMNS: &str = "r.id, r.loan_id, r.company_id, r.screen, r.status, r.note,
    r.updated_by, r.updated_at, u.name AS updated_by_name";

pub async fn get_uw_reviews(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<UwReview>> {
    let rows = sqlx::query_as::<_, UwReview>(&format!(
        "SELECT {UW_REVIEW_COLUMNS} FROM loan_uw_reviews r
         LEFT JOIN users u ON u.id = r.updated_by
         WHERE r.loan_id = $1 ORDER BY r.screen"
    ))
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn upsert_uw_review(
    pool: &PgPool,
    loan_id: Uuid,
    company_id: Uuid,
    screen: &str,
    status: &str,
    note: Option<&str>,
    updated_by: Uuid,
) -> anyhow::Result<UwReview> {
    let row = sqlx::query_as::<_, UwReview>(
        r#"
        INSERT INTO loan_uw_reviews (loan_id, company_id, screen, status, note, updated_by, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, now())
        ON CONFLICT (loan_id, screen) DO UPDATE SET
            status = EXCLUDED.status,
            note = EXCLUDED.note,
            updated_by = EXCLUDED.updated_by,
            updated_at = now()
        RETURNING id, loan_id, company_id, screen, status, note, updated_by, updated_at,
            (SELECT name FROM users WHERE id = updated_by) AS updated_by_name
        "#,
    )
    .bind(loan_id)
    .bind(company_id)
    .bind(screen)
    .bind(status)
    .bind(note)
    .bind(updated_by)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

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

const EMPLOYMENT_COLUMNS: &str = "id, borrower_id, loan_id, company_id, employer_name,
    position, self_employed, monthly_income::FLOAT8 AS monthly_income,
    start_date, end_date, is_current, months_in_line_of_work, business_phone";

pub async fn get_employments(
    pool: &PgPool,
    loan_id: Uuid,
) -> anyhow::Result<Vec<Employment>> {
    let rows = sqlx::query_as::<_, Employment>(
        &format!(
            "SELECT {EMPLOYMENT_COLUMNS} FROM borrower_employments
             WHERE loan_id = $1
             ORDER BY is_current DESC, start_date DESC"
        ),
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_employment(
    pool: &PgPool,
    id: Option<Uuid>,
    borrower_id: Uuid,
    loan_id: Uuid,
    company_id: Uuid,
    employer_name: Option<&str>,
    position: Option<&str>,
    self_employed: bool,
    monthly_income: Option<f64>,
    start_date: Option<chrono::NaiveDate>,
    end_date: Option<chrono::NaiveDate>,
    is_current: bool,
    months_in_line_of_work: Option<i32>,
    business_phone: Option<&str>,
) -> anyhow::Result<Employment> {
    let row = sqlx::query_as::<_, Employment>(&format!(
        r#"
        INSERT INTO borrower_employments
            (id, borrower_id, loan_id, company_id, employer_name, position,
             self_employed, monthly_income, start_date, end_date, is_current,
             months_in_line_of_work, business_phone, updated_at)
        VALUES
            (COALESCE($1, gen_random_uuid()), $2, $3, $4, $5, $6, $7, $8, $9,
             $10, $11, $12, $13, now())
        ON CONFLICT (id) DO UPDATE SET
            employer_name = EXCLUDED.employer_name,
            position = EXCLUDED.position,
            self_employed = EXCLUDED.self_employed,
            monthly_income = EXCLUDED.monthly_income,
            start_date = EXCLUDED.start_date,
            end_date = EXCLUDED.end_date,
            is_current = EXCLUDED.is_current,
            months_in_line_of_work = EXCLUDED.months_in_line_of_work,
            business_phone = EXCLUDED.business_phone,
            updated_at = now()
        RETURNING {EMPLOYMENT_COLUMNS}
        "#
    ))
    .bind(id)
    .bind(borrower_id)
    .bind(loan_id)
    .bind(company_id)
    .bind(employer_name)
    .bind(position)
    .bind(self_employed)
    .bind(monthly_income)
    .bind(start_date)
    .bind(end_date)
    .bind(is_current)
    .bind(months_in_line_of_work)
    .bind(business_phone)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn get_employment(
    pool: &PgPool,
    id: Uuid,
) -> anyhow::Result<Option<Employment>> {
    let row = sqlx::query_as::<_, Employment>(
        &format!("SELECT {EMPLOYMENT_COLUMNS} FROM borrower_employments WHERE id = $1"),
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn delete_employment(pool: &PgPool, id: Uuid) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM borrower_employments WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
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
               prior_year_monthly::FLOAT8 AS prior_year_monthly,
               verified,
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
    prior_year_monthly: Option<f64>,
    verified: bool,
    employer_name: Option<&str>,
    years_on_job: Option<f64>,
    notes: Option<&str>,
) -> anyhow::Result<LoanIncome> {
    let row = sqlx::query_as::<_, LoanIncome>(
        r#"
        INSERT INTO loan_income
            (id, loan_id, company_id, borrower_id, income_type,
             monthly_amount, prior_year_monthly, verified,
             employer_name, years_on_job, notes, updated_at)
        VALUES
            (COALESCE($1, gen_random_uuid()), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, now())
        ON CONFLICT (id) DO UPDATE SET
            income_type = EXCLUDED.income_type,
            monthly_amount = EXCLUDED.monthly_amount,
            prior_year_monthly = EXCLUDED.prior_year_monthly,
            verified = EXCLUDED.verified,
            employer_name = EXCLUDED.employer_name,
            years_on_job = EXCLUDED.years_on_job,
            notes = EXCLUDED.notes,
            updated_at = now()
        RETURNING id, loan_id, company_id, borrower_id, income_type,
                  monthly_amount::FLOAT8 AS monthly_amount,
                  prior_year_monthly::FLOAT8 AS prior_year_monthly,
                  verified,
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
    .bind(prior_year_monthly)
    .bind(verified)
    .bind(employer_name)
    .bind(years_on_job)
    .bind(notes)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

const ASSET_COLUMNS: &str = "id, borrower_id, loan_id, company_id, account_type, institution,
    balance::FLOAT8 AS balance, gift_donor, gift_relationship, notes";

pub async fn get_assets(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<Asset>> {
    let rows = sqlx::query_as::<_, Asset>(&format!(
        "SELECT {ASSET_COLUMNS} FROM loan_assets WHERE loan_id = $1
         ORDER BY balance DESC"
    ))
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_asset(
    pool: &PgPool,
    id: Option<Uuid>,
    borrower_id: Uuid,
    loan_id: Uuid,
    company_id: Uuid,
    account_type: &str,
    institution: Option<&str>,
    balance: f64,
    gift_donor: Option<&str>,
    gift_relationship: Option<&str>,
    notes: Option<&str>,
) -> anyhow::Result<Asset> {
    let row = sqlx::query_as::<_, Asset>(&format!(
        r#"
        INSERT INTO loan_assets
            (id, borrower_id, loan_id, company_id, account_type, institution,
             balance, gift_donor, gift_relationship, notes, updated_at)
        VALUES (COALESCE($1, gen_random_uuid()), $2, $3, $4, $5, $6, $7, $8, $9, $10, now())
        ON CONFLICT (id) DO UPDATE SET
            account_type = EXCLUDED.account_type,
            institution = EXCLUDED.institution,
            balance = EXCLUDED.balance,
            gift_donor = EXCLUDED.gift_donor,
            gift_relationship = EXCLUDED.gift_relationship,
            notes = EXCLUDED.notes,
            updated_at = now()
        RETURNING {ASSET_COLUMNS}
        "#
    ))
    .bind(id)
    .bind(borrower_id)
    .bind(loan_id)
    .bind(company_id)
    .bind(account_type)
    .bind(institution)
    .bind(balance)
    .bind(gift_donor)
    .bind(gift_relationship)
    .bind(notes)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn delete_asset(pool: &PgPool, id: Uuid) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM loan_assets WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_asset(pool: &PgPool, id: Uuid) -> anyhow::Result<Option<Asset>> {
    let row = sqlx::query_as::<_, Asset>(&format!(
        "SELECT {ASSET_COLUMNS} FROM loan_assets WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

const LIABILITY_COLUMNS: &str = "id, borrower_id, loan_id, company_id, liability_type,
    creditor_name, monthly_payment::FLOAT8 AS monthly_payment,
    unpaid_balance::FLOAT8 AS unpaid_balance, payoff_at_close, notes";

pub async fn get_liabilities(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<Liability>> {
    let rows = sqlx::query_as::<_, Liability>(&format!(
        "SELECT {LIABILITY_COLUMNS} FROM loan_liabilities WHERE loan_id = $1
         ORDER BY monthly_payment DESC"
    ))
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_liability(pool: &PgPool, id: Uuid) -> anyhow::Result<Option<Liability>> {
    let row = sqlx::query_as::<_, Liability>(&format!(
        "SELECT {LIABILITY_COLUMNS} FROM loan_liabilities WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_liability(
    pool: &PgPool,
    id: Option<Uuid>,
    borrower_id: Uuid,
    loan_id: Uuid,
    company_id: Uuid,
    liability_type: &str,
    creditor_name: Option<&str>,
    monthly_payment: Option<f64>,
    unpaid_balance: Option<f64>,
    payoff_at_close: bool,
    notes: Option<&str>,
) -> anyhow::Result<Liability> {
    let row = sqlx::query_as::<_, Liability>(&format!(
        r#"
        INSERT INTO loan_liabilities
            (id, borrower_id, loan_id, company_id, liability_type, creditor_name,
             monthly_payment, unpaid_balance, payoff_at_close, notes, updated_at)
        VALUES (COALESCE($1, gen_random_uuid()), $2, $3, $4, $5, $6, $7, $8, $9, $10, now())
        ON CONFLICT (id) DO UPDATE SET
            liability_type = EXCLUDED.liability_type,
            creditor_name = EXCLUDED.creditor_name,
            monthly_payment = EXCLUDED.monthly_payment,
            unpaid_balance = EXCLUDED.unpaid_balance,
            payoff_at_close = EXCLUDED.payoff_at_close,
            notes = EXCLUDED.notes,
            updated_at = now()
        RETURNING {LIABILITY_COLUMNS}
        "#
    ))
    .bind(id)
    .bind(borrower_id)
    .bind(loan_id)
    .bind(company_id)
    .bind(liability_type)
    .bind(creditor_name)
    .bind(monthly_payment)
    .bind(unpaid_balance)
    .bind(payoff_at_close)
    .bind(notes)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn delete_liability(pool: &PgPool, id: Uuid) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM loan_liabilities WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

const PROPERTY_COLUMNS: &str = "id, loan_id, company_id, address_line, city, state, postal_code,
    property_type, occupancy, year_built, units, estimated_value::FLOAT8 AS estimated_value, notes";

pub async fn get_subject_property(
    pool: &PgPool,
    loan_id: Uuid,
) -> anyhow::Result<Option<crate::loans::SubjectProperty>> {
    let row = sqlx::query_as::<_, crate::loans::SubjectProperty>(&format!(
        "SELECT {PROPERTY_COLUMNS} FROM loan_subject_property WHERE loan_id = $1"
    ))
    .bind(loan_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_subject_property(
    pool: &PgPool,
    loan_id: Uuid,
    company_id: Uuid,
    address_line: Option<&str>,
    city: Option<&str>,
    state: Option<&str>,
    postal_code: Option<&str>,
    property_type: &str,
    occupancy: &str,
    year_built: Option<i32>,
    units: i32,
    estimated_value: Option<f64>,
    notes: Option<&str>,
) -> anyhow::Result<crate::loans::SubjectProperty> {
    let row = sqlx::query_as::<_, crate::loans::SubjectProperty>(&format!(
        r#"
        INSERT INTO loan_subject_property
            (id, loan_id, company_id, address_line, city, state, postal_code,
             property_type, occupancy, year_built, units, estimated_value, notes, updated_at)
        VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, now())
        ON CONFLICT (loan_id) DO UPDATE SET
            address_line = EXCLUDED.address_line,
            city = EXCLUDED.city,
            state = EXCLUDED.state,
            postal_code = EXCLUDED.postal_code,
            property_type = EXCLUDED.property_type,
            occupancy = EXCLUDED.occupancy,
            year_built = EXCLUDED.year_built,
            units = EXCLUDED.units,
            estimated_value = EXCLUDED.estimated_value,
            notes = EXCLUDED.notes,
            updated_at = now()
        RETURNING {PROPERTY_COLUMNS}
        "#
    ))
    .bind(loan_id)
    .bind(company_id)
    .bind(address_line)
    .bind(city)
    .bind(state)
    .bind(postal_code)
    .bind(property_type)
    .bind(occupancy)
    .bind(year_built)
    .bind(units)
    .bind(estimated_value)
    .bind(notes)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

const REO_COLUMNS: &str = "id, borrower_id, loan_id, company_id, address_line, city, state,
    postal_code, property_type, market_value::FLOAT8 AS market_value,
    mortgage_payment::FLOAT8 AS mortgage_payment, rental_income::FLOAT8 AS rental_income,
    disposition, notes";

pub async fn get_reo(pool: &PgPool, loan_id: Uuid) -> anyhow::Result<Vec<ReoProperty>> {
    let rows = sqlx::query_as::<_, ReoProperty>(&format!(
        "SELECT {REO_COLUMNS} FROM loan_properties_reo WHERE loan_id = $1
         ORDER BY market_value DESC"
    ))
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_reo_by_id(pool: &PgPool, id: Uuid) -> anyhow::Result<Option<ReoProperty>> {
    let row = sqlx::query_as::<_, ReoProperty>(&format!(
        "SELECT {REO_COLUMNS} FROM loan_properties_reo WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_reo(
    pool: &PgPool,
    id: Option<Uuid>,
    borrower_id: Uuid,
    loan_id: Uuid,
    company_id: Uuid,
    address_line: Option<&str>,
    city: Option<&str>,
    state: Option<&str>,
    postal_code: Option<&str>,
    property_type: &str,
    market_value: Option<f64>,
    mortgage_payment: Option<f64>,
    rental_income: Option<f64>,
    disposition: &str,
    notes: Option<&str>,
) -> anyhow::Result<ReoProperty> {
    let row = sqlx::query_as::<_, ReoProperty>(&format!(
        r#"
        INSERT INTO loan_properties_reo
            (id, borrower_id, loan_id, company_id, address_line, city, state, postal_code,
             property_type, market_value, mortgage_payment, rental_income, disposition,
             notes, updated_at)
        VALUES (COALESCE($1, gen_random_uuid()), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11,
                $12, $13, $14, now())
        ON CONFLICT (id) DO UPDATE SET
            address_line = EXCLUDED.address_line,
            city = EXCLUDED.city,
            state = EXCLUDED.state,
            postal_code = EXCLUDED.postal_code,
            property_type = EXCLUDED.property_type,
            market_value = EXCLUDED.market_value,
            mortgage_payment = EXCLUDED.mortgage_payment,
            rental_income = EXCLUDED.rental_income,
            disposition = EXCLUDED.disposition,
            notes = EXCLUDED.notes,
            updated_at = now()
        RETURNING {REO_COLUMNS}
        "#
    ))
    .bind(id)
    .bind(borrower_id)
    .bind(loan_id)
    .bind(company_id)
    .bind(address_line)
    .bind(city)
    .bind(state)
    .bind(postal_code)
    .bind(property_type)
    .bind(market_value)
    .bind(mortgage_payment)
    .bind(rental_income)
    .bind(disposition)
    .bind(notes)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn delete_reo(pool: &PgPool, id: Uuid) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM loan_properties_reo WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_declarations(
    pool: &PgPool,
    loan_id: Uuid,
) -> anyhow::Result<Vec<crate::loans::Declaration>> {
    let rows = sqlx::query_as::<_, crate::loans::Declaration>(
        "SELECT id, borrower_id, loan_id, company_id, question_key, answer, explanation
         FROM loan_declarations WHERE loan_id = $1
         ORDER BY borrower_id, question_key",
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn upsert_declaration(
    pool: &PgPool,
    borrower_id: Uuid,
    loan_id: Uuid,
    company_id: Uuid,
    question_key: &str,
    answer: Option<bool>,
    explanation: Option<&str>,
) -> anyhow::Result<crate::loans::Declaration> {
    let row = sqlx::query_as::<_, crate::loans::Declaration>(
        r#"
        INSERT INTO loan_declarations
            (borrower_id, loan_id, company_id, question_key, answer, explanation, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, now())
        ON CONFLICT (borrower_id, question_key) DO UPDATE SET
            answer = EXCLUDED.answer,
            explanation = EXCLUDED.explanation,
            updated_at = now()
        RETURNING id, borrower_id, loan_id, company_id, question_key, answer, explanation
        "#,
    )
    .bind(borrower_id)
    .bind(loan_id)
    .bind(company_id)
    .bind(question_key)
    .bind(answer)
    .bind(explanation)
    .fetch_one(pool)
    .await?;
    Ok(row)
}

pub async fn get_demographics(
    pool: &PgPool,
    loan_id: Uuid,
) -> anyhow::Result<Vec<crate::loans::Demographics>> {
    let rows = sqlx::query_as::<_, crate::loans::Demographics>(
        "SELECT id, borrower_id, loan_id, company_id, ethnicity, race, sex, collection_basis
         FROM borrower_demographics WHERE loan_id = $1",
    )
    .bind(loan_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

#[allow(clippy::too_many_arguments)]
pub async fn upsert_demographics(
    pool: &PgPool,
    borrower_id: Uuid,
    loan_id: Uuid,
    company_id: Uuid,
    ethnicity: Option<&str>,
    race: Option<&str>,
    sex: Option<&str>,
    collection_basis: Option<&str>,
) -> anyhow::Result<crate::loans::Demographics> {
    let row = sqlx::query_as::<_, crate::loans::Demographics>(
        r#"
        INSERT INTO borrower_demographics
            (borrower_id, loan_id, company_id, ethnicity, race, sex, collection_basis, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, now())
        ON CONFLICT (borrower_id) DO UPDATE SET
            ethnicity = EXCLUDED.ethnicity,
            race = EXCLUDED.race,
            sex = EXCLUDED.sex,
            collection_basis = EXCLUDED.collection_basis,
            updated_at = now()
        RETURNING id, borrower_id, loan_id, company_id, ethnicity, race, sex, collection_basis
        "#,
    )
    .bind(borrower_id)
    .bind(loan_id)
    .bind(company_id)
    .bind(ethnicity)
    .bind(race)
    .bind(sex)
    .bind(collection_basis)
    .fetch_one(pool)
    .await?;
    Ok(row)
}
