// The loan cascade, in working order: property → terms → credit → income → assets
export type LoanScreen = 'property' | 'terms' | 'credit' | 'income' | 'assets';

export const LOAN_SCREENS: LoanScreen[] = ['property', 'terms', 'credit', 'income', 'assets'];

export const SCREEN_LABELS: Record<LoanScreen, string> = {
  property: 'Property',
  terms: 'Loan Terms',
  credit: 'Credit',
  income: 'Income Analysis',
  assets: 'Assets',
};

// Keyboard shortcut letter for each stage (Ctrl-modified)
export const STAGE_KEYS: Record<LoanScreen, string> = {
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
