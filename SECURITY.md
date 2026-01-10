# Security Policy

## Supported Versions

| Version | Supported |
|----------|------------|
| 0.1.x | ✅ Yes |
| < 0.1.0 | ❌ No |

**Note**: Only the latest minor version (0.1.x) receives security updates. Older versions are not supported.

## Reporting a Vulnerability

### Private Disclosure Process

We take security vulnerabilities seriously. If you discover a security vulnerability, please report it privately.

#### How to Report

**Email**: security@openISL.dev

**Subject**: Security Vulnerability Report - [Brief Description]

**Include in Report**:
- Description of the vulnerability
- Steps to reproduce
- Affected versions (or commit hash)
- Potential impact
- Suggested fix (if known)

**PGP Key**: Available at `https://github.com/bharat94/openISL/security/pgp-key`

### Response Timeline

We follow [OWASP Vulnerability Disclosure](https://cheatsheetseries.owasp.org/cheatsheets/Vulnerability_Disclosure_Cheat_Sheet.html) guidelines:

| Stage | Timeline |
|--------|----------|
| Acknowledgment | Within 48 hours |
| Initial Assessment | Within 7 business days |
| Fix Verification | Within 14 business days (depends on severity) |
| Public Disclosure | After fix is released |

### Severity Assessment

We use [CVSS v3.1](https://www.first.org/cvss/calculator/3.1) for severity classification:

| Severity | Response Timeline |
|----------|------------------|
| Critical (9.0-10.0) | 48-72 hours |
| High (7.0-8.9) | 7 days |
| Medium (4.0-6.9) | 14 days |
| Low (0.1-3.9) | Next minor release |

### Safe Harbor

**Good faith vulnerability reports**:
- Will not result in legal action against researchers
- May be eligible for acknowledgment in CHANGELOG.md
- May be eligible for bug bounty (if program exists)

**Requirements**:
- Give us reasonable time to respond before public disclosure
- Don't exploit the vulnerability beyond what's needed for verification
- Don't access, delete, or modify user data during testing

## Security Best Practices

### For Users

#### Installation
- Download from official sources only:
  - GitHub releases: https://github.com/bharat94/openISL/releases
  - crates.io: `cargo install openisl`
- Verify signature of binaries (when available)
- Use package managers with provenance tracking

#### Runtime Security
- Run with least necessary permissions
- Don't run as root/administrator unless required
- Keep openISL updated to latest version
- Review dependencies before accepting updates

#### Secrets Management
- openISL does not store or transmit API keys or secrets
- Configuration files (`.env`, `config.toml`) should have restricted permissions
- Don't commit secrets to repositories
- Use environment variables for sensitive data

### For Developers

#### Development Security
- Enable all linters: `cargo clippy --all-features`
- Run security scanning: `cargo audit`
- Use dependency pinning for critical releases
- Follow [OWASP Secure Coding](https://cheatsheetseries.owasp.org/) guidelines

#### Dependency Management
- Regular updates: `cargo update` weekly for direct dependencies
- Vulnerability scanning: Use GitHub Dependabot or similar
- Review dependencies before adding: Check maintainer, activity, security history
- Use private crates when public ones have known vulnerabilities

#### Secrets in Code
- Never commit secrets: API keys, tokens, passwords
- Use environment variables for credentials
- Rotate compromised credentials immediately
- Check for secrets in CI/CD logs

#### CI/CD Security
- Use encrypted secrets in workflows
- Require 2FA for maintainers with write access
- Sign all artifacts: Enable GitHub artifact signing
- Use protected branches and require PR reviews

## Supply Chain Security

### SLSA Compliance

openISL implements [SLSA Level 1](https://slsa.dev/) (Supply-chain Levels for Software Artifacts):

#### Build Verification
- **Reproducible builds**: Same source → same binary
- **Source provenance**: All artifacts traceable to source commit
- **Signed artifacts**: Release binaries cryptographically signed
- **Dependency verification**: Verify checksums of dependencies

#### User Verification

Users can verify:
1. Downloaded file matches published checksum
2. Binary signature is valid (when signing enabled)
3. Source code in release matches repository state

### Dependency Policy

We follow these principles:
- **Minimal dependencies**: Use only necessary crates
- **Well-maintained dependencies**: Prefer active, secure crates
- **Regular audits**: Review dependencies monthly for CVEs
- **Transparent manifest**: `Cargo.lock` committed for reproducibility

## Security Features

### Built-in Protections

#### Git Operations Safety
- Confirmation prompts for destructive operations (force push, branch deletion)
- Dry-run mode for testing commands: `openisl save --dry-run`
- Display of affected files before operations: `openisl commit --preview`

#### File System Safety
- Read-only mode for analysis: `openisl stack --read-only`
- Path sanitization: Prevent directory traversal
- File permission checks: Verify before modifications

#### Network Safety
- No automatic outbound connections
- Explicit user opt-in for network operations
- Display network activity when active: `--verbose` shows network calls

## Security Monitoring

### Continuous Monitoring

We monitor for:
- **GitHub Security Advisories**: Automated alerts for dependencies
- **Crates.io security advisories**: Dependency vulnerability notifications
- **Common Vulnerabilities and Exposures (CVE)**: Track reported issues

### Incident Response

For security incidents:
1. **Immediate Assessment**: Evaluate severity and impact
2. **Internal Coordination**: Assemble response team
3. **Patch Development**: Prioritize security fixes
4. **Coordinated Disclosure**: Notify users with clear guidance
5. **Post-Incident Review**: Document lessons learned

## Transparency

### Public Communication

We communicate about security:
- **Advisories**: GitHub Security Advisories for critical issues
- **CHANGELOG.md**: Document all security fixes
- **Release notes**: Highlight security updates
- **Blog posts**: For significant security improvements

### Update Policy

- Critical/High: Immediate security releases (x.x.x-critical.1)
- Medium: Next minor release (x.x.x) with security fixes
- Low: Next patch release (x.x.x) with security fixes

## Related Documents

- [Code of Conduct](CODE_OF_CONDUCT.md) - Safe community standards
- [Contributing Guidelines](CONTRIBUTING.md) - Security-focused development
- [Open Source Standards](OPEN_SOURCE_STANDARDS.md) - Security compliance
- [Governance](GOVERNANCE.md) - Security governance model

## Contact

### Security Questions

**Email**: security@openISL.dev
**Response Time**: Within 48 hours

### Non-Security Issues

For non-security bugs or feature requests, please use:
- GitHub Issues: https://github.com/bharat94/openISL/issues
- Issue Templates: Use appropriate template in `templates/issue-templates/`

---

**Commitment**: We are committed to maintaining the security and integrity of openISL for all our users.

**Last Updated**: 2026-01-09
