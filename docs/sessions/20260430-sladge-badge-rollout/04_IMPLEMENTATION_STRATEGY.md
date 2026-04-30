# Implementation Strategy

- Keep the project change documentation-only.
- Preserve the dirty canonical checkout by using the required worktree path.
- Attempt the Rust quality gates from the isolated worktree and record any
  unrelated blocker instead of broadening the badge change.
