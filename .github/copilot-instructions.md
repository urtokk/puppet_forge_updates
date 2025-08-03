# Copilot Instructions for puppet_forge_updates

Welcome, Copilot!
This document provides explicit instructions for contributing code, writing commit messages, and following best practices in this repository. Please adhere to these guidelines to ensure high code quality, maintainability, and a smooth developer experience.

---

## 1. Commit Messages: Conventional Commits

- **All commits MUST use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) format.**
- Example:
  ```
  fix(puppet_module): handle invalid version string gracefully
  feat(cli): add --json output option
  docs(readme): update usage instructions
  ```
- Use the imperative mood (“add”, “fix”, “update”, not “added”, “fixes”, “updated”).
- Keep the subject line concise (max. 72 characters).
- Use a scope in parentheses to indicate the affected area (e.g., `puppet_module`, `ci`, `deps`).
- For breaking changes, add `!` after the type or scope and include a `BREAKING CHANGE:` footer.

---

## 2. Rust Project Best Practices

- **Formatting:**
  Always run `cargo fmt` before committing.

- **Linting:**
  Run `cargo clippy` and address all warnings where possible.

- **Testing:**
  Ensure all tests pass with `cargo test`.
  Add or update tests for new features and bug fixes.
  Prefer `Result` and meaningful error types over panics.
  Use `unwrap` only in tests or when absolutely safe.

- **Documentation:**
  Public functions, structs, and modules should have doc comments (`///`).

- **Dependencies:**
  Use the minimal required versions.
  Update dependencies only when necessary and use `[dev-dependencies]` for test-only crates.

- **Code Structure:**
  Organize code into modules.
  Keep functions short and focused.
  Avoid `unsafe` unless absolutely necessary and document all usages.

- **Error Handling:**
  Prefer returning `Result` over panicking or exiting the process.
  Handle errors gracefully and propagate them upwards.

- **CI/CD:**
  Ensure all GitHub Actions workflows pass before merging.
  Pre-commit hooks for formatting, linting, and commit message checks are enforced.

---

## 3. Pull Requests & Collaboration

- Reference related issues in the PR description.
- Keep PRs focused and small; split large changes into multiple PRs if possible.
- Ensure your branch is up to date with `main` before requesting review.
- Be open to feedback and ready to make changes.
- Keep documentation up to date as workflows and project structure evolve.

---

## 4. Automation & Tooling

- Automated PR labeling, changelog drafting, and contributor bots are in use.
- License and dependency checks are enforced via CI.
- Test coverage is measured and should be improved over time.
- Use the provided PR and issue templates.

---

## 5. Copilot-Specific Instructions

- **Always** generate commit messages in the Conventional Commits format.
- **Never** introduce code that ignores the above Rust best practices.
- **Do not** use `unwrap` or `expect` in production code unless it is absolutely safe and justified.
- **Prefer** robust error handling and clear, maintainable code.
- **Follow** the guidelines in `CONTRIBUTING.md` as if you were a human contributor.
- **If unsure**, prefer clarity, explicitness, and safety over cleverness or brevity.

---

Thank you for helping to keep this project modern, robust, and welcoming!
