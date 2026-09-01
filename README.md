# Orion

**AI-native, on-premise mortgage origination platform.** Ships as a native
desktop app (Tauri v2) that connects to the buyer's own SQL server and runs
AI locally via Ollama — or against any OpenAI-compatible cloud endpoint. No
SaaS, no vendor-hosted data.

## Core ideas

- **Screen-aware AI sidebar** — the current screen's loan data is injected as
  system context, so answers are always narrow and relevant
- **Unified team task list** — everyone touching a loan sees the same
  outstanding items
- **Transparent loan locking** — single editor, everyone else read-only with
  full visibility of who holds the lock and where
- **Immutable audit log** — every data change is recorded
- **Local AI by default** — Ollama on-device, zero per-query cost, no data
  leaves the building; cloud AI is a config switch, not a rewrite

## Stack

Tauri v2 · SolidJS + TypeScript · Rust · sqlx (Postgres) · Tokio · Ollama

## Development

```bash
# prerequisites: Node, Rust, and (for local AI) Ollama
brew install ollama
ollama serve &
ollama pull granite4.2:3b

npm install
npm run tauri dev
```

With no database configured the app opens in **demo mode** — seeded loans,
editable income screen (read-only persistence), and a fully working AI
sidebar against local Ollama.

To run against a real Postgres instance, the setup wizard (Phase 4, not yet
built) will write `~/Library/Application Support/orion/config.toml`; until
then create it by hand:

```toml
[database]
url = "postgres://user:pass@localhost/oriondb"

[ai]
provider = "local"                       # "local" or "cloud"
endpoint = "http://localhost:11434/v1"
model = "granite4.2:3b"
api_key = ""
thinking = true

[app]
company_id = "<uuid of row in companies>"
```

Migrations live in `src-tauri/migrations/` and deploy via the
`deploy_schema` IPC command (or `sqlx migrate run`).

## Architecture

The Rust core (`src-tauri/src/`) is the only layer that touches the database
and AI. The SolidJS frontend calls Tauri IPC commands exclusively.

```
src-tauri/src/
├── config/     config.toml load/save
├── ai/         AiProvider trait + local (Ollama) and cloud impls
├── db/         pool setup, queries, migrations
├── loans/      loan/borrower/income types, per-screen AI context builders
├── tasks/      unified task list
├── locks/      single-editor loan locking
├── audit/      immutable audit log writes
└── commands/   all Tauri IPC commands
src/
├── components/ AiSidebar, TaskPanel, LockBadge
└── screens/    Dashboard, LoanList, loan/IncomeScreen
```

## Roadmap

- [x] Phase 1 — foundation: config, AI abstraction, DB + migrations, Tauri shell
- [x] Phase 2 (partial) — loan list, income screen, AI sidebar w/ context injection
- [ ] Phase 3 — locking UX, tasks polish, per-screen presence
- [ ] Phase 4 — setup wizard + first-run detection
