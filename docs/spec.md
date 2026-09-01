# Orion — AI-Native Mortgage Origination Platform
## Claude Code Build Prompt

---

## What You Are Building

Orion is an **on-premise, AI-native mortgage origination platform** built for
sales and processing/underwriting teams to work hand-in-hand. It is not SaaS.
It ships as a native desktop application that installs on the buyer's machines
and connects to the buyer's own SQL database. The vendor (us) never touches
their data.

The core product differentiators are:
- **Local AI** running on-device with no per-query cost and no data leaving
  the building — or optionally cloud AI via any OpenAI-compatible endpoint
- **Screen-aware AI sidebar** that injects the current screen's loan data as
  context so answers are always relevant and narrow
- **Unified team task list** — every user touching a loan sees the same
  outstanding items
- **Transparent loan locking** — single-editor model (one user edits, others
  read-only with full visibility of who has the lock and on which screen)
- **Mouse-driven native UI** — modern, clean, nothing like the legacy
  keyboard-driven Unix systems this replaces
- **Presence per screen** — team members and contacts relevant to the current
  screen surface automatically

The working name is **Orion**. Use this throughout the codebase.

---

## Tech Stack — Use Exactly These

| Layer | Technology |
|---|---|
| Desktop shell | Tauri v2 |
| Frontend | SolidJS + TypeScript |
| Backend/core | Rust |
| Database ORM | sqlx (async, compile-time checked) |
| Async runtime | Tokio |
| Serialization | Serde + serde_json |
| HTTP client | reqwest |
| Config | toml |
| Migrations | sqlx migrate |
| Local AI | Ollama (OpenAI-compatible endpoint) |
| Error handling | anyhow |

Dev machine: **Apple Silicon Mac (M4)**. Metal acceleration is available.
Target buyers run Windows and Mac — build cross-platform from day one.

---

## What We Are NOT Building (Hard Constraints)

- **No cloud hosting** — this is on-premise software. No servers we run.
- **No data storage by us** — the buyer's SQL server holds all data. We deploy
  our schema to their server. We never see the data.
- **No proprietary AI dependency** — the AI layer must be fully abstracted
  behind a trait so any OpenAI-compatible endpoint works (Ollama locally, or
  any cloud provider the buyer already has).
- **No complex LOS integrations yet** — we are building our own schema from
  scratch, not reading Encompass or any other LOS.
- **No compliance automation yet** — TRID disclosures, HMDA reporting are
  future phases. Do not build these now.
- **No web app** — desktop only for now. Mobile is a future Swift phase.

---

## Architecture Overview

```
┌─────────────────────────────────────────────┐
│              Tauri Desktop App              │
│  ┌─────────────────┐  ┌───────────────────┐ │
│  │   SolidJS UI    │  │    AI Sidebar     │ │
│  │  (loan screens) │  │ (context-injected)│ │
│  └────────┬────────┘  └────────┬──────────┘ │
│           │ Tauri IPC          │ Tauri IPC  │
│  ┌────────▼────────────────────▼──────────┐ │
│  │              Rust Core                  │ │
│  │  ┌──────────┐ ┌──────────┐ ┌────────┐  │ │
│  │  │ Loan CRUD│ │AI Layer  │ │ Tasks  │  │ │
│  │  │ (sqlx)   │ │(trait)   │ │ Locks  │  │ │
│  │  └────┬─────┘ └────┬─────┘ └────────┘  │ │
│  └───────┼────────────┼───────────────────┘ │
└──────────┼────────────┼─────────────────────┘
           │            │
    Buyer's SQL    Ollama local
      Server       or cloud AI
```

The Rust core is the only layer that talks to the database and AI. The
SolidJS frontend never touches the database directly — it calls Tauri IPC
commands that are handled by Rust.

---

## Dev Environment Setup

Run these in order on a fresh Mac:

