# Pilot-Readiness Checklist

Phase 5's gate is PILOT-READY. The software side is complete — this
checklist is what turns a fresh install into a working pilot shop.

## Install (per pilot shop)

- [ ] Postgres 14+ reachable from the pilot machine
- [ ] Orion installed; setup wizard run (DB URL → schema deployed →
      company created → assistant checked)
- [ ] Team roster created on the Team screen (⌃m) — one admin minimum,
      then loan officers / processors / underwriters; admin signs in
      first and verifies the audit shows `user.create` rows
- [ ] Demo-mode escape hatch understood: unconfigured installs open
      into demo data; connecting a DB flips the app live

## Data to seed before the first file

- [ ] At least one active lender rate sheet with rows (⌃y) — pricing
      and the pricing AI both read the *active* sheet only
- [ ] Company name verified on the nav tray (rename in Settings if
      the wizard placeholder stuck)

## Known limits to state upfront

- No passwords: sign-in is persona selection on a trusted machine.
  Acceptable for a single-shop desktop pilot; server builds need real
  auth.
- GSE submissions run against the **built-in mock engine** until
  `aus.du_endpoint` / `aus.lpa_endpoint` are configured — the AUS
  screen labels mock rows, but set expectations with the pilot LOs.
- MISMO export carries `<!-- ULAD: ... -->` comments where the exact
  3.4 element form awaits ULAD confirmation (MISMO membership, PT-04).
- SSN field permissions and the audit mask (last-4) are in place, but
  pilot machines should still be disk-encrypted.

## Signals the pilot is healthy

- `audit_log` shows one row per mutation; `mismo.export` /
  `aus.submit_*` events appear as files leave the shop
- Loan locks behave: two personas, one file — the second goes
  read-only with a badge naming who holds it
- Tracker tells the truth: a green overview row means the stage's
  required facts exist, not that someone checked a box

## Before GSE certification (Phase 6)

- [ ] PT-02/PT-03: Fannie + Freddie Technology Partner approvals
- [ ] PT-04: MISMO membership — confirm every `ULAD:` marker in
      `src-tauri/src/mismo.rs` against the mapping document
- [ ] Configure the DU/LPA sandbox endpoints in Settings and submit a
      complete demo-adjacent loan through both engines live
