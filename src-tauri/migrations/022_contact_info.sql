-- P3-17: Contact info on the 1003. Two gaps:
--   1. Borrower mailing address (URLA p.1) — the borrower party has
--      email/phone but no postal address. MISMO: PARTY/ADDRESSES.
--   2. Loan originator contact (URLA p.1 originator block) — users
--      carry name + email only; phone and title complete the block.
--      MISMO: PARTY/ROLES/ROLE RoleType=LoanOriginator.

ALTER TABLE borrowers
    ADD COLUMN mailing_address_line TEXT,
    ADD COLUMN mailing_city TEXT,
    ADD COLUMN mailing_state TEXT,
    ADD COLUMN mailing_postal_code TEXT;

-- MISMO: PARTY/ADDRESSES/ADDRESS (AddressType=Mailing)
COMMENT ON COLUMN borrowers.mailing_address_line IS
    'MISMO: PARTY/ADDRESSES/ADDRESS AddressLineText (AddressType=Mailing)';

ALTER TABLE users
    ADD COLUMN phone TEXT,
    ADD COLUMN title TEXT;