```bash
# Prerequisites
xcode-select --install
brew install node rustup
rustup-init  # accept defaults, restart terminal

# Tauri CLI
cargo install tauri-cli --version "^2.0"

# Local AI
brew install ollama
ollama serve &  # start Ollama in background
ollama pull granite4.2:3b  # primary default model

# Scaffold the project
npm create tauri-app@latest orion -- --template solid-ts
cd orion
npm install

# Add Rust dependencies to src-tauri/Cargo.toml
# (see Cargo.toml section below)

# Verify it works
npm run tauri dev
# A native window should open
```

---

## Project Structure

```
orion/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs              # Tauri app entry, registers commands
│   │   ├── lib.rs               # Module declarations
│   │   ├── config/
│   │   │   └── mod.rs           # Config loading, Config struct
│   │   ├── ai/
│   │   │   ├── mod.rs           # AiProvider trait, Message types
│   │   │   ├── local.rs         # Ollama/local implementation
│   │   │   └── cloud.rs         # Cloud (OpenAI-compatible) implementation
│   │   ├── db/
│   │   │   ├── mod.rs           # DB pool setup, connection helpers
│   │   │   └── queries.rs       # sqlx query functions
│   │   ├── loans/
│   │   │   ├── mod.rs           # Loan types, CRUD
│   │   │   └── screens/
│   │   │       └── income.rs    # Income screen context builder
│   │   ├── tasks/
│   │   │   └── mod.rs           # Task CRUD, unified task list
│   │   ├── locks/
│   │   │   └── mod.rs           # Loan locking system
│   │   ├── audit/
│   │   │   └── mod.rs           # Audit log writes
│   │   └── commands/
│   │       ├── mod.rs           # All Tauri IPC commands
│   │       ├── loan_commands.rs
│   │       ├── ai_commands.rs
│   │       └── task_commands.rs
│   ├── migrations/
│   │   ├── 001_init.sql
│   │   ├── 002_loans.sql
│   │   └── 003_tasks_locks.sql
│   └── Cargo.toml
├── src/
│   ├── App.tsx
│   ├── components/
│   │   ├── AiSidebar.tsx        # Persistent AI chat sidebar
│   │   ├── TaskPanel.tsx        # Unified task list
│   │   └── LockBadge.tsx        # Who has the edit lock
│   ├── screens/
│   │   ├── Dashboard.tsx        # Pipeline overview
│   │   ├── LoanList.tsx
│   │   └── loan/
│   │       └── IncomeScreen.tsx # First loan screen
│   └── index.tsx
└── package.json
```

---

## Cargo.toml Dependencies

```toml
[dependencies]
tauri = { version = "2", features = ["shell-open"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
sqlx = { version = "0.7", features = [
  "runtime-tokio",
  "postgres",    # primary target
  "sqlite",      # for dev/testing
  "chrono",
  "uuid",
  "migrate",
] }
reqwest = { version = "0.11", features = ["json"] }
anyhow = "1"
toml = "0.8"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
async-trait = "0.1"
```

---

## Config System

The config file lives at the platform-appropriate app data directory.
On Mac: `~/Library/Application Support/orion/config.toml`
On Windows: `%APPDATA%\orion\config.toml`

```toml
# config.toml — generated by the setup wizard

[database]
url = "postgres://user:pass@localhost/oriondb"

[ai]
provider = "local"          # "local" or "cloud"
endpoint = "http://localhost:11434/v1"
model = "granite4.2:3b"
api_key = ""                # empty for local
thinking = true             # enable thinking mode if supported

[app]
company_id = "uuid-here"    # set during setup wizard
```

```rust
// src-tauri/src/config/mod.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub ai: AiConfig,
    pub app: AppConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiConfig {
    pub provider: String,    // "local" or "cloud"
    pub endpoint: String,
    pub model: String,
    pub api_key: String,
    pub thinking: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub company_id: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let path = config_path();
        let contents = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| String::new());
        Ok(toml::from_str(&contents)?)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = config_path();
        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn is_configured(&self) -> bool {
        !self.database.url.is_empty() && !self.app.company_id.is_empty()
    }
}

fn config_path() -> PathBuf {
    dirs::data_dir()
        .expect("No data dir")
        .join("orion")
        .join("config.toml")
}
```

