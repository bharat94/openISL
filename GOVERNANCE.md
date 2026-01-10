# Governance Model

This document describes how the openISL project is governed, how decisions are made, and how the community is structured.

## Table of Contents

- [Overview](#overview)
- [Leadership Roles](#leadership-roles)
- [Decision Making](#decision-making)
- [Contribution Recognition](#contribution-recognition)
- [Conflict Resolution](#conflict-resolution)
- [Community Management](#community-management)
- [Changes to Governance](#changes-to-governance)

## Overview

openISL follows an **open governance model** inspired by [Open Source Initiative's Open Governance](https://opensource.org/blog/what-is-open-governance-drafting-a-charter-for-an-open-source-project) and [Apache Project Maturity Model](https://community.apache.org/apache-way/apache-project-maturity-model.html).

### Principles

- **Openness**: All discussions happen in public (GitHub Issues, Discussions)
- **Transparency**: Decisions and their rationale are documented
- **Meritocracy**: Contributions and decisions are based on merit
- **Inclusivity**: Welcoming to all contributors, regardless of background
- **Community-driven**: Project serves its users and contributors

## Leadership Roles

### Project Maintainer

**Responsibilities**:
- Overall technical leadership and direction
- Final authority on technical decisions
- Release management and versioning
- Security incident response coordination
- Maintainer recruitment and onboarding

**Current Maintainer**: [bharat94](https://github.com/bharat94)

### Module Maintainers

**Responsibilities**:
- Own specific modules (cli, tui, stack, git)
- Review and merge PRs in their domain
- Guide contributors in their area
- Ensure code quality and standards adherence

**Current Module Maintainers**:
- CLI: TBD
- TUI: TBD
- Stack: TBD
- Git: TBD

### Contributors

**Responsibilities**:
- Active participation in issues and PRs
- Code review participation
- Documentation improvements
- Testing and bug reporting
- Community support

**Becoming a Maintainer**:
Contributors can become maintainers through:
- Sustained, quality contributions (6+ months)
- Successful mentorship of new contributors
- Active code review and participation
- Community endorsement

## Decision Making

### Proposal Process

For significant changes, we use an RFC (Request for Comments) process:

1. **RFC Creation**:
   - Create issue with label `rfc`
   - Include: Problem, Proposed Solution, Alternatives, Impact
   - Allow at least 7 days for feedback

2. **Discussion Period**:
   - Gather feedback in GitHub Issues and Discussions
   - Encourage diverse perspectives
   - Maintain constructive, focused dialogue

3. **Decision**:
   - Maintainers summarize feedback
   - Document decision and rationale
   - Announce outcome with clear explanation

### Lazy Consensus

For day-to-day decisions:

- **Silent = Consent**: If no objections within 7 business days, proposal is accepted
- **Explicit Objection**: Must provide rationale for blocking
- **Maintainer Authority**: Maintainers can decide to move forward or reconsider

### Types of Decisions

| Decision Type | Process |
|--------------|---------|
| Trivial/Obvious | Maintainer decision, document after |
| Standard | Lazy consensus (7-day review) |
| Significant | RFC process (minimum 14-day discussion) |
| Architectural | RFC + documented in ARCHITECTURE.md |
| Breaking Changes | RFC + 21-day minimum discussion |

## Contribution Recognition

### Contribution Credit

Contributors are credited in:
- [CONTRIBUTORS.md](CONTRIBUTORS.md) - All-time contributors list
- [CHANGELOG.md](CHANGELOG.md) - Release-specific credits
- GitHub release notes - Highlighted contributions

### Types of Recognition

#### Code Contributions
- PRs merged into main branch
- Bug fixes and features
- Documentation and tests
- Code reviews

#### Non-Code Contributions
- Issue triage and management
- Community support and mentorship
- Documentation writing
- Design contributions (logos, branding)
- Translation and localization
- Security research and vulnerability reports

### Community Awards

We plan to introduce (as community grows):
- **Quarterly MVP**: Most valuable contributions per quarter
- **Annual Awards**: Recognize outstanding contributors yearly
- **Mentor of the Year**: Exceptional community guidance

## Conflict Resolution

### Disagreement Process

1. **Direct Resolution**: Parties attempt to resolve directly
2. **Mediation**: Ask neutral community member to mediate
3. **Maintainer Escalation**: Escalate to maintainers for decision
4. **Community Input**: If unresolved, seek broader community feedback

### Code of Conduct Enforcement

For CoC violations:
- Report to `conduct@openISL.dev` or [GitHub Issue](https://github.com/bharat94/openISL/issues/new?template=conduct_report)
- Initial response within 48 hours
- Investigation within 7 days
- Document outcome while protecting privacy

### Technical Disagreements

When technical disagreements occur:
- Focus on technical merits, not personalities
- Use evidence, benchmarks, or user feedback
- Consider maintainability and long-term impact
- Accept maintainer final decision for progress

## Community Management

### Communication Channels

| Channel | Purpose | Audience |
|----------|---------|----------|
| GitHub Issues | Bug reports, feature requests | Public |
| GitHub Discussions | Questions, RFC discussions | Public |
| Discord (planned) | Real-time chat, community building | Public |
| Mailing List (planned) | Announcements, RFC discussions | Public |

### Meeting Rhythm (Planned)

As community grows, we plan:
- **Monthly Community Calls**: Open to all contributors
- **Quarterly Planning**: Roadmap and strategy
- **Annual State of Project**: Yearly review and planning

### Documentation of Governance

This governance model is:
- **Publicly accessible**: This document and all discussions are open
- **Version controlled**: Changes to governance are tracked in git
- **Reviewable**: Community can propose and discuss changes
- **Living document**: Updated as project matures

## Changes to Governance

### Amending Governance

Changes to this governance model require:
- **RFC Proposal**: Open discussion in GitHub issue with `rfc` and `governance` labels
- **Supermajority**: 2/3 of maintainers + majority support from contributors
- **Notice Period**: 30 days for community feedback
- **Documenting**: Update this document with clear rationale

### Trigger Events

Governance review is triggered by:
- Annual review cycle
- Significant community growth (>50 active contributors)
- Major incidents or disputes
- Community-initiated proposals

## Related Documents

- [Code of Conduct](CODE_OF_CONDUCT.md) - Community behavior standards
- [Contributing Guidelines](CONTRIBUTING.md) - How to contribute
- [Open Source Standards](OPEN_SOURCE_STANDARDS.md) - Technical and community standards
- [Security Policy](SECURITY.md) - Security practices and reporting

## Transparency Commitment

We commit to:
- Open discussions for all significant decisions
- Publicly documented rationale for decisions
- Regular community feedback sessions
- Clear communication of changes and policies

---

**Version**: 1.0
**Last Updated**: 2026-01-09
**Next Review**: 2026-07-09 (6 months)

This governance model ensures openISL remains transparent, inclusive, and community-driven.
