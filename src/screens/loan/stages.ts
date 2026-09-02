// The loan cascade, in working order:
// borrower → property → terms → credit → income → assets
export type LoanScreen =
  | 'borrower'
  | 'property'
  | 'terms'
  | 'credit'
  | 'income'
  | 'assets';

export const LOAN_SCREENS: LoanScreen[] = [
  'borrower',
  'property',
  'terms',
  'credit',
  'income',
  'assets',
];

export const SCREEN_LABELS: Record<LoanScreen, string> = {
  borrower: 'Borrower Info',
  property: 'Property',
  terms: 'Loan Terms',
  credit: 'Credit',
  income: 'Income Analysis',
  assets: 'Assets',
};

// Keyboard shortcut letter for each stage (Ctrl-modified)
export const STAGE_KEYS: Record<LoanScreen, string> = {
  borrower: 'b',
  property: 'p',
  terms: 't',
  credit: 'c',
  income: 'i',
  assets: 'a',
};

export const nextStage = (screen: LoanScreen): LoanScreen | null =>
  LOAN_SCREENS[LOAN_SCREENS.indexOf(screen) + 1] ?? null;

export const prevStage = (screen: LoanScreen): LoanScreen | null =>
  LOAN_SCREENS[LOAN_SCREENS.indexOf(screen) - 1] ?? null;
