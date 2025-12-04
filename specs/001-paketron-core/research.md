# Research: Tech Stack Selection

**Feature**: Paketron Core (001-paketron-core)
**Date**: 2025-12-04

## Decision: Rust

We have selected **Rust** as the primary language for Paketron.

### Rationale
1.  **Performance**: Rust offers C++ level performance with zero-cost abstractions. Startup time is near-instant, which is critical for a CLI tool.
2.  **Safety**: Memory safety guarantees without a garbage collector prevent common crashes and vulnerabilities.
3.  **Distribution**: Rust compiles to a single, static binary. Users do not need to install a runtime (like Node.js or .NET) to use Paketron.
4.  **Windows Support**: The `windows-rs` crate provides excellent, official access to Windows APIs.
5.  **Ecosystem**: Strong CLI ecosystem (`clap`, `tokio`, `serde`, `reqwest`) makes development rapid.

### Alternatives Considered

#### Node.js
-   **Pros**: Rapid development, huge package ecosystem.
-   **Cons**: Requires runtime or large bundled binary. Slower startup time.
-   **Verdict**: Rejected due to performance and distribution overhead.

#### C++
-   **Pros**: Native performance, standard for system tools.
-   **Cons**: Manual memory management, complex build systems, lack of modern package management for dependencies.
-   **Verdict**: Rejected due to safety concerns and developer experience compared to Rust.

#### C# (Chocolatey)
-   **Pros**: Native to Windows, huge standard library.
-   **Cons**: Requires .NET runtime. We aim to compete by being lighter and faster.
-   **Verdict**: Rejected to differentiate from Chocolatey.

## Technical Context for Plan

-   **Language**: Rust (Latest Stable)
-   **Build System**: Cargo
-   **Target**: Windows (x86_64-pc-windows-msvc)
-   **Key Crates**:
    -   `clap` (CLI parsing)
    -   `tokio` (Async runtime)
    -   `reqwest` (HTTP client)
    -   `serde` / `serde_json` / `toml` (Serialization)
    -   `windows` (Windows API)
    -   `indicatif` (Progress bars)