---

## AI Abstraction Layer

This is the first real code to write. Everything else depends on it.

```rust
// src-tauri/src/ai/mod.rs

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,    // "system" | "user" | "assistant"
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiResponse {
    pub content: String,
    pub thinking: Option<String>,  // chain-of-thought if enabled
}

#[async_trait]
pub trait AiProvider: Send + Sync {
    async fn complete(
        &self,
        system: &str,            // screen context injected here
        messages: &[Message],    // conversation history
        thinking: bool,          // enable reasoning mode
    ) -> anyhow::Result<AiResponse>;
}

// Factory: build the right provider from config
pub fn build_provider(config: &crate::config::AiConfig) 
    -> Box<dyn AiProvider> 
{
    match config.provider.as_str() {
        "local" => Box::new(local::LocalProvider::new(config)),
        "cloud" => Box::new(cloud::CloudProvider::new(config)),
        _ => Box::new(local::LocalProvider::new(config)),
    }
}
```

```rust
// src-tauri/src/ai/local.rs
// Ollama via OpenAI-compatible endpoint

use super::{AiProvider, AiResponse, Message};
use async_trait::async_trait;
use serde_json::{json, Value};

pub struct LocalProvider {
    endpoint: String,
    model: String,
    client: reqwest::Client,
}

impl LocalProvider {
    pub fn new(config: &crate::config::AiConfig) -> Self {
        Self {
            endpoint: config.endpoint.clone(),
            model: config.model.clone(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AiProvider for LocalProvider {
    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        thinking: bool,
    ) -> anyhow::Result<AiResponse> {
        let mut all_messages = vec![
            json!({ "role": "system", "content": system })
        ];
        for m in messages {
            all_messages.push(json!({ "role": m.role, "content": m.content }));
        }

        let mut body = json!({
            "model": self.model,
            "messages": all_messages,
            "stream": false,
        });

        // Granite 4.2 thinking mode
        if thinking {
            body["thinking"] = json!(true);
        }

        let resp = self.client
            .post(format!("{}/chat/completions", self.endpoint))
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;

        let content = resp["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(AiResponse { content, thinking: None })
    }
}
```

The `cloud.rs` implementation is identical but includes the `Authorization`
header with the API key. Both hit the same `/v1/chat/completions` endpoint.

---

## Database Schema

Deploy this to the buyer's SQL server via `sqlx migrate run`.
Every table has `company_id` — multi-tenant from day one.

```sql
-- migrations/001_init.sql

CREATE TABLE companies (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id  UUID NOT NULL REFERENCES companies(id),
    email       TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    role        TEXT NOT NULL DEFAULT 'loan_officer',
    -- roles: loan_officer | processor | underwriter | admin
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON users(company_id);

-- migrations/002_loans.sql

CREATE TABLE loans (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id      UUID NOT NULL REFERENCES companies(id),
    loan_number     TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'application',
    -- statuses: application | processing | underwriting | approval | closing | funded
    assigned_to     UUID REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON loans(company_id);
CREATE INDEX ON loans(assigned_to);

CREATE TABLE borrowers (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),
    first_name      TEXT NOT NULL,
    last_name       TEXT NOT NULL,
    email           TEXT,
    phone           TEXT,
    is_primary      BOOLEAN NOT NULL DEFAULT true,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE loan_income (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),
    borrower_id     UUID NOT NULL REFERENCES borrowers(id),
    income_type     TEXT NOT NULL,
    -- types: base | overtime | bonus | commission | self_employed | other
    monthly_amount  NUMERIC(12, 2) NOT NULL DEFAULT 0,
    employer_name   TEXT,
    years_on_job    NUMERIC(4, 1),
    notes           TEXT,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- migrations/003_tasks_locks.sql

CREATE TABLE tasks (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    loan_id         UUID NOT NULL REFERENCES loans(id) ON DELETE CASCADE,
    company_id      UUID NOT NULL REFERENCES companies(id),
    title           TEXT NOT NULL,
    description     TEXT,
    assigned_to     UUID REFERENCES users(id),
    screen          TEXT,  -- which screen this task belongs to
    status          TEXT NOT NULL DEFAULT 'open',
    -- statuses: open | in_progress | done
    created_by      UUID NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at    TIMESTAMPTZ
);

CREATE INDEX ON tasks(loan_id);
CREATE INDEX ON tasks(company_id, status);

-- Loan locking: first user in gets the lock, others are read-only
CREATE TABLE loan_locks (
    loan_id     UUID PRIMARY KEY REFERENCES loans(id) ON DELETE CASCADE,
    locked_by   UUID NOT NULL REFERENCES users(id),
    locked_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    screen      TEXT  -- which screen they're currently on
);

-- Immutable audit log: every data change goes here
CREATE TABLE audit_log (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    company_id  UUID NOT NULL REFERENCES companies(id),
    loan_id     UUID REFERENCES loans(id),
    user_id     UUID NOT NULL REFERENCES users(id),
    action      TEXT NOT NULL,     -- e.g. "income.update"
    table_name  TEXT NOT NULL,
    record_id   UUID NOT NULL,
    old_value   JSONB,
    new_value   JSONB,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX ON audit_log(loan_id);
CREATE INDEX ON audit_log(company_id, created_at);
```

