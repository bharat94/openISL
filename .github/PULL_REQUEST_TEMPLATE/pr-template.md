---
name: Pull Request Template
about: "PR template for contributing to openISL"
title: "[<type>]: "
labels: []
assignees: []
---

## Type of Change

Please check the type of change your PR introduces:

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update (changes to documentation only)
- [ ] Performance improvement (non-breaking change which improves performance)
- [ ] Code refactoring (non-breaking change which improves code structure without changing functionality)

## Related Issue

Fixes #<issue_number> (or `None` if not related)

## Description

Describe your changes in detail:

## Motivation and Context

Why is this change required? What problem does it solve?

## How Has This Been Tested?

Please describe the tests that you ran to verify your changes:

- [ ] Unit tests added/updated and passing
- [ ] Integration tests added/updated and passing
- [ ] Manual testing performed (describe what you tested)
- [ ] Existing tests still pass

## Checklist

Before submitting your PR, please ensure:

- [ ] Code follows project [style guidelines](CONTRIBUTING.md)
- [ ] Self-review completed (reviewed your own code)
- [ ] All tests pass locally (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated (if applicable)
- [ ] CHANGELOG.md updated (if user-facing change)
- [ ] Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/)
- [ ] PR title follows conventional commits format

## Breaking Changes

If this PR includes breaking changes, please describe them:

## Additional Notes

Any additional context or notes for reviewers:

---

Thank you for contributing to openISL! 🚀
