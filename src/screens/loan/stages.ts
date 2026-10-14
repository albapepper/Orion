// The loan cascade, in working order:
// borrower → employment → property → terms → income → assets
// → liabilities → reo → declarations → demographic → documents → conditions
// → pricing → AUS
export type LoanScreen =
  | 'borrower'
  | 'employment'
  | 'property'
  | 'terms'
  | 'income'
  | 'assets'
  | 'liabilities'
  | 'reo'
  | 'declarations'
  | 'demographic'
  | 'documents'
  | 'conditions'
  | 'pricing'
  | 'aus'
  | 'closing'
  | 'funding';

export const LOAN_SCREENS: LoanScreen[] = [
  'borrower',
  'employment',
  'property',
  'terms',
  'income',
  'assets',
  'liabilities',
  'reo',
  'declarations',
  'demographic',
  'documents',
  'conditions',
  'pricing',
  'aus',
  'closing',
  'funding',
];

export const SCREEN_LABELS: Record<LoanScreen, string> = {
  borrower: 'Borrower Info',
  employment: 'Employment',
  property: 'Property',
  terms: 'Loan Terms',
  income: 'Income Analysis',
  assets: 'Assets',
  liabilities: 'Liabilities',
  reo: 'REO',
  declarations: 'Declarations',
  demographic: 'Demographic',
  documents: 'Documents',
  conditions: 'Conditions',
  pricing: 'Pricing',
  aus: 'AUS',
  closing: 'Closing',
  funding: 'Funding',
};

// Keyboard shortcut letter for each stage (Ctrl-modified)
export const STAGE_KEYS: Record<LoanScreen, string> = {
  borrower: 'b',
  employment: 'e',
  property: 'p',
  terms: 't',
  income: 'i',
  assets: 'a',
  liabilities: 'l',
  reo: 'r',
  declarations: 'q',
  demographic: 'g',
  documents: 'f', // files
  conditions: 'h',
  pricing: 'v',
  aus: 'u',
  closing: 'w',
  funding: 'z',
};

export const nextStage = (screen: LoanScreen): LoanScreen | null =>
  LOAN_SCREENS[LOAN_SCREENS.indexOf(screen) + 1] ?? null;

export const prevStage = (screen: LoanScreen): LoanScreen | null =>
  LOAN_SCREENS[LOAN_SCREENS.indexOf(screen) - 1] ?? null;
