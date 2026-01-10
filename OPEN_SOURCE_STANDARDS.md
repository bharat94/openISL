# Open Source Standards

This document outlines the open source standards, best practices, and frameworks that openISL follows. These standards guide our development, documentation, community management, and release processes.

## Table of Contents

- [Overview](#overview)
- [Documentation Standards](#documentation-standards)
- [Code & Contribution Standards](#code--contribution-standards)
- [Security Standards](#security-standards)
- [Community & Governance](#community--governance)
- [Versioning & Release](#versioning--release)
- [Testing & Quality](#testing--quality)
- [CLI/TUI Standards](#clitui-standards)
- [References](#references)

## Overview

openISL is committed to following industry-recognized open source standards to ensure:
- **High quality code** and practices
- **Clear, accessible documentation**
- **Welcoming and inclusive community**
- **Secure and stable releases**
- **Transparent governance**

These standards evolve as best practices in the open source community mature.

## Documentation Standards

### Diátaxis Framework

We adopt the [Diátaxis Framework](https://diataxis.fr/) for structuring our documentation. Diátaxis categorizes documentation into four types:

#### 1. Tutorials
- **Purpose**: Step-by-step learning paths from zero to working knowledge
- **Audience**: New users learning the tool
- **Examples**:
  - [Getting Started Guide](docs/tutorials/getting-started.md)
  - [First Stack Analysis](docs/tutorials/first-analysis.md)

#### 2. How-to Guides
- **Purpose**: Task-focused instructions for specific goals
- **Audience**: Users who want to accomplish specific tasks
- **Examples**:
  - [Configuring Stack Detection](docs/how-to-guides/configuring-detection.md)
  - [Setting up CI Integration](docs/how-to-guides/ci-integration.md)

#### 3. Reference
- **Purpose**: Complete, authoritative information
- **Audience**: Experienced users who need quick lookup
- **Examples**:
  - [CLI Command Reference](docs/cli-commands/)
  - [Configuration Options](docs/reference/configuration.md)
  - [TUI Component API](docs/tui-reference/)

#### 4. Explanation
- **Purpose**: Deep dives into concepts and context
- **Audience**: Users wanting to understand "why" and "how it works"
- **Examples**:
  - [Architecture Overview](docs/explanation/architecture.md)
  - [Stack Detection Algorithm](docs/explanation/detection-algorithm.md)

### Technical Writing Standards

We follow [Google's Technical Writing](https://developers.google.com/tech-writing/) guidelines:
- **Clear, concise language** - Avoid jargon when possible
- **Audience-first approach** - Write for the user, not the developer
- **Concrete examples** - Show, don't just tell
- **Inclusive language** - Use gender-neutral, welcoming terms
- **Accessibility** - Follow web accessibility standards (WCAG 2.1)

### Documentation Quality Standards

Following [The Good Docs Project](https://www.thegooddocsproject.dev/) templates:
- **Clear structure** - Use consistent formatting and headings
- **Code examples** - All examples tested and documented
- **Links work** - Verify all internal and external links
- **Up-to-date** - Docs reviewed with each release
- **Searchable** - Use clear titles and keywords

## Code & Contribution Standards

### Conventional Commits

All commits follow [Conventional Commits 1.0.0](https://www.conventionalcommits.org/) specification.

#### Commit Format
```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

#### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, semicolons, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `ci`: CI/CD changes

#### Scopes
Common scopes: `cli`, `tui`, `stack`, `git`, `docs`, `tests`

#### Breaking Changes
Add `!` after type: `feat(api)!: remove deprecated endpoints`

#### Examples
```bash
git commit -m "feat(stack): add Python 3.12 support"
git commit -m "fix(cli): resolve branch detection in monorepos"
git commit -m "docs(tui): update installation guide for Windows"
git commit -m "feat(git)!: simplify save command interface"
```

### Code Style Guidelines

#### Rust Guidelines
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Address `cargo clippy` warnings
- Prefer idiomatic Rust patterns
- Document public APIs with `///` comments

#### General Standards
- **Meaningful names** - Functions, variables, modules use descriptive names
- **Small functions** - Functions do one thing well
- **Error handling** - Use `Result<T, E>` for recoverable errors
- **Tests** - Write tests for all new functionality
- **Documentation** - Document non-obvious code

### Code Review Standards

Following [Microsoft's Pull Request Guidelines](https://microsoft.github.io/code-with-engineering-playbook/code-reviews/pull-requests/):

#### Review Focus
- Correctness and logic
- Code quality and style
- Test coverage
- Documentation completeness
- Security implications

#### Review Process
1. Automated checks pass (tests, linting, formatting)
2. At least one maintainer review
3. Address all feedback
4. Approval required for merge
5. Tests must pass on all branches

## Security Standards

### OpenSSF Security Baseline

We follow the [OpenSSF Security Baseline](https://baseline.openssf.org/) controls for open source projects.

#### Maturity Levels

**Current Level: Bronze**

**Planned Level: Silver**

#### Security Controls
- **Dependency Management**: Regular dependency updates and vulnerability scanning
- **Supply Chain Security**: SLSA Level 1 compliance
- **Vulnerability Disclosure**: Coordinated disclosure process
- **Secure Development**: Code review, testing, secrets management

### Vulnerability Disclosure

We follow [OWASP Vulnerability Disclosure](https://cheatsheetseries.owasp.org/cheatsheets/Vulnerability_Disclosure_Cheat_Sheet.html) guidelines.

#### Reporting Vulnerabilities
- Email: security@openISL.dev
- Include clear description and reproduction steps
- Allow reasonable time for response (up to 90 days)
- Do not disclose publicly until coordinated

#### Response Timeline
- Acknowledge within 48 hours
- Initial assessment within 7 days
- Fix timeline depends on severity (critical: 7 days, high: 14 days, etc.)
- Public disclosure after fix is released

### Supply Chain Security

Following [SLSA Framework](https://slsa.dev/) principles:
- **Reproducible builds**: Ensure consistent builds across environments
- **Signed artifacts**: Use cryptographic signatures for releases
- **Artifact verification**: Allow users to verify downloaded files
- **Dependency provenance**: Track and verify dependency sources

### Dependency Management

- Regular updates (weekly for direct dependencies)
- Security scanning in CI/CD
- Use of private registries when applicable
- Audit dependencies before adding new ones

## Community & Governance

### Code of Conduct

We adopt [Contributor Covenant 2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct.html):
- Welcoming, inclusive community
- Clear standards of behavior
- Enforcement procedures
- Contact for reporting issues

See [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for full details.

### Governance Model

Following [Open Source Initiative Governance](https://opensource.org/blog/what-is-open-governance-drafting-a-charter-for-an-open-source-project):

#### Leadership Roles
- **Project Maintainer**: Overall technical leadership, release decisions
- **Maintainer**: Module ownership, code review authority
- **Contributor**: Active participation, can become maintainer
- **Community Member**: Participation in discussions and issues

#### Decision Making
- **Proposal Process**: RFC discussions for major changes
- **Lazy Consensus**: Inaction equals consent
- **Final Authority**: Maintainers have final say for technical decisions
- **Transparency**: All decisions documented in governance discussions

#### Role Progression
Contributors can become maintainers based on:
- Sustained, quality contributions
- Code review activity
- Community engagement
- Mentoring other contributors

### Recognition

Contributors are recognized in:
- [CONTRIBUTORS.md](CONTRIBUTORS.md) - List of all contributors
- [CHANGELOG.md](CHANGELOG.md) - Credit in release notes
- GitHub releases - Highlighted contributions
- Annual contributor awards (planned)

## Versioning & Release

### Semantic Versioning

We follow [Semantic Versioning 2.0.0](https://semver.org/) (SemVer):

#### Format
```
MAJOR.MINOR.PATCH

Examples: 0.1.0, 1.2.3, 2.0.0
```

#### Version Number Rules
- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality additions
- **PATCH**: Backwards-compatible bug fixes

#### Pre-releases
- **Alpha**: 0.1.0-alpha.1 (early development)
- **Beta**: 0.1.0-beta.1 (feature complete, testing)
- **RC**: 0.1.0-rc.1 (release candidate, final testing)

### Changelog Standards

Following [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format:

#### CHANGELOG.md Format
```markdown
# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Feature additions not yet released

### Changed
- Modifications to existing features

### Deprecated
- Features marked for future removal

### Removed
- Features removed in this version

### Fixed
- Bug fixes

### Security
- Security vulnerability fixes
```

#### Principles
- **Human-written**: Not auto-generated from git log
- **User-focused**: Only include user-facing changes
- **Grouped by type**: Added, Changed, Deprecated, Removed, Fixed, Security
- **Clear descriptions**: Explain what changed, not technical details

### Release Process

1. **Prepare Release**:
   - All tests passing
   - Documentation updated
   - CHANGELOG.md updated
   - Version number updated in Cargo.toml

2. **Pre-release**:
   - Tag version: `git tag -a v0.1.0 -m "Release 0.1.0"`
   - Push tag: `git push origin v0.1.0`

3. **Build & Test**:
   - Build release binaries: `cargo build --release`
   - Run integration tests
   - Verify against sample repositories

4. **Publish**:
   - Publish to crates.io: `cargo publish`
   - Create GitHub release with notes from CHANGELOG.md
   - Announce on community channels

5. **Post-release**:
   - Update documentation
   - Close related issues
   - Notify community

## Testing & Quality

### Test Coverage Standards

- **Minimum Coverage**: 80% line coverage for critical paths
- **Test Types**:
  - Unit tests: Individual function/component tests
  - Integration tests: Component interaction tests
  - End-to-end tests: Complete workflow tests

### Quality Gates

All changes must pass:
1. **Static Analysis**: `cargo clippy` - no warnings
2. **Formatting**: `cargo fmt --check` - consistent formatting
3. **Unit Tests**: `cargo test` - all tests pass
4. **Integration Tests**: Test against sample repositories
5. **Documentation Builds**: All docs compile without errors

### CI/CD Standards

Following [OWASP Secure Pipeline Verification](https://owasp.org/www-project-spvs/):

#### Pipeline Stages
1. **Lint**: Static analysis and formatting checks
2. **Test**: Unit and integration tests
3. **Build**: Compile release binaries
4. **Security**: Dependency vulnerability scanning
5. **Package**: Create installable artifacts

#### Environment Standards
- **Reproducible**: Same commits, same results
- **Secure**: No secrets in logs or artifacts
- **Fast**: Parallel execution where possible

## CLI/TUI Standards

### CLI Design Principles

Following [CLI Guidelines](https://clig.dev/) and [BetterCLI.org](https://bettercli.org/):

#### Core Principles
- **Do one thing well**: Each command has a single, clear purpose
- **Discoverable**: Users can find commands without documentation
- **Consistent**: Similar operations use similar patterns
- **Forgiving**: Provide helpful errors, not crashes
- **Efficient**: Fast, minimal output for common operations

#### Command Design
- **Verb-first naming**: `openisl stack`, `openisl branch`, `openisl save`
- **Help system**: `--help` flag on all commands
- **Verbose flag**: `--verbose` for detailed output
- **Progress indication**: Show progress for long operations
- **Confirmation prompts**: Ask before destructive operations

### TUI Design Standards

Following modern terminal UI patterns:

#### User Interface
- **Keyboard-first navigation**: Efficient keyboard shortcuts
- **Visual clarity**: Clean layout, meaningful colors
- **Screen reader support**: Accessible via text-only mode
- **Responsive**: Adapt to terminal size changes

#### Interactions
- **Immediate feedback**: Show results of actions
- **Undo support**: Where possible, allow undoing operations
- **Helpful errors**: Explain what went wrong and how to fix
- **Context awareness**: Show relevant information based on current state

#### Accessibility
- **Color blind safe**: Use distinct patterns, not just colors
- **Text-only mode**: Support `--no-color` flag
- **Resize handling**: Gracefully adapt to terminal size
- **Unicode support**: Handle emojis and special characters correctly

## Continuous Improvement

This document is reviewed and updated quarterly. Changes proposed and discussed in:
- [Issue tracker](https://github.com/bharat94/openISL/issues)
- [Community meetings](https://github.com/bharat94/openISL/discussions)
- [Governance discussions](GOVERNANCE.md)

## References

### Documentation Frameworks
- [Diátaxis Framework](https://diataxis.fr/)
- [The Good Docs Project](https://www.thegooddocsproject.dev/)
- [Google Technical Writing](https://developers.google.com/tech-writing/)
- [MDN Writing Style](https://developer.mozilla.org/en-US/docs/MDN/Writing_guidelines/Writing_style_guide)

### Open Source Standards
- [Open Source Guides](https://opensource.guide/)
- [OpenSSF Security Baseline](https://baseline.openssf.org/)
- [Contributor Covenant](https://www.contributor-covenant.org/)
- [Open Governance](https://opensource.org/blog/what-is-open-governance-drafting-a-charter-for-an-open-source-project)

### Versioning & Release
- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)

### Security
- [OWASP Security Practices](https://cheatsheetseries.owasp.org/)
- [SLSA Framework](https://slsa.dev/)
- [GitHub Security Best Practices](https://docs.github.com/en/code-security/getting-started/securing-your-repository)

### CLI/TUI Design
- [CLI Guidelines](https://clig.dev/)
- [BetterCLI.org](https://bettercli.org/)
- [Atlassian CLI Principles](https://www.atlassian.com/blog/it-teams/10-design-principles-for-delightful-clis)
- [Textual TUI Framework](https://textualize.io/blog/7-things-ive-learned-building-a-modern-tui-framework/)

### Project Templates
- [CNCF Project Template](https://github.com/cncf/project-template)
- [Open Containers Project Template](https://github.com/opencontainers/project-template)
- [GitHub Issue/PR Templates](https://docs.github.com/en/communities/using-templates-to-encourage-useful-issues-and-pull-requests/)

---

This standards document ensures openISL maintains high quality, security, and community best practices as an open source project.

**Last Updated**: 2026-01-09
