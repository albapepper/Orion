export interface LoanSummary {
  id: string;
  loan_number: string;
  status: string;
  borrower_name: string | null;
  assigned_to_name: string | null;
  updated_at: string;
  flagged_reviews: number;
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

export interface SubjectProperty {
  id: string;
  loan_id: string;
  company_id: string;
  address_line: string | null;
  city: string | null;
  state: string | null;
  postal_code: string | null;
  property_type: string;
  occupancy: string;
  year_built: number | null;
  units: number;
  estimated_value: number | null;
  notes: string | null;
}

export interface ReoProperty {
  id: string;
  borrower_id: string;
  loan_id: string;
  company_id: string;
  address_line: string | null;
  city: string | null;
  state: string | null;
  postal_code: string | null;
  property_type: string;
  market_value: number | null;
  mortgage_payment: number | null;
  rental_income: number | null;
  disposition: string;
  notes: string | null;
}

export interface Declaration {
  id: string;
  borrower_id: string;
  loan_id: string;
  company_id: string;
  question_key: string;
  answer: boolean | null;
  explanation: string | null;
}

export interface Demographics {
  id: string;
  borrower_id: string;
  loan_id: string;
  company_id: string;
  ethnicity: string | null;
  race: string | null;
  sex: string | null;
  collection_basis: string | null;
}

export interface LoanDetail {
  summary: LoanSummary;
  borrowers: Borrower[];
  dependents: BorrowerDependent[];
  employments: Employment[];
  income: LoanIncome[];
  assets: Asset[];
  liabilities: Liability[];
  property: SubjectProperty | null;
  reo: ReoProperty[];
  declarations: Declaration[];
  demographics: Demographics[];
  terms: LoanTerms | null;
  uw_reviews: UwReview[];
  documents: Document[];
  conditions: Condition[];
  pricing: LoanPricingOption[];
  lock: LockInfo | null;
}

// A pricing option built for one loan from a rate-sheet row. P&I and
// points cost are computed server-side against the loan's terms.
export interface LoanPricingOption {
  id: string;
  loan_id: string;
  company_id: string;
  product: string;
  rate: number;
  points: number;
  monthly_pi: number | null;
  points_cost: number | null;
  comp_plan: string | null;
  selected: boolean;
  created_by: string | null;
  created_at: string;
}

export type DocumentType =
  | 'application'
  | 'income_verification'
  | 'bank_statement'
  | 'tax_return'
  | 'appraisal'
  | 'title'
  | 'insurance'
  | 'id_document'
  | 'other';

// A record of one document on the loan file. Bytes live outside the
// database for now — file_name carries the reference.
export interface Document {
  id: string;
  loan_id: string;
  company_id: string;
  title: string;
  doc_type: DocumentType;
  screen: string | null;
  file_name: string | null;
  mime_type: string | null;
  size_bytes: number | null;
  uploaded_by: string | null;
  uploaded_at: string;
  removed_at: string | null;
  uploaded_by_name: string | null;
}

export type ConditionStatus = 'open' | 'submitted' | 'cleared' | 'waived';

// Underwriting's checklist: outstanding conditions block approval.
export interface Condition {
  id: string;
  loan_id: string;
  company_id: string;
  title: string;
  detail: string | null;
  source: 'underwriter' | 'processor' | 'system';
  status: ConditionStatus;
  priority: 'low' | 'normal' | 'high';
  clear_by: string | null;
  cleared_at: string | null;
  cleared_by: string | null;
  created_at: string;
  updated_at: string;
  cleared_by_name: string | null;
}

export interface Task {
  id: string;
  loan_id: string;
  loan_number: string;
  title: string;
  description: string | null;
  assigned_to: string | null;
  assigned_to_name: string | null;
  screen: string | null;
  status: 'open' | 'in_progress' | 'done';
  priority: 'low' | 'normal' | 'high';
  due_date: string | null;
  created_at: string;
  completed_at: string | null;
}

export type UserRole = 'loan_officer' | 'processor' | 'underwriter' | 'admin';

export type LoanProduct =
  | '30yr_fixed'
  | '20yr_fixed'
  | '15yr_fixed'
  | '7_1_arm'
  | '5_1_arm';

// A lender's rate sheet: the container (who, when, active).
export interface RateSheet {
  id: string;
  company_id: string;
  lender: string;
  effective_date: string | null;
  expires_date: string | null;
  is_active: boolean;
  created_at: string;
}

// One priceable row: a rate for a product with its points.
// Positive points = borrower pays; negative = lender credit.
export interface RateOption {
  id: string;
  rate_sheet_id: string;
  company_id: string;
  product: LoanProduct;
  rate: number;
  points: number;
  comp_plan: string | null;
  min_loan_amount: number | null;
  max_loan_amount: number | null;
  created_at: string;
}

// The signed-in persona. No passwords — the desktop app binds identity to
// a real user row so locks, audit, and field permissions are per-user.
export interface UserAccount {
  id: string;
  email: string;
  name: string;
  role: UserRole;
}

export type UwReviewStatus = 'pending' | 'verified' | 'flagged';

// Underwriter review of one loan screen: verification status + notes.
// Everyone touching the loan can read it; only underwriters write it.
export interface UwReview {
  id: string;
  loan_id: string;
  company_id: string;
  screen: string;
  status: UwReviewStatus;
  note: string | null;
  updated_by: string | null;
  updated_at: string;
  updated_by_name: string | null;
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
