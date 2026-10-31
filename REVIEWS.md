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

## 2026-10-20: Step 1.1 Owned BST (re-review)

- Scores: Correctness 9 / Tests 8 / Idiom 8 / API 8 / Understanding 6. Overall 7.8 (B)
- Tooling: build ok; 8/8 tests; clippy pedantic clean; fmt clean. All committed.
- Resolved: clippy (`as_ref()?`), `value: V` + `mem::replace`, bounds moved to impls, verdict added to Q3, `for` loops in tests.
- Must fix (NOTES only):
  1. Q3 still has the "reborrow again" sentence and no Polonius mention; the stale clippy NOTE is still there.
  2. Q2 says PartialOrd "does not have an Equals value", which is wrong: `partial_cmp` returns `Option<Ordering>`. Also explain why `Eq` isn't a separate bound.
- Outcome: **not passed**, pending the NOTES fixes. The code is approved as is.

## 2026-10-20: Step 1.1 Owned BST (final)

- Scores: Correctness 9 / Tests 8 / Idiom 8 / API 8 / Understanding 8. Overall 8.2 (B+)
- Tooling: 8/8 tests, clippy pedantic clean, fmt clean.
- NOTES Q2 and Q3 fixed (Option<Ordering> and incomparable keys, Eq as a supertrait, a single loan before the branch, Polonius).
- Carry-over (non-blocking): redundant `as_ref()` in `get`, comments that restate code, no docs on public items. Commit the NOTES change.
- Outcome: **passed**. Current step moves to 1.2.

## 2026-10-22: Correction to the step 1.1 carry-over

- Retracted: "redundant `as_ref()` in `get`". It is required. `?` needs an owned `Option<T>`, and `cur` is `&Option<Box<Node>>`. `as_ref()` produces `Option<&Box<Node>>` without moving out of the borrow. The student pointed this out correctly.

## 2026-10-31: Step 1.2 Remove and Drop

- Scores: Correctness 5 / Tests 6 / Idiom 5 / API 7 / Understanding 6. Overall 5.8 (C)
- Tooling: build ok; 14 passed, 1 ignored (`one_mil_test`); clippy pedantic clean; fmt clean. NOTES.md uncommitted.
- Verified bug (scratch probe outside crates/): removing a two-children node loses the successor's right subtree. Inserts [5,3,8,9], remove 5: 9 is lost. Inserts [5,3,10,7,8], remove 5: 8 is lost. `len` still reports the old count minus one.
- Must fix:
  1. Successor's right child dropped in the two-children case; add red tests for both shapes (successor is the direct right child / successor deep with a right child).
  2. Iterative drop is unprotected: the only drop test is `#[ignore]`d. Need an always-on test that crashes without the `Drop` impl (stash experiment), plus a drop test with depth on both sides of a node.
  3. `# Panics` doc on `remove` says it never panics; that's not what the section is for. Either it can panic (document when) or it can't (restructure so the expects go away, or drop the section honestly).
  4. NOTES Q3 is wrong: `Vec::pop` takes from the end; size analysis is confused. Redo: max stack size for degenerate vs balanced vs recursion depth.
  5. NOTES Q5: "2^n" is wrong, your own data is 4x per doubling = O(n^2). Add the O(n log n) comparison and the real release timing explanation (pointer chasing / cache misses).
  6. Commit NOTES; no more "green something" messages.
- Outcome: **not passed**. Current step stays at 1.2.