---

## The Loan Lock System

```rust
// src-tauri/src/locks/mod.rs

use uuid::Uuid;

pub async fn acquire_lock(
    pool: &sqlx::PgPool,
    loan_id: Uuid,
    user_id: Uuid,
    screen: &str,
) -> anyhow::Result<bool> {
    // Try to insert; fails if loan_id already exists (another user has it)
    let result = sqlx::query!(
        r#"
        INSERT INTO loan_locks (loan_id, locked_by, screen)
        VALUES ($1, $2, $3)
        ON CONFLICT (loan_id) DO NOTHING
        "#,
        loan_id, user_id, screen
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn release_lock(
    pool: &sqlx::PgPool,
    loan_id: Uuid,
    user_id: Uuid,
) -> anyhow::Result<()> {
    sqlx::query!(
        "DELETE FROM loan_locks WHERE loan_id = $1 AND locked_by = $2",
        loan_id, user_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_lock(
    pool: &sqlx::PgPool,
    loan_id: Uuid,
) -> anyhow::Result<Option<LockInfo>> {
    let row = sqlx::query_as!(
        LockInfo,
        r#"
        SELECT ll.loan_id, ll.locked_by, ll.screen, u.name as user_name
        FROM loan_locks ll
        JOIN users u ON u.id = ll.locked_by
        WHERE ll.loan_id = $1
        "#,
        loan_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

#[derive(Debug, serde::Serialize)]
pub struct LockInfo {
    pub loan_id: Uuid,
    pub locked_by: Uuid,
    pub user_name: String,
    pub screen: Option<String>,
}
```

---

## AI Context Injection Pattern

Each screen defines a function that builds a system prompt from current
loan data. This is the core of what makes the AI useful.

```rust
// src-tauri/src/loans/screens/income.rs

use crate::loans::LoanIncome;

pub fn build_context(
    borrower_name: &str,
    income_records: &[LoanIncome],
    loan_type: &str,
) -> String {
    let total_monthly: f64 = income_records.iter()
        .map(|i| i.monthly_amount)
        .sum();

    let income_detail: String = income_records.iter()
        .map(|i| format!(
            "  - {} ({}): ${:.0}/month, employer: {}",
            i.income_type,
            if i.years_on_job.unwrap_or(0.0) >= 2.0 { 
                "2yr history ✓" 
            } else { 
                "< 2yr — verify" 
            },
            i.monthly_amount,
            i.employer_name.as_deref().unwrap_or("not listed"),
        ))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"You are an expert mortgage assistant helping a loan officer 
on the Income Analysis screen.

CURRENT LOAN CONTEXT:
  Borrower: {borrower_name}
  Loan type: {loan_type}

INCOME BREAKDOWN:
{income_detail}

  Total qualifying income: ${total_monthly:.0}/month

YOUR ROLE:
- Answer questions about this borrower's income qualification
- Flag potential issues (employment gaps, income type eligibility)
- Reference guidelines when asked (use the get_guidelines tool)
- Be concise — the user is in the middle of processing a loan

Do not discuss other screens, other borrowers, or general mortgage
topics unless directly related to this income analysis."#
    )
}
```

