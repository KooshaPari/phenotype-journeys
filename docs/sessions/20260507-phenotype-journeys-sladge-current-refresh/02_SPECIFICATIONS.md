# Specifications

## Acceptance Criteria

- Current-head README contains the Sladge badge.
- Session docs record the superseded stale branch and current evidence.
- Validation covers diff hygiene, badge presence, and repo-local task gates or
  records exact blockers.
- Canonical checkout remains unchanged unless full integration is safe.

## Assumptions, Risks, Uncertainties

- Assumption: current local `chore/trufflehog-20260502` is the right evidence
  base because it is ahead of origin by three commits.
- Risk: stale `46bfc8c` evidence could be mistaken as current.
- Mitigation: record the new worktree, branch, and commit in downstream and
  projects-landing ledgers.
