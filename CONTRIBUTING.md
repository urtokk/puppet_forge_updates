# Contributing Guidelines

Thank you for considering contributing to this project! To ensure a smooth workflow and maintain high code quality, please follow these guidelines.

---

## Conventional Commit Message Rules

All commits **must** follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

### Commit Message Format

```
<type>(<scope>): <short description>

[optional body]

[optional footer(s)]
```

#### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools and libraries such as documentation generation

#### Scope

- The scope is optional but recommended. It should be a noun describing the section of the codebase affected (e.g., `puppet_module`, `ci`, `deps`).

#### Description

- Use the imperative mood (“change” not “changed” or “changes”).
- Keep it concise (max. 72 characters).

#### Body

- Use the body to explain **what** and **why** vs. **how**.
- Wrap lines at 72 characters.

#### Footer

- Use for breaking changes and issues references.
- Breaking changes must start with `BREAKING CHANGE:`.

#### Examples

```
feat(puppet_module): add integration test for live Forge API

fix: handle empty version string gracefully

docs(readme): update usage instructions

chore(deps): update reqwest to 0.11

refactor: simplify version parsing logic

feat!: change Version struct fields to public

BREAKING CHANGE: Version struct fields are now public API.
```

---

## Rust Development Guidelines

- **Formatting**: Always run `cargo fmt` before committing.
- **Linting**: Run `cargo clippy` and address warnings where possible.
- **Testing**: Ensure all tests pass with `cargo test`. Add tests for new features and bug fixes.
- **Documentation**: Public functions, structs, and modules should have doc comments (`///`).
- **Error Handling**: Prefer `Result` and meaningful error types over panics. Use `unwrap` only in tests or when absolutely safe.
- **Dependencies**: Use the minimal required versions. Update dependencies only when necessary.
- **Code Structure**: Organize code into modules. Keep functions short and focused.
- **Unsafe Code**: Avoid `unsafe` unless absolutely necessary. Document all usages.
- **CI/CD**: If a CI pipeline exists, ensure it passes before merging.

---

## Pull Requests

- Reference related issues in the PR description.
- Keep PRs focused and small; split large changes into multiple PRs if possible.
- Ensure your branch is up to date with `main` before requesting review.
- Be open to feedback and ready to make changes.

---

Thank you for helping to make this project better!