This pattern repeats for every screen. The context is always narrow,
always relevant, always injected fresh when the screen loads.

---

## Tauri IPC Commands

All database and AI calls go through Tauri commands — never direct from
the frontend.

```rust
// src-tauri/src/commands/ai_commands.rs

#[tauri::command]
pub async fn ai_chat(
    state: tauri::State<'_, AppState>,
    loan_id: String,
    screen: String,
    messages: Vec<crate::ai::Message>,
    thinking: bool,
) -> Result<String, String> {
    let loan_id = uuid::Uuid::parse_str(&loan_id).map_err(|e| e.to_string())?;

    // Build screen-appropriate context
    let system_context = build_context_for_screen(
        &state.db,
        loan_id,
        &screen,
    ).await.map_err(|e| e.to_string())?;

    // Call AI provider (local or cloud, doesn't matter here)
    let response = state.ai_provider
        .complete(&system_context, &messages, thinking)
        .await
        .map_err(|e| e.to_string())?;

    Ok(response.content)
}

async fn build_context_for_screen(
    pool: &sqlx::PgPool,
    loan_id: uuid::Uuid,
    screen: &str,
) -> anyhow::Result<String> {
    match screen {
        "income" => {
            // fetch income data, build context
            let income = crate::db::queries::get_income(pool, loan_id).await?;
            Ok(crate::loans::screens::income::build_context(
                "Borrower Name",  // fetch from DB
                &income,
                "conventional",   // fetch from loan
            ))
        }
        _ => Ok(format!("You are a mortgage assistant. Current screen: {screen}"))
    }
}
```

---

## SolidJS AI Sidebar Component

```typescript
// src/components/AiSidebar.tsx

import { createSignal, For } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';

interface Message {
  role: 'user' | 'assistant';
  content: string;
}

interface Props {
  loanId: string;
  screen: string;  // passed from whichever screen is active
}

export function AiSidebar(props: Props) {
  const [messages, setMessages] = createSignal<Message[]>([]);
  const [input, setInput] = createSignal('');
  const [loading, setLoading] = createSignal(false);

  const send = async () => {
    const text = input().trim();
    if (!text || loading()) return;

    const userMsg: Message = { role: 'user', content: text };
    setMessages(m => [...m, userMsg]);
    setInput('');
    setLoading(true);

    try {
      const response = await invoke<string>('ai_chat', {
        loanId: props.loanId,
        screen: props.screen,
        messages: messages(),
        thinking: false,  // toggle for complex questions
      });

      setMessages(m => [...m, { role: 'assistant', content: response }]);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div class="ai-sidebar">
      <div class="ai-header">
        <span>AI Assistant</span>
        <span class="screen-badge">{props.screen}</span>
      </div>

      <div class="ai-messages">
        <For each={messages()}>
          {(msg) => (
            <div class={`message ${msg.role}`}>
              {msg.content}
            </div>
          )}
        </For>
        {loading() && <div class="message assistant loading">Thinking...</div>}
      </div>

      <div class="ai-input">
        <input
          type="text"
          value={input()}
          onInput={(e) => setInput(e.currentTarget.value)}
          onKeyDown={(e) => e.key === 'Enter' && send()}
          placeholder={`Ask about ${props.screen}...`}
          disabled={loading()}
        />
        <button onClick={send} disabled={loading()}>Send</button>
      </div>
    </div>
  );
}
```

---

## Build Order

Work through these in strict order. Do not skip ahead.

### Phase 1 — Foundation (get this working before any UI)

1. **Config system** — `Config::load()` and `Config::save()` working,
   config.toml reads and writes to correct path
