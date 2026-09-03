// The loan cascade, in working order:
// borrower → employment → property → terms → credit → income → assets
// → liabilities → reo → declarations → demographic → documents → conditions
export type LoanScreen =
  | 'borrower'
  | 'employment'
  | 'property'
  | 'terms'
  | 'credit'
  | 'income'
  | 'assets'
  | 'liabilities'
  | 'reo'
  | 'declarations'
  | 'demographic'
  | 'documents'
  | 'conditions';

export const LOAN_SCREENS: LoanScreen[] = [
  'borrower',
  'employment',
  'property',
  'terms',
  'credit',
  'income',
  'assets',
  'liabilities',
  'reo',
  'declarations',
  'demographic',
  'documents',
  'conditions',
];

export const SCREEN_LABELS: Record<LoanScreen, string> = {
  borrower: 'Borrower Info',
  employment: 'Employment',
  property: 'Property',
  terms: 'Loan Terms',
  credit: 'Credit',
  income: 'Income Analysis',
  assets: 'Assets',
  liabilities: 'Liabilities',
  reo: 'REO',
  declarations: 'Declarations',
  demographic: 'Demographic',
  documents: 'Documents',
  conditions: 'Conditions',
};

// Keyboard shortcut letter for each stage (Ctrl-modified)
export const STAGE_KEYS: Record<LoanScreen, string> = {
  borrower: 'b',
  employment: 'e',
  property: 'p',
  terms: 't',
  credit: 'c',
  income: 'i',
  assets: 'a',
  liabilities: 'l',
  reo: 'r',
  declarations: 'q',
  demographic: 'g',
  documents: 'f', // files
  conditions: 'h',
};

export const nextStage = (screen: LoanScreen): LoanScreen | null =>
  LOAN_SCREENS[LOAN_SCREENS.indexOf(screen) + 1] ?? null;

export const prevStage = (screen: LoanScreen): LoanScreen | null =>
  LOAN_SCREENS[LOAN_SCREENS.indexOf(screen) - 1] ?? null;
