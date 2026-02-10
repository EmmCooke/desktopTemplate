# Desktop App Template

A project template for building desktop applications with a **React** frontend, **Rust** backend, and **SQLite** database, powered by [Tauri 2.x](https://v2.tauri.app/).

## Stack

| Layer | Technology |
|-------|------------|
| Desktop framework | [Tauri 2.x](https://v2.tauri.app/) |
| Frontend | [React](https://react.dev/) + [TypeScript](https://www.typescriptlang.org/) + [Vite](https://vite.dev/) |
| UI | [shadcn/ui](https://ui.shadcn.com/) + [Tailwind CSS v4](https://tailwindcss.com/) |
| State management | [Zustand](https://zustand.docs.pmnd.rs/) (client) + [TanStack Query](https://tanstack.com/query) (server) |
| Backend | [Rust](https://www.rust-lang.org/) |
| Database | [SQLite](https://www.sqlite.org/) via [sqlx](https://github.com/launchbadge/sqlx) |
| Frontend tests | [Vitest](https://vitest.dev/) + [React Testing Library](https://testing-library.com/docs/react-testing-library/intro/) |
| Backend tests | Rust built-in + [mockall](https://github.com/asomers/mockall) + [proptest](https://github.com/proptest-rs/proptest) |
| E2E tests | [Playwright](https://playwright.dev/) |

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v20+)
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Tauri CLI](https://v2.tauri.app/reference/cli/) (`cargo install tauri-cli`)
- Platform-specific dependencies — see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

### Development

```bash
# Install frontend dependencies
npm install

# Start the app in dev mode (frontend + backend)
cargo tauri dev

# Frontend-only dev server (for UI work without Rust compilation)
npm run dev
```

### Testing

```bash
# Frontend tests
npm run test              # watch mode
npm run test:run          # single run
npm run test:coverage     # with coverage

# Backend tests
cargo test                # all Rust tests
cargo test --lib          # unit tests only

# E2E tests
npx playwright test       # headless
npx playwright test --ui  # interactive UI
```

### Building

```bash
# Production build with platform-specific installers
cargo tauri build
```

## Project Structure

```
├── src/                  # React frontend
│   ├── app/              #   Routes and router config
│   ├── components/       #   Shared UI components
│   ├── features/         #   Feature modules (components + hooks + API)
│   ├── hooks/            #   Shared React hooks
│   ├── stores/           #   Zustand stores
│   ├── lib/              #   Utilities and Tauri helpers
│   └── test/             #   Test setup, custom render, mocks
├── src-tauri/            # Rust backend
│   ├── src/
│   │   ├── commands/     #   Tauri command handlers
│   │   ├── db/           #   Database layer (connection, models, queries)
│   │   ├── error.rs      #   Unified error types
│   │   └── state.rs      #   Application state
│   ├── migrations/       #   SQLite migrations
│   └── capabilities/     #   Tauri security permissions
├── e2e/                  # Playwright E2E tests
│   ├── specs/            #   Test files
│   ├── pages/            #   Page Object Models
│   └── fixtures/         #   Custom test fixtures
└── CLAUDE.md             # Development conventions and best practices
```

## Conventions

All development conventions — architecture patterns, testing strategies, commenting guidelines, and issue formatting — are documented in [CLAUDE.md](CLAUDE.md).

