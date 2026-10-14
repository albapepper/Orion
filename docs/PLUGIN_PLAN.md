# Plugin-driven AI context (backend)

Status: **complete** (scaffold, 15 plugins, shared tools, metadata command).

Status: plan only — no code changes yet.  
Concept follows `scoracle-backend` harness → plugin → tools → LLM contract:  
the app harness runs an active plugin; the plugin owns domain scope and context assembly; tools fetch/measure data; the LLM reasons over the resulting context. Cheap code owns collection; the model owns discovery.

## Why

`src-tauri/src/commands/ai_commands.rs::build_context_for_screen` is one big `match screen` over 15+ screens. Every new tab means editing this central dispatcher, re-plumbing `Option<PgPool>`/demo branches, and re-adding fallbacks. The screen module (`loans::screens::*`) already holds the per-screen context builder, but discovery, ownership, and tool lists are implicit.

## Target shape

```
src-tauri/src/
  harness/
    plugin.rs          # ScreenPlugin trait + PluginRegistry + generic fallback
    context.rs         # PluginContext (state handle, loan_id, screen) + shared loaders
  plugins/
    mod.rs              # compile-time registration list
    dashboard.rs
    tasks.rs
    income.rs           # thin adapter over loans::screens::income
    …one per tab…
  tools/                # shared data tools plugins call (borrower name, list loans, etc.)
  loans/screens/        # stays: typed data + build_context remain here
```

A plugin = one active frontend tab. It declares:

- `id` — the screen key the frontend sends (`"income"`, `"terms"`, …)
- `build_context(&self, ctx) -> String` — owns DB-vs-demo fetch + context assembly
- `instructions()` — per-tab system voice appended to context (optional; most reuse the base loan context)
- `tools()` — list of tool names it uses (`["get_income", "get_borrowers"]`) for discovery/observability only (no live tool-calling runtime required)

## Core types

```rust
#[async_trait::async_trait]
pub trait ScreenPlugin: Send + Sync {
    fn id(&self) -> &'static str;
    fn tools(&self) -> &'static [&'static str] { &[] }
    fn instructions(&self) -> Option<&'static str> { None }
    async fn build_context(&self, ctx: &PluginContext<'_>) -> anyhow::Result<String>;
}

pub struct PluginRegistry {
    plugins: std::collections::HashMap<&'static str, std::sync::Arc<dyn ScreenPlugin>>,
    fallback: std::sync::Arc<dyn ScreenPlugin>,
}
impl PluginRegistry {
    pub fn new() -> Self { /* registers all plugins from plugins::mod.rs */ }
    pub fn get(&self, screen: &str) -> std::sync::Arc<dyn ScreenPlugin> { … }
}
```

`PluginContext` carries `loan_id: Option<Uuid>`, `state: &AppState` (or a narrower handle), and helpers like `borrower_name(...)` so fetch logic stays in plugins without repeating conversions.

## ai_chat flow after migration

1. Parse `loan_id`.
2. If `screen == "dashboard"/"tasks"` → registry plugin handles pipeline-level context (no loan).
3. Else → `registry.get(screen)`; if unknown → fallback generic plugin.
4. `plugin.build_context(&ctx).await` replaces the entire `match`.
5. Prepend `plugin.instructions()` if present; otherwise base assistant preamble (current fallback strings).
6. Call provider as today.

## Data ownership

Keep `loans::screens::*::build_context` as the typed renderer. Plugins are adapters: fetch with the DB/demo branch, then delegate rendering. Shared fetch helpers move to `tools/` (`borrower_name`, `demo_or_db`) to cut the repeated `match &db` blocks. Do not move DB queries out of `db::queries` — plugins only orchestrate.

## Registration

Single list in `plugins::mod.rs`:

```rust
pub fn register(reg: &mut PluginRegistry) {
    reg.add(DashboardPlugin);
    reg.add(IncomePlugin);
    …
}
```

Adding a tab = one entry here + one plugin file + one frontend tab. No dispatcher edit.

## Migration sequence (smallest safe slices)

1. **Scaffold only.** Add `harness/plugin.rs` + `PluginRegistry` + generic fallback. Add `plugins::mod.rs` with Dashboard/Tasks registered as plugins wrapping the existing `build_dashboard_context`/`build_tasks_context`. `ai_chat` uses registry for those two; rest stays in `match`. Build + tests.
2. **Loan plugins, one group at a time.** Move `income`, `terms`, `borrower`, … into plugin files. Each plugin keeps its exact DB/demo branch and delegates to `loans::screens::*::build_context`. Remove the corresponding `match` arm only after a byte-identical context snapshot test passes. Order: income → terms → borrower → employment → assets/liabilities/property/reo/declarations → documents/conditions/pricing.
3. **Shared tools.** Extract `borrower_name` and the demo/db boilerplate into `tools/`; plugins call them. No behavior change.
4. **Metadata/command.** Optional IPC `list_plugins` returning `{ id, tools }` so the frontend can render tabs/legend from the backend. Frontend tabs can stay hard-coded; this is additive.
5. **Cleanup.** Delete the now-empty `match` and unused helpers. Update `docs/PILOT_CHECKLIST`/roadmap if desired.

## Invariants to preserve

- Unknown screen still returns the current generic "concise expert assistant" context (never an error that breaks chat).
- Demo mode: every plugin keeps its `None` branch returning seeded data; no persistence calls.
- Context bytes must match before/after for each screen (capture one request per screen as a frozen fixture before migrating that screen).
- Loan locking/audit unaffected — plugins are read-only context builders.
- Keyboard + route handling in the frontend untouched; plugin id == existing `LoanScreen` string.

## Out of scope (YAGNI for now)

- Live tool-calling by the LLM (tools are preparation APIs today).
- Dynamic plugin loading from disk / wasm.
- Per-plugin prompts beyond the current context strings.
- Changing the ChatBox UI or divider behavior.
- Replacing the frontend tab list with a backend-driven registry (optional, later).

## Acceptance for "done"

- `ai_chat` has no `match` over screen names; it delegates to `PluginRegistry`.
- Each former screen renders an identical system context to a committed fixture snapshot.
- `cargo test` (existing) still passes; add one `#[test]` per plugin asserting non-empty context from demo data.