2. **AI abstraction layer** — `AiProvider` trait, `LocalProvider` and
   `CloudProvider` both implemented, test with a hardcoded prompt against
   local Ollama, confirm response comes back
3. **Database connection** — connect to a local Postgres or SQLite instance,
   run all migrations with `sqlx migrate run`
4. **Basic Tauri shell** — window opens, AppState initialized with DB pool
   and AI provider from config

### Phase 2 — First Working Screen

5. **Loan list screen** — read loans from DB, display in SolidJS table,
   no editing yet
6. **Loan detail + income screen** — display income records, all fields
   editable via mouse, save writes to DB and writes audit log entry
7. **AI sidebar on income screen** — context injection working, user can
   ask "can we use the bonus income?" and get a contextually correct answer

### Phase 3 — Team Layer

8. **Loan lock system** — acquire on screen entry, release on exit,
   display lock badge showing who has edit access
9. **Task layer** — create/complete tasks, unified list visible to all
   users on the same loan
10. **Presence indicator** — show which screen each user is currently on

### Phase 4 — Setup Wizard

11. **Wizard UI** — two-path flow (cloud AI or local AI), database
    connection string input, test connection button, schema deployment,
    admin account creation
12. **First-run detection** — if config is empty or `is_configured()`
    returns false, show wizard before main app

---

## Setup Wizard Flow

The wizard is the first thing IT sees. It must feel like installing Slack.

```
Step 1: Welcome
        "Orion — AI-native mortgage origination"
        [Get Started]

Step 2: Choose AI provider
        ◉ Local AI (recommended)
           - Runs on your hardware
           - No data leaves your network
           - No per-query cost
        ○ Cloud AI
           - Use your existing API key
           - OpenAI, Anthropic, Azure, or compatible

Step 3a (Local path):
        Checking your hardware...  ✓ Apple Silicon detected
        Downloading Granite 4.2:3B model... [=====>   ] 67%
        Running test inference...  ✓ 64 tokens/second
        [Continue]

Step 3b (Cloud path):
        Provider endpoint: [________________]
        API key:           [________________]
        Model name:        [________________]
        [Test Connection]  ✓ Connected

Step 4: Database
        Connection string: [________________________________]
        [Test Connection]  ✓ Connected to PostgreSQL 15
        [Deploy Schema]    ✓ 3 migrations applied

Step 5: Admin account
        Name:  [____________]
        Email: [____________]
        [Create Account & Launch]

Done → Opens to loan dashboard
```

---

## Design Principles — Apply to Every Screen

- **Mouse first.** Every action is clickable. No keyboard-only workflows.
- **Context always visible.** The loan name, borrower name, and current
  screen label are always on screen.
- **The AI sidebar is always present** on loan screens, on the right side.
  It is not a modal or a drawer — it is a persistent panel.
- **The task list is always present** as a collapsible panel, showing
  outstanding items for the current loan.
- **The lock badge** shows in the top bar on every loan screen. If the
  current user has the lock: green "Editing". If another user has it:
  amber "John P. is editing · Income screen".
- **Clean, professional UI.** This is enterprise software that costs real
  money. It should look like it.
- **No clever UX.** If a loan officer has to think about how to do
  something, the design failed.

---

## AppState

```rust
// Shared state across all Tauri commands
pub struct AppState {
    pub db: sqlx::PgPool,
    pub ai_provider: Box<dyn crate::ai::AiProvider>,
    pub config: crate::config::Config,
    pub current_user_id: std::sync::Mutex<Option<uuid::Uuid>>,
}
```

Initialize this in `main.rs` before the Tauri app starts. If config
is not configured, defer DB connection until after the wizard completes.

---

## First Commit Checklist

- [ ] `npm run tauri dev` opens a native window
- [ ] `Config::load()` and `Config::save()` work
- [ ] `LocalProvider::complete()` returns a response from Ollama
- [ ] Migrations run against a local DB
- [ ] `LoanList` screen shows a hardcoded or seeded loan

That's the foundation. Everything else is screens on top of this.

