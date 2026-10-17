# Review log

Newest entry at the bottom. Each entry: date, step, scores (Correctness / Tests / Idiom / API / Understanding), overall, must-fix items, outcome.



## 2026-10-17: Step 0.1 Skeleton

- Scores: Correctness 8 / Tests 5 / Idiom 6 / API 6 / Understanding 5. Overall 6.0 (C+)
- Tooling: build, test (0 tests), clippy pedantic and fmt all clean.
- Must fix:
  1. NOTES Q2 is the H2 hint pasted almost word for word. It doesn't answer "when do you want it", and the `tests/` experiment wasn't run. Rewrite it in your own words, include what the compiler told you, and decide whether `KvsError` gets `#[non_exhaustive]`, with your reasons.
  2. `KvsError::Placeholder` has no justification. Either remove it (and answer the empty-enum questions in NOTES) or justify it. The comment wrongly calls an empty enum "the never type".
- Outcome: **not passed**. Current step stays at 0.1.
