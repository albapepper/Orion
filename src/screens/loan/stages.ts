// The loan cascade, in working order: property → credit → income → assets
export type LoanScreen = 'property' | 'credit' | 'income' | 'assets';

export const LOAN_SCREENS: LoanScreen[] = ['property', 'credit', 'income', 'assets'];

export const SCREEN_LABELS: Record<LoanScreen, string> = {
  property: 'Property',
  credit: 'Credit',
  income: 'Income Analysis',
  assets: 'Assets',
};

// Keyboard shortcut letter for each stage (Ctrl-modified)
export const STAGE_KEYS: Record<LoanScreen, string> = {
  property: 'p',
  credit: 'c',
  income: 'i',
  assets: 'a',
};

export const nextStage = (screen: LoanScreen): LoanScreen | null =>
  LOAN_SCREENS[LOAN_SCREENS.indexOf(screen) + 1] ?? null;

export const prevStage = (screen: LoanScreen): LoanScreen | null =>
  LOAN_SCREENS[LOAN_SCREENS.indexOf(screen) - 1] ?? null;
