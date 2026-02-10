# Desktop App Template — CLAUDE.md

> Tauri 2.x + React + Rust + SQLite desktop application template.

---

## Project Structure

```
my-app/
├── package.json
├── tsconfig.json
├── vite.config.ts
├── index.html
├── src/                          # React frontend
│   ├── main.tsx                  # React entry point
│   ├── App.tsx                   # Root component with router
│   ├── app/
│   │   ├── router.tsx            # Route definitions
│   │   └── routes/               # Page-level route components
│   ├── components/               # Shared UI components (pure/presentational)
│   │   ├── ui/                   # Primitive UI components (shadcn/ui style)
│   │   └── layouts/              # Layout shells
│   ├── features/                 # Feature modules (business logic + UI)
│   │   └── <feature>/
│   │       ├── components/       # Feature-specific components
│   │       ├── hooks/            # Feature-specific hooks
│   │       └── api.ts            # Tauri invoke wrappers for this feature
│   ├── hooks/                    # Shared React hooks
│   ├── stores/                   # Zustand stores (global UI state)
│   ├── lib/                      # Utility functions, Tauri helpers
│   │   ├── tauri.ts              # Typed invoke wrappers / tauri-specta bindings
│   │   └── utils.ts
│   ├── styles/                   # Global styles / Tailwind config
│   └── test/                     # Test infrastructure
│       ├── setup.ts              # Vitest global setup
│       ├── test-utils.tsx        # Custom render with providers
│       └── mocks/                # MSW handlers, mock data
├── src-tauri/                    # Rust backend
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json           # Primary Tauri configuration
│   ├── capabilities/             # Security capability definitions
│   │   └── default.json
│   ├── icons/                    # App icons (all platforms)
│   ├── migrations/               # SQLite migration SQL files
│   └── src/
│       ├── main.rs               # Desktop entry point (thin)
│       ├── lib.rs                # App builder setup, plugin registration
│       ├── commands/             # Tauri command modules
│       │   ├── mod.rs
│       │   └── <domain>.rs
│       ├── db/                   # Database layer
│       │   ├── mod.rs
│       │   ├── connection.rs     # Pool setup, migration runner
│       │   ├── models.rs         # Data structs (Serialize + FromRow)
│       │   └── test_helpers.rs   # create_test_db(), seed functions
│       ├── state.rs              # AppState struct definitions
│       └── error.rs              # Unified error types
│   └── tests/                    # Rust integration tests
│       ├── common/
│       │   └── mod.rs            # Shared test helpers
│       ├── db_integration.rs
│       └── ipc_integration.rs
├── e2e/                          # Playwright E2E tests
│   ├── playwright.config.ts
│   ├── fixtures/                 # Custom Playwright fixtures
│   ├── pages/                    # Page Object Models
│   ├── helpers/                  # Mock data, IPC mocks
│   └── specs/                    # Test spec files
└── .github/
    └── workflows/                # CI/CD pipelines
```

---

## Technology Stack

| Layer | Tool | Rationale |
|-------|------|-----------|
| Desktop framework | Tauri 2.x | Lightweight, Rust-native, secure |
| Frontend bundler | Vite | Fast HMR, native ESM, first-class Tauri support |
| Frontend language | TypeScript (strict) | Type safety across the IPC boundary |
| UI components | shadcn/ui + Tailwind CSS v4 | Composable unstyled primitives; works well in desktop contexts |
| Client state | Zustand v5 | Minimal boilerplate, works outside React tree |
| Server/persistent state | TanStack Query v5 | Caching, invalidation, and sync for Tauri backend data |
| Routing | React Router or TanStack Router | SPA routing for multi-view desktop apps |
| Backend language | Rust | Tauri-native, safe, performant |
| Database | SQLite via sqlx | Async, compile-time query checking, built-in migrations |
| Error handling | thiserror | Ergonomic error types with `#[from]` conversion |
| Frontend tests | Vitest + React Testing Library | Shares Vite config, native ESM/TS |
| Backend tests | Built-in Rust test framework | In-memory SQLite, mockall, proptest |
| E2E tests | Playwright | Auto-waiting, tracing, cross-platform |

---

## Tauri Architecture

### Commands (IPC)

