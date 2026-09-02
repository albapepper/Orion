export interface LoanSummary {
  id: string;
  loan_number: string;
  status: string;
  borrower_name: string | null;
  assigned_to_name: string | null;
  updated_at: string;
}

export interface Borrower {
  id: string;
  loan_id: string;
  company_id: string;
  first_name: string;
  last_name: string;
  email: string | null;
  phone: string | null;
  is_primary: boolean;
  middle_name: string | null;
  suffix_name: string | null;
  ssn: string | null;
  birth_date: string | null; // ISO date
  marital_status: string | null;
  citizenship_status: string | null;
}

export interface BorrowerDependent {
  id: string;
  borrower_id: string;
  loan_id: string;
  company_id: string;
  age: number | null;
}

export interface LoanIncome {
  id: string;
  loan_id: string;
  company_id: string;
  borrower_id: string;
  income_type: string;
  monthly_amount: number;
  prior_year_monthly: number | null;
  verified: boolean;
  employer_name: string | null;
  years_on_job: number | null;
  notes: string | null;
}

export interface LockInfo {
  loan_id: string;
  locked_by: string;
  user_name: string;
  screen: string | null;
}

export interface LoanTerms {
  id: string;
  loan_id: string;
  company_id: string;
  loan_purpose: string;
  mortgage_type: string;
  amortization_type: string;
  loan_amount: number | null;
  interest_rate: number | null;
  term_months: number | null;
  lien_priority: string;
  property_value: number | null;
  application_date: string | null; // ISO date, YYYY-MM-DD
}

export interface Employment {
  id: string;
  borrower_id: string;
  loan_id: string;
  company_id: string;
  employer_name: string | null;
  position: string | null;
  self_employed: boolean;
  monthly_income: number | null;
  start_date: string | null; // ISO date
  end_date: string | null; // ISO date, null = present
  is_current: boolean;
  months_in_line_of_work: number | null;
  business_phone: string | null;
}

export interface Asset {
  id: string;
  borrower_id: string;
  loan_id: string;
  company_id: string;
  account_type: string;
  institution: string | null;
  balance: number;
  gift_donor: string | null;
  gift_relationship: string | null;
  notes: string | null;
}

export interface Liability {
  id: string;
  borrower_id: string;
  loan_id: string;
  company_id: string;
  liability_type: string;
  creditor_name: string | null;
  monthly_payment: number | null;
  unpaid_balance: number | null;
  payoff_at_close: boolean;
  notes: string | null;
}

export interface LoanDetail {
  summary: LoanSummary;
  borrowers: Borrower[];
  dependents: BorrowerDependent[];
  employments: Employment[];
  income: LoanIncome[];
  assets: Asset[];
  liabilities: Liability[];
  terms: LoanTerms | null;
  lock: LockInfo | null;
}

export interface Task {
  id: string;
  loan_id: string;
  title: string;
  description: string | null;
  screen: string | null;
  status: 'open' | 'in_progress' | 'done';
  created_at: string;
}

export interface AiResponse {
  content: string;
  thinking: string | null;
}

export interface AppStatus {
  configured: boolean;
  db_connected: boolean;
  ai_provider: string;
  ai_model: string;
}

export const LOAN_STATUSES = [
  'application',
  'processing',
  'underwriting',
  'approval',
  'closing',
  'funded',
] as const;

export const INCOME_TYPES = [
  'base',
  'overtime',
  'bonus',
  'commission',
  'self_employed',
  'retirement',
  'social_security',
  'rental',
  'military_benefits',
  'other',
] as const;
