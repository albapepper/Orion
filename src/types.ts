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
}

export interface LoanIncome {
  id: string;
  loan_id: string;
  company_id: string;
  borrower_id: string;
  income_type: string;
  monthly_amount: number;
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

export interface LoanDetail {
  summary: LoanSummary;
  borrowers: Borrower[];
  income: LoanIncome[];
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
  'other',
] as const;