Commands are the primary frontend-to-backend communication. They follow a request/response pattern via `invoke()`.

- **Always use `async` commands** for anything involving I/O (database, file system, network). Sync commands block the main thread and freeze the UI.
- **Always return `Result<T, AppError>`** where `AppError` implements `serde::Serialize`. Never panic in commands.
- **Keep commands thin** — validate input, call into a service/db layer, return results. Business logic belongs in separate modules.
- **Use `#[tauri::command(rename_all = "camelCase")]`** to match JavaScript naming conventions.
- Register all commands in `lib.rs` via `tauri::generate_handler![]`.

### Events

Events are fire-and-forget, bidirectional messages. Use them for:
- Notifying the frontend of backend state changes (e.g., "download-progress")
- Broadcasting changes across multiple windows
- Lifecycle notifications

### Channels

For streaming/large data transfers (file reads, progress), use Tauri channels instead of repeated events.

### Capabilities & Security

- Define capabilities in `src-tauri/capabilities/default.json`.
- Apply the **Principle of Least Privilege** — only grant permissions each window actually needs.
- Use separate capability files for different windows.
- Always sanitize user input in commands; never trust frontend data.
- Set a strict CSP in `tauri.conf.json`.

### Type Safety

Use [tauri-specta](https://github.com/specta-rs/tauri-specta) to generate TypeScript bindings from Rust commands. This eliminates string-based command names and provides full autocomplete.

---

## Frontend Patterns

### Three-Layer State Management

1. **Component state (`useState`)** — ephemeral UI state (form inputs, toggle visibility, hover states).
2. **Zustand** — global UI state not tied to backend data (sidebar open, theme, modals).
3. **TanStack Query** — all persistent/backend data fetched via Tauri `invoke()`. Wrap `invoke()` calls in `useQuery`/`useMutation` hooks per feature.

### Desktop-Specific UI

- Custom title bars: use `decorations: false` + `data-tauri-drag-region` for custom window chrome.
- Platform-aware strings: detect OS and adjust UI text ("Reveal in Finder" vs "Show in Explorer").
- Unified command palette: route keyboard shortcuts through a Cmd+K / Ctrl+K pattern.
- Multi-window sync: use Tauri events to synchronize state across windows.

---

## Rust Backend Patterns

### Application State

- Define state in `state.rs`. Tauri's `manage()` handles shared ownership — **do NOT wrap in `Arc`**.
- Use `std::sync::Mutex` (not `tokio::Mutex`) for simple mutable state when critical sections are short.
- Access state in commands via `state: State<'_, AppState>`.

### Error Handling

Use `thiserror` for defining error variants with `#[from]` for automatic conversion. Implement `serde::Serialize` manually (serialize as string) so errors cross the IPC boundary cleanly.

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    Validation(String),
}
```

### SQLite Best Practices

- **WAL mode** (`journal_mode(SqliteJournalMode::Wal)`) — enables concurrent reads during writes.
- **`create_if_missing(true)`** — database file created on first launch.
- **Pool size 1–5** — SQLite is single-writer; a small pool suffices.
- **Store DB in app data dir** — use `app.path().app_data_dir()`.
- **Migrations** — store in `src-tauri/migrations/` as numbered SQL files. Run at startup via `sqlx::migrate!()`. Make migrations idempotent (`IF NOT EXISTS`).
- **Backend-driven SQL** — keep all SQL on the Rust side, expose only typed commands to the frontend. Avoid `tauri-plugin-sql` in production; use custom sqlx integration.

---

## Frontend Testing (Vitest + React Testing Library)

### Setup

- Use Vitest with `jsdom` environment. Configure in `vite.config.ts` under `test`.
- Setup file: `src/test/setup.ts` — import `@testing-library/jest-dom/vitest`, call `cleanup()` after each test.
- Custom render wrapper: `src/test/test-utils.tsx` — wraps components with all providers (Router, Theme, etc.). Import `render` from here, not from `@testing-library/react`.

### Test Organization

- **Co-locate test files** next to source: `Button.tsx` → `Button.test.tsx`.
- Name tests with human-readable sentences describing behavior, not implementation.
- Structure: imports → mocks → describe → beforeEach → tests grouped by behavior.

### Component Testing Rules

1. **Always use `screen`** — do not destructure queries from `render()`.
2. **Use `userEvent` over `fireEvent`** — `userEvent.setup()` before render, use returned `user` throughout.
3. **Never wrap `render`/`fireEvent` in `act()`** — they handle it internally.
4. **Query priority (accessibility-first):**
   - `getByRole` (first choice for almost everything)
   - `getByLabelText` (form fields)
   - `getByPlaceholderText`
   - `getByText` (non-interactive elements)
   - `getByTestId` (**last resort only**)

### Mocking Tauri IPC

- Use `@tauri-apps/api/mocks` with `mockIPC`, `mockWindows`, `clearMocks`.
- Add crypto polyfill in setup for jsdom (Tauri IPC needs WebCrypto).
- Clear mocks in `afterEach`.

### Mocking HTTP

- Use **MSW (Mock Service Worker)** for any HTTP calls outside Tauri IPC.
- Define handlers in `src/test/mocks/handlers.ts`, server in `src/test/mocks/server.ts`.
- `beforeAll(() => server.listen())`, `afterEach(() => server.resetHandlers())`, `afterAll(() => server.close())`.

### Async Testing

- Use `findBy*` when expecting an element to appear after async operation.
- Use `waitFor` when asserting on something other than element presence.
- **Side effects BEFORE `waitFor`, assertions INSIDE `waitFor`** — never put clicks inside `waitFor`.

### Zustand Testing

- Mock Zustand's `create` to auto-reset stores between tests (see Zustand docs testing guide).
- This prevents state leakage between tests.

### Coverage

- Use `@vitest/coverage-v8`. Target: 80% lines/functions/statements, 75% branches.
- Exclude test files, entry points, type definitions from coverage.

---

## Rust Backend Testing

### Three-Tier Testing Strategy

**Tier 1 — Pure logic tests (no Tauri, no DB):**
Test validation, transformations, business rules. Fastest and most reliable. Maximize coverage here.

**Tier 2 — Logic + DB tests (no Tauri):**
Use real in-memory SQLite via `Connection::open_in_memory()`. Test actual SQL queries and schema. Still fast.

**Tier 3 — Tauri IPC tests (full mock runtime):**
Use `tauri::test::mock_builder()` + `assert_ipc_response()`. Use sparingly — only to verify serialization and IPC routing.

### Key Practices

- **Separate business logic from Tauri commands.** Keep `#[tauri::command]` functions as thin wrappers that delegate to plain Rust functions. Test the plain functions directly.
- **In-memory SQLite for all DB tests.** Each test gets a fresh database via `Connection::open_in_memory()` with migrations applied. Tests run in parallel without interference.
- **Test migrations explicitly.** Verify they are valid, apply cleanly, and are idempotent.
- **Trait-based dependency injection** for the data layer. Use `mockall` with `#[automock]` for mocking when needed.
- **Test error paths explicitly** with `is_err()`, `matches!()`, `unwrap_err()`.
- **Property-based testing** with `proptest` for serialization round-trips, validation functions, and invariants.

