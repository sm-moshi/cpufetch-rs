---
name: clippy
description: Run cargo clippy with pedantic warnings as errors and fix any issues found
disable-model-invocation: true
---

Run `cargo +beta clippy --all-targets --all-features -- -D warnings`.

If there are warnings or errors:
1. Read each diagnostic carefully
2. Fix the issues in the source files
3. Re-run clippy to confirm all issues are resolved

Do not suppress warnings with `#[allow(...)]` unless there is a clear justification documented in a comment.
