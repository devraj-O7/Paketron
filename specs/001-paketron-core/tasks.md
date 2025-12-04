---
description: "Task list for Paketron Core (CLI)"
---

# Tasks: Paketron Core

**Input**: Design documents from `/specs/001-paketron-core/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

## Phase 1: Setup (Shared Infrastructure)

- [x] T001 Create Rust project structure (`cargo new paketron`)
- [x] T002 Add dependencies to `Cargo.toml` (`clap`, `tokio`, `reqwest`, `serde`, `windows`)
- [x] T003 [P] Configure rustfmt and clippy

---

## Phase 2: Foundational (Blocking Prerequisites)

- [x] T004 Define `Package` struct and serialization in `src/core/package.rs`
- [x] T005 Implement `Registry` trait and file-based storage in `src/core/registry.rs`
- [x] T006 [P] Setup basic CLI argument parsing with `clap` in `src/cli.rs`
- [x] T007 Implement basic `Downloader` struct in `src/core/downloader.rs`

---

## Phase 3: User Story 1 - Install a Package (Priority: P1)

**Goal**: Enable users to install packages from the registry.

- [x] T008 [US1] Create mock registry file for testing
- [x] T009 [US1] Implement `install` command logic in `src/commands/install.rs`
- [x] T010 [US1] Implement package download and verification logic
- [x] T011 [US1] Implement installer execution (silent mode)
- [x] T012 [US1] Update local registry after successful install
- [ ] T013 [US1] Add integration test for install flow

---

## Phase 4: User Story 2 - Search for Packages (Priority: P2)

**Goal**: Enable users to find packages.

- [x] T014 [US2] Implement `search` command logic in `src/commands/search.rs`
- [x] T015 [US2] Implement fuzzy search or substring match on registry
- [x] T016 [US2] Display search results in a table format

---

## Phase 5: User Story 3 - Uninstall a Package (Priority: P2)

**Goal**: Enable users to remove packages.

- [x] T017 [US3] Implement `uninstall` command logic in `src/commands/uninstall.rs`
- [x] T018 [US3] Implement uninstaller execution
- [x] T019 [US3] Remove package from local registry