### Test Organization

- **Unit tests:** `#[cfg(test)] mod tests` at the bottom of each source file (access to private items).
- **Integration tests:** `tests/` directory (public API only, forces clean API design).
- **Shared helpers:** `tests/common/mod.rs` for integration tests, `src/db/test_helpers.rs` for unit tests.
- **Naming:** `test_<function>_<scenario>`, e.g., `test_insert_note_empty_title_returns_validation_error`.

### Coverage

- Use `cargo-llvm-cov` (cross-platform: Windows, macOS, Linux).
- Target 80%+ on `commands/`, `db/`, and `models/`. Don't chase coverage on Tauri boilerplate.

### Tauri Test Feature

Enable `test` feature in Cargo.toml: `tauri = { version = "2", features = ["test"] }`.

### Windows Gotcha

`cargo test` with Tauri's mock runtime can fail with `STATUS_ENTRYPOINT_NOT_FOUND` on Windows. Apply the build.rs manifest workaround (see tauri-apps/tauri#13419).

---

## Playwright E2E Testing

### Testing Approach

**Primary: Test the web layer directly (Approach A).** Run Playwright against the Vite dev server with Tauri IPC mocked via `@tauri-apps/api/mocks`. This is fast, cross-platform, and covers 90% of test needs.

**Optional: CDP connection to Tauri binary (Approach B, Windows only).** Connect Playwright to WebView2 via Chrome DevTools Protocol for true end-to-end testing with the real Rust backend. Use only for scenarios requiring the real backend.

