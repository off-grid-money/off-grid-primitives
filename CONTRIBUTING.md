# Contributing to Off-Grid Primitives

Thanks for your interest in contributing. This document outlines how to report issues, propose changes, and submit contributions.

## Code of conduct

Be respectful and constructive. Focus on the code and the problem at hand.

## How to contribute

### Reporting bugs

- Use the [Bug report](.github/ISSUE_TEMPLATE/bug_report.md) issue template.
- Include steps to reproduce, expected vs actual behavior, and your environment (Rust version, OS, crate version).

### Suggesting features

- Use the [Feature request](.github/ISSUE_TEMPLATE/feature_request.md) issue template.
- Describe the use case, proposed API or behavior, and any alternatives you considered.

### Pull requests

1. **Fork and branch**  
   Fork the repo and create a branch from `main` (e.g. `fix/orderbook-snapshot`, `feat/asset-types`).

2. **Implement and test**  
   - Follow existing style and patterns.  
   - Add or update tests as needed.  
   - Run the test suite:
     ```bash
     cargo test
     ```

3. **Commit**  
   Use clear, concise commit messages (e.g. `fix(matching_engine): handle zero quantity`, `docs(spot): update README`).

4. **Open a PR**  
   - Target the `main` branch.  
   - Describe what changed and why.  
   - Reference any related issues.

5. **Review**  
   Address review feedback; we may ask for changes before merging.

## Development setup

### Prerequisites (all platforms)

- **Rust** — Stable toolchain, edition 2021. Install from [rustup.rs](https://rustup.rs/) (Windows, Linux, macOS).
- **Git** — For cloning and contributing.
- After cloning, run `cargo test` to confirm tests pass.

### Platform-specific setup

**Windows**

- Install [Rust with rustup](https://rustup.rs/) (use the “x86_64-pc-windows-msvc” or “x86_64-pc-windows-gnu” target as needed).
- Use **PowerShell** or **Command Prompt** for the commands below; WSL2 is also supported and follows the Linux steps.
- For pre-commit: install Python (e.g. from [python.org](https://www.python.org/) or Windows Store), then `pip install pre-commit`, or use `scoop install pre-commit` / `choco install pre-commit` if you use those package managers.

**Linux**

- Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` (or use your distro’s package manager and ensure `rustup` is available).
- Install build dependencies for your distro if you hit link errors (e.g. on Debian/Ubuntu: `build-essential`, `pkg-config`, and any libs required by your dependencies).
- For pre-commit: `pip install pre-commit` (or `pip3 install --user pre-commit`), or use your package manager (e.g. `apt install pre-commit` on Debian/Ubuntu if available).

**MacOS**

- Install Rust from [rustup.rs](https://rustup.rs/) or `brew install rustup-init && rustup init`.
- For pre-commit: `pip install pre-commit` or `brew install pre-commit`.

### Pre-commit checks (optional but recommended)

Pre-commit runs formatting, clippy, and tests before each commit so broken or unformatted code doesn’t get committed.

1. Install the [pre-commit](https://pre-commit.com/) framework using one of the options above for your OS.
2. Add Rust components used by the hooks (same on all platforms):
   ```bash
   rustup component add rustfmt clippy
   ```
3. Install the git hooks (same on all platforms):
   ```bash
   pre-commit install
   ```
4. On each commit, pre-commit will run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`. To run the same checks manually: `pre-commit run --all-files`.

If a hook fails: run `cargo fmt` to fix formatting, fix any clippy lints, and ensure `cargo test` passes, then commit again.

## Project structure

- `src/spot/` — Spot market primitives (L1/L2/L3, orderbook, matching engine, pair).  
- `src/account/` — Account balance interfaces (spot, futures, option).  
- `src/asset/` — Asset types (spot, futures, option).  
- `tests/` — Integration and component tests.

When adding or changing primitives, update the relevant `src/*/README.md` and the root [README](README.md) or [CHANGELOG](CHANGELOG.md) as appropriate.
