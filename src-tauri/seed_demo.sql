-- Demo seed for Orion real mode.
-- Run after migrations: psql $DATABASE_URL -f seed_demo.sql

-- Company
INSERT INTO companies (id, name)
VALUES ('11111111-1111-1111-1111-111111111111', 'Demo Mortgage LLC')
ON CONFLICT (id) DO NOTHING;

-- Users
INSERT INTO users (id, company_id, email, name, role) VALUES
  ('22222222-0000-0000-0000-000000000001','11111111-1111-1111-1111-111111111111','admin@demo.test','Avery Admin','admin'),
  ('22222222-0000-0000-0000-000000000002','11111111-1111-1111-1111-111111111111','lo@demo.test','Loan Officer','loan_officer'),
  ('22222222-0000-0000-0000-000000000003','11111111-1111-1111-1111-111111111111','uw@demo.test','Underwriter Uma','underwriter')
ON CONFLICT (id) DO NOTHING;

-- Loan
INSERT INTO loans (id, company_id, loan_number, status, assigned_to) VALUES
  ('33333333-3333-3333-3333-333333333333','11111111-1111-1111-1111-111111111111','ORN-2026-0001','processing','22222222-0000-0000-0000-000000000002')
ON CONFLICT (id) DO NOTHING;

-- Borrower
INSERT INTO borrowers (id, loan_id, company_id, first_name, last_name, email, phone, is_primary, middle_name, marital_status, citizenship_status, birth_date)
VALUES ('44444444-4444-4444-4444-444444444444','33333333-3333-3333-3333-333333333333','11111111-1111-1111-1111-111111111111','Jordan','Borrower','jordan@demo.test','555-0001',true,'Rae','married','us_citizen','1990-05-10')
ON CONFLICT (id) DO NOTHING;

-- Terms
INSERT INTO loan_terms (loan_id, company_id, loan_purpose, mortgage_type, amortization_type, loan_amount, interest_rate, term_months, property_value)
VALUES ('33333333-3333-3333-3333-333333333333','11111111-1111-1111-1111-111111111111','purchase','conventional','fixed',420000,6.75,360,525000)
ON CONFLICT (loan_id) DO NOTHING;

-- Income
INSERT INTO loan_income (id, loan_id, company_id, borrower_id, income_type, monthly_amount, employer_name, years_on_job, notes)
VALUES ('66666666-6666-6666-6666-666666666666','33333333-3333-3333-3333-333333333333','11111111-1111-1111-1111-111111111111','44444444-4444-4444-4444-444444444444','base',7800,'Acme Inc',5.2,'steady W-2')
ON CONFLICT (id) DO NOTHING;

-- Assets
INSERT INTO loan_assets (id, loan_id, company_id, borrower_id, account_type, institution, balance)
VALUES ('55555555-5555-5555-5555-555555555555','33333333-3333-3333-3333-333333333333','11111111-1111-1111-1111-111111111111','44444444-4444-4444-4444-444444444444','checking','Demo Bank',25000)
ON CONFLICT (id) DO NOTHING;

-- Property
INSERT INTO loan_subject_property (loan_id, company_id, property_type, occupancy, units, estimated_value, address_line, city, state, postal_code)
VALUES ('33333333-3333-3333-3333-333333333333','11111111-1111-1111-1111-111111111111','detached','primary',1,525000,'123 Demo St','Springfield','IL','62701')
ON CONFLICT (loan_id) DO NOTHING;

-- Liabilities
INSERT INTO loan_liabilities (id, loan_id, company_id, borrower_id, liability_type, creditor_name, monthly_payment, unpaid_balance)
VALUES ('77777777-7777-7777-7777-777777777777','33333333-3333-3333-3333-333333333333','11111111-1111-1111-1111-111111111111','44444444-4444-4444-4444-444444444444','installment','Auto Creditor',420,12000)
ON CONFLICT (id) DO NOTHING;

-- Handbooks
INSERT INTO handbooks (source, product, screen, title, content) VALUES
  ('fannie','30yr_fixed','income','Income basics','Fannie: stable income, 2-yr history, verify; variable income averaged 2 years.'),
  ('fannie','30yr_fixed','assets','Reserves','Fannie: ~2 months reserves for most; gift funds require donor and letter.'),
  ('fannie','30yr_fixed','property','Property','Fannie: occupancy and LTV; condos need review; manufactured homes conditions.'),
  ('fha','30yr_fixed','income','FHA income','FHA: consistent income, overtime/bonus averaged if 2 yrs.'),
  ('freddie','30yr_fixed','assets','Freddie assets','Freddie: acceptable assets and gifts per Guide.');