### Configuration

```typescript
// playwright.config.ts key settings:
fullyParallel: true,
retries: process.env.CI ? 2 : 0,
workers: process.env.CI ? 4 : undefined,
trace: 'on-first-retry',
screenshot: 'only-on-failure',
video: 'on-first-retry',
```

- Test primarily against **Chromium** (Tauri's WebView2 is Chromium-based).
- Use `webServer` config to start dev server automatically.
- Set `VITE_PLAYWRIGHT=true` env var to load IPC mocks in the frontend.

### Test Organization

- Directory: `e2e/specs/` for test files, `e2e/pages/` for Page Objects, `e2e/fixtures/` for custom fixtures.
- Use **Page Object Model**: encapsulate locators and actions in page classes. Do not put assertions in page objects.
- Wire page objects via Playwright fixtures (`test.extend<AppFixtures>`).
- Use setup projects for authentication state reuse.

### Selectors (Priority Order)

1. `getByRole` — mirrors how users and assistive tech perceive the page
2. `getByLabel` / `getByPlaceholder` — form elements
3. `getByText` — non-interactive content
4. `getByTestId` — last resort for complex/non-semantic elements
5. CSS/XPath — avoid; brittle

### Waiting & Stability

- **Rely on auto-waiting.** Playwright waits for elements to be actionable before performing actions.
- **Use web-first assertions:** `await expect(locator).toBeVisible()` (retries automatically).
- **Never use `waitForTimeout`** in production tests.
- Disable CSS animations in test mode for stability.

### Network & IPC Mocking

- Use `page.route()` to mock HTTP requests.
- Use `mockIPC` from `@tauri-apps/api/mocks` for Tauri command mocking.
- Set up mocks BEFORE navigation.

### Data Isolation

- Each test runs in a fresh browser context (cookies, localStorage isolated).
- For database state: use API-based seeding, fresh DB per worker, or transaction rollback.

### CI Integration

- Install only needed browsers: `npx playwright install chromium --with-deps`.
- Upload HTML report as artifact on every run. Upload traces only on failure.
- Use test sharding (`--shard=N/M`) across parallel CI jobs for large suites.

### Visual Regression

- Use `toHaveScreenshot()` for layout-heavy pages, custom components, theme switching.
- Mask dynamic content (timestamps, avatars) to avoid false positives.
- Run visual tests in a consistent environment (pin OS, browser version, viewport, fonts).

---

## Build & Dev Workflow

### Development

- `cargo tauri dev` — starts Vite dev server + compiles Rust + opens app window.
- Frontend HMR works instantly. Rust auto-rebuilds on file changes (incremental).
- For frontend-only work: run `npm run dev` and open `localhost:5173` in browser. Mock `invoke()` calls.

### Production Build

- `cargo tauri build` — compiles frontend to `dist/`, Rust in release mode, generates platform installers (.msi, .dmg, .AppImage/.deb).

### Speed Tips

- Use a faster linker (lld on Linux/Windows, zld on macOS) via `.cargo/config.toml`.
- Set `RUST_LOG=debug` for Tauri internal logging during dev.

### Cross-Platform CI

Tauri cannot cross-compile. Use GitHub Actions with `tauri-apps/tauri-action` on platform-specific runners (ubuntu, macos, windows).

### Quality Gate

Run a single `npm run check:all` command before every commit:
- TypeScript type checking
- ESLint + Prettier
- `cargo clippy`
- `cargo test`
- `vitest run`

---

## Code Comments

### Core Principle

**Code says what and how; comments say why.** Before writing a comment, ask: can I rename a variable, extract a function, or add a type to make this self-evident? If yes, do that instead. Comments that restate what code does are noise — they rot when code changes and train developers to ignore all comments.

### Decision Framework

1. Can I rename something to make this comment unnecessary? **Do that instead.**
2. Can I extract a well-named function? **Do that instead.**
3. Can I add a type or interface that makes the constraint explicit? **Do that instead.**
4. Is this explaining **why**, not **what**? **Write the comment.**
5. Is this documenting a public API others consume without reading source? **Write the doc comment.**

### Anti-Patterns to Avoid

- **Redundant**: `let count = 0; // initialize count to zero`
- **Journal**: `// 2024-01-15 emmet: added validation` — git blame does this better
- **Mumbling**: `// not sure if this works` — fix it or file an issue
- **Mandated**: `@param id The id` on every function — zero value, maximum clutter
- **Closing brace**: `} // end for loop` — extract a function if the block is too long
- **Commented-out code**: use version control; dead code confuses readers
- **Misleading**: comment says "never returns null" but code does — actively causes bugs

### When Comments ARE Valuable

- Workarounds linking to browser bugs, upstream issues, platform quirks
- Performance justifications (why O(n^2) was chosen for small n)
- Invariants and constraints ("this list is always sorted; binary search depends on it")
- Regex explanations for complex patterns
- "Why not" — explaining why an obvious alternative was rejected
- Safety contracts (Rust `# Safety` sections, any unsafe/unchecked operation)
- Public API documentation for shared modules

### TypeScript / React

**Use TSDoc syntax** (`/** ... */`). Types already document shape; comments document intent.

**Do document:**
- Components with non-obvious behavior or complex prop interactions
- Custom hooks (especially return values, side effects, cancellation behavior)
- Props where the name alone doesn't convey constraints or valid ranges
- Anything exported from shared `components/` or `hooks/` directories

**Skip documenting:**
- Trivial components where name and props are self-explanatory
- Props already well-described by their TypeScript type and name
- Internal components used in only one place

```tsx
// BAD — restating types the compiler already knows:
/** @param title - The title string */

// GOOD — describing behavior the signature cannot convey:
/**
 * Modal dialog that traps focus and disables background scrolling.
 * Closes on Escape key and backdrop click. Uses `isOpen` prop
 * instead of conditional rendering so exit animation plays.
 */
function Dialog({ title, isOpen, onClose }: DialogProps) { ... }
```

```tsx
// GOOD — documenting a prop where the type alone is insufficient:
interface DataTableProps {
  /**
   * Maximum rows before virtualizing.
   * Set to 0 to disable (not recommended for datasets > 500 rows).
   */
  virtualizationThreshold?: number;
}
```

```tsx
// GOOD — documenting a custom hook:
/**
 * Debounced search that cancels in-flight Tauri IPC calls on query change.
 * Returns stale results until new query resolves (no loading flash).
 */
function useDebouncedSearch(query: string, delayMs: number) { ... }
```

**TODO format**: `TODO(<ticket-or-author>):` with a ticket number or expiration date. Naked TODOs rot.
```tsx
// TODO(#142): Replace with server-side pagination once the API supports it
// TODO(emmet, 2025-06-01): Remove this polyfill after dropping Safari 16
```

### Rust

Use `///` for items below, `//!` for the enclosing module/crate. First line is a **single-sentence summary** in third-person present ("Returns", "Creates", not "Return" or "This function returns").

**Standard sections** (per RFC 1574 and Rust API Guidelines):

- `# Panics` — when the function can panic and under what conditions
- `# Errors` — what error variants are returned and when
- `# Safety` — invariants callers must uphold for `unsafe` functions
- `# Examples` — runnable code blocks (compiled as doctests by `cargo test`)

```rust
/// Parses a configuration file from the given path.
///
/// # Errors
///
/// Returns `Err(ConfigError::NotFound)` if the path does not exist.
/// Returns `Err(ConfigError::Parse)` if the file contains invalid TOML.
pub fn from_path(path: &str) -> Result<Config, ConfigError> { ... }
```

**Public vs private:**
- **Public API**: document everything. Enable `#![warn(missing_docs)]` at crate level.
- **Private code**: do not mandate doc comments on every private function. Use `//` inline comments for the "why" when non-obvious.

**Doctests stay honest** — code in `///` blocks is compiled and run by `cargo test`. Use `?` (not `unwrap()`) in examples. Hide boilerplate with `# ` prefix lines.

**Enable in CI:**
```rust
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
```

### Tauri Commands

Commands bridge the frontend/backend boundary and warrant extra documentation because: the function signature doesn't reveal what the frontend sees (JSON serialization, camelCase renaming), error behavior must be understood by both Rust and TypeScript developers, and side effects are invisible to the frontend caller.

```rust
/// Creates a new note and persists it to the database.
///
/// # Errors
///
/// Returns `AppError::Validation` if `title` is empty or exceeds 200 chars.
/// Returns `AppError::Database` if the insert fails.
#[tauri::command(rename_all = "camelCase")]
pub async fn create_note(
    state: State<'_, AppState>,
    title: String,
    body: String,
) -> Result<Note, AppError> { ... }
```

If not using tauri-specta, optionally include a `# Frontend Usage` section with the TypeScript `invoke()` call.

### SQL Migrations

Migrations are **immutable records**. Comments explain the **why** of schema changes for future developers reading migration history.

**Do comment:**
- Header block: migration purpose, business context
- Non-obvious schema decisions (why COLLATE NOCASE, why CASCADE vs RESTRICT)
- Indexes: explain the query pattern the index supports
- Data migrations: document reversibility

**Don't comment the obvious:**
```sql
-- BAD: "Create the tags table" above a CREATE TABLE tags statement
-- GOOD:
-- Tags are case-insensitive (COLLATE NOCASE) to prevent
-- duplicates like "Work" and "work".
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE COLLATE NOCASE
);

-- Index for reverse lookup: "which notes have this tag?"
CREATE INDEX IF NOT EXISTS idx_note_tags_tag_id ON note_tags(tag_id);
```

### Test Files

**Test names are the primary documentation.** Comments are for context that names and assertions cannot convey: why an edge case matters, what bug a regression test prevents, or what real-world scenario the test simulates.

| Test Type | Primary Documentation | When to Add Comments |
|-----------|----------------------|---------------------|
| Vitest/RTL | Descriptive `it('...')` strings | Regression context, non-obvious mock rationale, async timing nuances |
| Rust | `test_<fn>_<scenario>` names | Regression links (`#issue`), subtle invariants, non-obvious setup |
| Playwright | User-story-style test titles | Wait strategy explanations, mock setup context, real bug references |

```tsx
// BAD — commenting the obvious:
// Test that the form renders
it('renders', () => { ... });

// GOOD — comment adds non-obvious context:
it('disables submit while login request is in-flight', async () => {
  // ...
  // Must be disabled immediately, before IPC resolves,
  // to prevent double-submission (see incident #47).
  expect(screen.getByRole('button', { name: 'Sign in' })).toBeDisabled();
});
```

```rust
/// Regression test for #89: empty titles were silently accepted,
/// causing NULL constraint violations on sync.
#[test]
fn test_validate_note_empty_title_returns_error() { ... }
```

### Configuration Files

Config files are often the most confusing files for newcomers. Comments here are high-value because options are terse and the "why" is invisible.

```typescript
// vite.config.ts
server: {
  port: 5173,
  strictPort: true, // Tauri expects a fixed port; random ports break tauri.conf.json devUrl
},
base: '', // Tauri uses tauri:// protocol; empty base ensures relative paths in built HTML
```

```toml
# Cargo.toml
# sqlx "offline" feature enables compile-time query checking without a live database
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "migrate", "offline"] }
```

For JSON files that don't support comments (`tauri.conf.json`): rely on the Tauri configuration reference, or use `tauri.conf.json5` format which supports `//` comments.

For `tsconfig.json`: TypeScript's parser supports `//` comments (JSONC). Use sparingly for non-obvious compiler flags.

---

## Commands Reference

```bash
# Development
cargo tauri dev                    # Start app in dev mode
npm run dev                        # Frontend-only dev server

# Testing
npm run test                       # Vitest watch mode
npm run test:run                   # Vitest single run
npm run test:coverage              # Vitest with coverage
cargo test                         # All Rust tests
cargo test --lib                   # Rust unit tests only
cargo test --test <name>           # Single integration test
cargo llvm-cov --html              # Rust coverage report
npx playwright test                # E2E tests
npx playwright test --ui           # E2E with interactive UI
npx playwright codegen             # Generate test selectors

# Building
cargo tauri build                  # Production build with installers

# Database
sqlx migrate add <name>            # Create new migration
```
