# Review log

Newest entry at the bottom. Each entry: date, step, scores (Correctness / Tests / Idiom / API / Understanding), overall, must-fix items, outcome.



## 2026-10-17: Step 0.1 Skeleton

- Scores: Correctness 8 / Tests 5 / Idiom 6 / API 6 / Understanding 5. Overall 6.0 (C+)
- Tooling: build, test (0 tests), clippy pedantic and fmt all clean.
- Must fix:
  1. NOTES Q2 is the H2 hint pasted almost word for word. It doesn't answer "when do you want it", and the `tests/` experiment wasn't run. Rewrite it in your own words, include what the compiler told you, and decide whether `KvsError` gets `#[non_exhaustive]`, with your reasons.
  2. `KvsError::Placeholder` has no justification. Either remove it (and answer the empty-enum questions in NOTES) or justify it. The comment wrongly calls an empty enum "the never type".
- Outcome: **not passed**. Current step stays at 0.1.

## 2026-10-17: Step 0.1 Skeleton (re-review)

- Scores: Correctness 8 / Tests 6 / Idiom 8 / API 8 / Understanding 7. Overall 7.4 (B)
- Tooling: build, test (0 tests), clippy pedantic and fmt all clean.
- Earlier must-fix items resolved: Q2 is in the student's own words with the semver "when"; `Placeholder` removed; `KvsError` is empty and `#[non_exhaustive]`. The `tests/` experiment and the empty-enum notes were dropped as optional.
- Carry-over (non-blocking): explain why non_exhaustive on a variant protects the *author's* ability to add fields; use descriptive commit messages and red/green/refactor from 1.1.
- Outcome: **passed**. Current step moves to 1.1.

## 2026-10-19: Step 1.1 Owned BST

- Scores: Correctness 9 / Tests 7 / Idiom 6 / API 7 / Understanding 6. Overall 7.0 (B-)
- Tooling: build ok; 8/8 tests pass; fmt clean; clippy pedantic **6 warnings** (question_mark in `get`, 2x bool_assert_comparison, useless_vec, semicolon_if_nothing_returned).
- Must fix:
  1. Get clippy pedantic clean, including the `?` suggestion: work out why the naive version doesn't compile.
  2. `Node.value: Option<V>` lets a valueless node exist. Make that state impossible.
  3. NOTES Q3: give a clear verdict (checker too conservative vs code wrong), mention Polonius, and fix the inaccuracies (the Equal-arm problem was break-then-use-after-loop, not "mutating"; it was a reborrow via as_mut, not a "reborrow again").
  4. Commit the outstanding work.
- Outcome: **not passed**. Current step stays at 1.1.
