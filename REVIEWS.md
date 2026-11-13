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

## 2026-11-10: Step 1.2 Remove and Drop (re-review)

- Scores: Correctness 7 / Tests 7 / Idiom 5 / API 7 / Understanding 7. Overall 6.6 (C+)
- Tooling: build ok; 17/17 tests (1M test now always on, 0.36s); clippy pedantic clean; fmt clean.
- Resolved: successor right-subtree bug (both regression shapes tested); 1M degenerate test built directly in O(n), red commit = test with Drop stashed; NOTES Q3 (LIFO end, O(n) front removal, d+1 vs (n+1)/2, degenerate peak 1).
- Must fix:
  1. 1M test builds an invalid tree: keys ascend toward the root along right links (right child < parent), 999_999 appears twice, and `len` is 0 for a million-node tree.
  2. Still no drop test with depth on both sides of a node.
  3. `remove`: `#[allow(clippy::missing_panics_doc)]` silences the question. Many expect/unwrap and an `unimplemented!()` remain. Restructure so they're unnecessary, or document the real panic conditions.
  4. NOTES Q5 unchanged in substance: "2^n" is wrong (4x per doubling = O(n^2)), no O(n log n) comparison, release-speedup explanation still not right.
- Outcome: **not passed**. Current step stays at 1.2.

## 2026-11-11: Step 1.2 Remove and Drop (re-review 2)

- Scores: Correctness 8 / Tests 8 / Idiom 8 / API 7 / Understanding 7. Overall 7.6 (B)
- Tooling: build ok; 18/18 tests (0.19s); clippy pedantic clean; fmt clean; all committed. Red/green commits for both drop tests (Drop stashed in red).
- Resolved: 1M test is a valid tree with len set; `remove` restructured (take the node, one match on the children pair, successor via slot cursor, two arms merged with `|`, single exit); `#[expect(..., reason)]` replaces the blanket allow; expect messages state invariants; Q5 now has O(n^2) and the compute-vs-memory-latency explanation.
- Must fix:
  1. `one_mil_both_sides_test`: the left subtree links by `right` with decreasing keys (right child < parent), the same invalid-tree problem fixed in the first 1M test.
  2. NOTES Q5: "6 orders of magnitude" is wrong. Redo n^2/2 vs n log2 n at n = 1M.
- Outcome: **not passed** (score is enough; two small must-fix items). Current step stays at 1.2.

## 2026-11-11: Step 1.2 Remove and Drop (final)

- Scores: Correctness 9 / Tests 8 / Idiom 8 / API 7 / Understanding 8. Overall 8.0 (B+)
- Tooling: 18/18 tests; clippy pedantic and fmt clean (checked in re-review 2; only the test tree and NOTES changed since).
- Resolved: both-sides drop test now links the left chain via `left` (valid BST); Q5 says ~4 orders of magnitude (n^2/2 = 5e11 vs n log2 n = 2e7, ~25,000x).
- Carry-over (non-blocking): Q3 recursion depth wording, Q1 tail-call reason, Q2 niche optimization, Q4 name the loan; `Drop` is_some()+expect pairs could be `if let`; "n^2 for a linear change" should say "per doubling". Commit the fixes.
- Outcome: **passed**. Current step moves to 1.3.

## 2026-11-11: Course re-plan (not a review)

- About 20h spent over 26 days; 3 of 31 steps done. Projection was 8–10 months at ~5h/week.
- Trimmed to a job-focused core: 2.1+2.2 merged, 3.2+3.3 merged, 5.x condensed, 1.5 reduced to insert/successor. Phase 1–5 renumbered (old 2.3→2.2, 2.4→2.3, 2.6→2.4, 3.4→3.3).
- Moved to "Later: extras": sealed traits, macro_rules, full benchmarks, docs/CI, proc macros. 1.6 unsafe kept.
- Review rules changed in CLAUDE.md: max 3 NOTES questions, narrow must-fix, one re-review max, auto-escalating hints, answer before probing.

## 2026-11-12: Policy change
- Commit hygiene (messages, red/green/refactor rhythm, history) is no longer reviewed or graded. All earlier commit-related carry-overs are dropped.

## 2026-11-13: Step 1.3 Iterators with lifetimes

- Scores: Correctness 9 / Tests 8 / Idiom 7 / API 7 / Understanding 8. Overall 7.8 (B+)
- Tooling: build ok; 32/32 tests (0.42s); clippy pedantic clean; fmt clean.
- Done: `Iter`, `IterMut` (split borrows: stack holds `(&K, &mut V, Option<&mut Node>)`), `IntoIter` with iterative `Drop`; `IntoIterator` for `&Bst`, `&mut Bst`, `Bst`; 1M drop tests (unconsumed/half-consumed, chain and both-sides); subtree-shape tests with key-derived values for all three iterators.
- Must fix: none.
- Carry-over (non-blocking): `'a` declared on the `impl Bst` block instead of per method/elided; `is_some()`+`unwrap()` in all three `left_spine`s (a `while let` over the Option removes it); `IterMut` tuple fields `.0/.1/.2` vs a named struct; iterator types live in a private module and aren't re-exported, so users can't name them; `new` on iterators is `pub`; duplicated 1M test builders in mod.rs and iter.rs.
- Outcome: **passed**. Current step moves to 1.4.
