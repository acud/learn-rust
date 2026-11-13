# Rust to senior level: syllabus

**Learner profile:** coming from Go. Has written ownership/borrowing code, traits and generics. Has done some real tokio work (spawn, channels, `Arc<Mutex>`). Has not yet written explicit lifetimes or smart-pointer-heavy code.

**Project:** `kvs`, a persistent key-value store. It starts as a library with an in-memory tree and grows into a log-structured storage engine behind an async TCP server.

---

## Progress

**Current step:** 1.3

A box gets ticked only when a review passes (7/10 or better, no must-fix items left). Scores and details live in `REVIEWS.md`.

The course is trimmed to the job-relevant core (re-planned 2026-11-11). Topics you can pick up on the job live in **Later: extras** at the end.

- **Phase 0**
  - [x] 0.1 Skeleton
- **Phase 1: Ownership**
  - [x] 1.1 Owned BST
  - [x] 1.2 Remove and Drop
  - [ ] 1.3 Iterators with lifetimes
  - [ ] 1.4 Parent pointers: the cycle
  - [ ] 1.5 Arena tree (reduced scope)
  - [ ] 1.6 unsafe (after phase 4)
- **Phase 2: Traits**
  - [ ] 2.1 The `Engine` trait and an injected clock
  - [ ] 2.2 Property tests
  - [ ] 2.3 Newtypes and typestate
  - [ ] 2.4 `?Sized`, `Borrow` and DSTs
- **Phase 3: Persistence**
  - [ ] 3.1 Append-only log
  - [ ] 3.2 Recovery and compaction
  - [ ] 3.3 Errors done properly
- **Phase 4: Async**
  - [ ] 4.0 The `Send`/`Sync`/`'static` kata
  - [ ] 4.1 The server and codec
  - [ ] 4.2 Sharing the engine
  - [ ] 4.3 Testing async without sleeping
  - [ ] 4.4 Shutdown and cancellation
  - [ ] 4.5 Backpressure
- **Phase 5: Production quality**
  - [ ] 5.1 Tracing
  - [ ] 5.2 Actor vs lock benchmark
  - [ ] 5.3 Concurrent reads
  - [ ] 5.4 Client, integration test, light API review
- **Later: extras** (optional, no order, pick up if there's time and curiosity)
  - [ ] X.1 Sealed traits
  - [ ] X.2 Declarative macros
  - [ ] X.3 Full benchmark suite and allocation hunting
  - [ ] X.4 Docs, API Guidelines checklist, CI
  - [ ] X.5 Procedural macros (four parts)

---

## How we work

1. Each step has a **goal**, **requirements**, **questions to answer** and **constraints**. You write the code. I don't.
2. When you're done, say `review step X.Y`. I read the code, run `cargo test`, `cargo clippy` and anything else relevant, then grade it.
3. When you're stuck, ask for a hint. Hints come in three levels, and you choose which one you want:
   - **H1:** which concept or doc to look at
   - **H2:** the shape of the solution, without code
   - **H3:** a small, unrelated example of the same technique (still not your code)
4. Answer the **questions** in `NOTES.md` in your own words. Writing the answers down counts toward the grade, because "it compiles" is not the same as "I understand it".

### Grading (each step out of 10, plus a letter grade)

| Axis | What I look for |
|---|---|
| Correctness | Tests pass, edge cases covered, no panics on bad input |
| Tests / TDD | Tests cover the behaviour (not the implementation), have readable names |
| Idiom | Ownership chosen deliberately, no needless `clone()` / `unwrap()` / `Rc<RefCell>`, standard traits used where they fit |
| API design | Types make invalid states unrepresentable, signatures are minimal and honest, follow the Rust API Guidelines |
| Understanding | `NOTES.md` answers are correct and in your own words |

**Commits:** your call. Git history and commit messages are not reviewed or graded.

---

## Tooling (set up in step 0)

- `cargo clippy -- -W clippy::pedantic` (we'll allow specific lints on purpose, with a reason for each)
- `cargo nextest` as the test runner, `cargo llvm-cov` for coverage
- `proptest` for property tests, `insta` for snapshot tests when output gets large
- `thiserror` in library code, `anyhow` only in binaries
- `tracing` + `tracing-subscriber`, never `println!` for diagnostics
- `criterion` for benchmarks
- `cargo +nightly miri test` for checking `unsafe` code and leaks
- `cargo expand` to see macro output, `trybuild` for compile-fail tests
- Async stack: `tokio`, `tokio-util` (codec, `CancellationToken`), `bytes`, `futures`

---

## Phase 0: Workspace and discipline (1 step)

**0.1: Skeleton.** A cargo workspace with `crates/kvs` (lib) and `crates/kvs-server` (bin). Workspace-level lints in `Cargo.toml` (`[workspace.lints]`). A `rust-toolchain.toml`. A `KvsError` enum built with `thiserror` and a `pub type Result<T>` alias.

- Questions: Why should a library return a typed error enum instead of `anyhow::Error`? What does `#[non_exhaustive]` do to your error enum, and when do you want it? What is the difference between a unit test in `mod tests` and a test in `tests/`, in what each one can access?

---

## Phase 1: Ownership in anger (trees and cycles)

This is where lifetimes, smart pointers and the borrow checker's limits stop being abstract.

**1.1: Owned BST.** `BTree<K, V>` (call it `Bst` to avoid clashing with std) with `Option<Box<Node<K, V>>>` children. TDD these, one test at a time: `new`, `insert` (returns the old value), `get`, `len`, `contains_key`.
- Constraint: **iterative** `insert` and `get`, no recursion. This is where you'll meet the borrow checker's famous limitations. Fight it and write down what happened.
- Questions: Why can't a node be `Node { left: Node, right: Node }`? What is the trait bound you need on `K`, and why exactly that one and not more?

**1.2: Remove and Drop.** `remove(&k) -> Option<V>`, including the two-children case. Then write a test that builds a 1,000,000-node degenerate tree (sorted inserts) and drops it.
- Questions: Why did that test crash (or why didn't it)? How do you fix it? What's `Option::take` doing for you in remove, at the memory level?

**1.3: Iterators with lifetimes.** `iter()` returning in-order `(&K, &V)`, implemented as a struct with an explicit stack. Then `IntoIterator` for `&Bst` and for `Bst`. Then `iter_mut()`.
- This is your first real explicit-lifetime work.
- Questions: Why does `iter_mut` give you more trouble than `iter`? What does the `'a` on `Iter<'a, K, V>` actually tie together? What would go wrong if `Iter` didn't carry it?

**1.4: Parent pointers: the cycle.** In a separate module, a tree where each node knows its parent: `Rc<RefCell<Node>>` children, `Weak` parents. Implement `insert` and `successor(node)`, which walks up via the parent.
- Write a test that **proves no leak**: a `Drop` counter (an `AtomicUsize` the node increments on drop). Then deliberately make the parent `Rc` instead of `Weak` and watch the test fail.
- Questions: When does `RefCell` panic, and why is that a runtime check instead of compile time? Why is `Rc` not `Send`? What does that mean for phase 4?

**1.5: Arena tree: the idiomatic answer (reduced scope, ~3–5h).** The same parent-aware tree, but nodes live in a `Vec<Node>` and links are `NodeId(u32)` newtypes. Implement only `insert` and `successor`, and reuse the 1.4 tests. No `remove`.
- Questions: Compare the three versions (Box, Rc/Weak, arena) on ergonomics, performance and safety. What did you trade away, and what would `remove` cost you here? Why do big Rust projects (rustc, petgraph, ECS engines) prefer this?

**1.6: unsafe (do it after phase 4).** A doubly linked list or a parent-pointer tree with raw pointers, verified under Miri.

**Compiler caveats you'll meet in phase 1:** conditional returns of borrows (the NLL "problem case #3"), `&mut` reborrowing in loops, two-phase borrows, `Box` patterns and moving out of a `Box`, the drop check, and why `Option<Box<T>>` is the same size as a pointer (niche optimisation). You'll also meet `Box<T>` and `&T` accepting unsized `T`, which sets up the `?Sized` work in step 2.4.

---

## Phase 2: Traits for design and testability (no `dyn` by default)

The Go habit to unlearn: "accept an interface, mock it in tests". In Rust the default is **generics over a trait**, with test doubles as concrete types and `dyn` only when you need heterogeneous collections or want to cut compile time.

**2.1: The `Engine` trait and an injected clock.** `get`, `set`, `remove`. Implement it for an in-memory engine (backed by your arena BST). Write the **contract tests once** as a generic function `fn engine_contract<E: Engine>(make: impl Fn() -> E)` and run them against every implementation. Then add a `Clock` trait for TTL expiry, with a `FakeClock` you advance by hand in tests, and make the store generic: `Store<E: Engine, C: Clock = SystemClock>`.
- Questions: `&self` or `&mut self` on `set`? (You will revisit this answer in phase 4, so keep a record of your reasoning.) Should `Engine` have associated types? How does the default type parameter keep the public API clean, and what would `dyn Clock` cost you instead?

**2.2: Property tests.** Use `proptest` to check your BST against `std::collections::BTreeMap` as a model: random sequences of insert/remove/get, which must always agree. Also use it to test the BST invariant itself.
- Expect it to find a bug. Write the shrunk failing case as a regular test before you fix it.

**2.3: Newtypes and typestate.** `Key` and `Value` newtypes with validation (max size), using `TryFrom`. A `StoreBuilder` whose `build()` only exists once a path has been set (typestate via a generic marker type).
- Questions: What's the zero-cost argument for newtypes? Which traits (`AsRef`, `Borrow`, `Deref`?) are right or wrong to implement for them? Why is `Deref` on a newtype a known anti-pattern?

**2.4: `?Sized`, `Borrow` and dynamically sized types.** Make `Bst<String, V>::get` accept a `&str` without allocating, the same way `HashMap` and `BTreeMap` do (`K: Borrow<Q>, Q: Ord + ?Sized`). Then write a generic helper that works on `&str`, `&[u8]` and `&dyn Display` alike.
- Questions: Why does `T: ?Sized` *remove* a bound instead of adding one? What is a fat pointer, and what's in its second word for `&str` and for `&dyn Trait`? Why does `Box<dyn Trait>` mean `Box<dyn Trait + 'static>`, and when does that bite you?

---

## Phase 3: Persistence (log-structured engine)

**3.1: Append-only log.** A Bitcask-style design: every `set`/`remove` is appended to a log file as a serialized record (`serde` + `bincode` or your own framing with a length prefix and a CRC). An in-memory index maps key → file offset.
- TDD with `tempfile`. Write generic code over `Read + Seek` / `Write` so tests can use `Cursor<Vec<u8>>` and need no filesystem at all.

**3.2: Recovery and compaction.** Reopen and rebuild the index. Handle a truncated last record (a crash mid-write): detect it, truncate, carry on. Then compact: rewrite live records into a new file and swap atomically (write, fsync, rename).
- Questions: Where do you need `fsync`, and what does `BufWriter` do to your crash guarantees? What happens if a `BufWriter` is dropped with data in it and the flush fails? Why is write-fsync-rename atomic, and what does it still not protect against?

**3.3: Errors done properly.** Review every `unwrap` you've written. Classify each error: recoverable for the caller, a bug (panic is fine), or I/O. Add context without losing the source chain.

---

## Phase 4: Async, and the `Send + Sync + 'static` mayhem

You've used tokio. Now we take apart why it demands what it demands.

**4.0: The kata (no project code).** In a scratch crate, deliberately produce each of these errors, then fix it **two ways**, and write in `NOTES.md` why each fix works:
1. `tokio::spawn` with a borrowed `&str` (needs `'static`)
2. Holding a `std::sync::MutexGuard` across `.await` (the future isn't `Send`)
3. `Rc` inside an async block that's spawned
4. A `RefCell` borrow alive across an await
5. A trait with an `async fn` used generically in a spawned task (the `Send` bound on the returned future; look up return type notation and the `trait_variant` crate)
6. A `Box<dyn Error>` returned from a spawned task (why you need `Box<dyn Error + Send + Sync>`)
7. Spawning a closure that captures `&self`

- Questions: State precisely what `Send` and `Sync` mean in one sentence each, and why `&T: Send` iff `T: Sync`. Why does `spawn` require `'static`, and why can `std::thread::scope` avoid it when tokio can't? What *is* a future structurally (the state machine), and why does that make "held across an await" the thing that matters?

**4.1: The server.** `kvs-server` listens on TCP. Define a wire protocol (a length-prefixed frame or a RESP subset) using a `tokio_util::codec::{Decoder, Encoder}`. TDD the codec as **pure, sync unit tests** over `BytesMut`, with no sockets.

**4.2: Sharing the engine.** Each connection is a task. Pick your sharing design and justify it:
- (a) `Arc<Mutex<E>>`: which mutex, std or tokio? (The tokio docs have a strong opinion. Read it.)
- (b) Interior mutability inside the engine so it's `&self` + `Sync` (revisit your answer from 2.1)
- (c) An actor: one task owns the engine and talks over an `mpsc` channel, with `oneshot` for replies
- Implement (c) and one of (a)/(b). You'll benchmark them in 5.2.

**4.3: Testing async without sleeping.** Use `#[tokio::test(start_paused = true)]` with `tokio::time::advance` for TTL. Test the server through `tokio::io::duplex` instead of real sockets, by keeping the connection handler generic over `AsyncRead + AsyncWrite + Unpin`.
- Questions: What is `Unpin`, and why does this bound show up here? What does `Pin` guarantee, and to whom?

**4.4: Shutdown and cancellation.** Graceful shutdown on Ctrl-C with `CancellationToken` and `JoinSet`. Stop accepting, let in-flight requests finish within a deadline, then flush the log.
- Questions: What is cancellation safety? Which of your `select!` branches are cancel-safe? What happens to a half-written frame when the future is dropped?

**4.5: Backpressure.** A connection limit using `Semaphore` (owned permits moved into the task), and bounded channels everywhere.

---

## Phase 5: Production quality

**5.1: Tracing.** `tracing` spans per connection and per request, with structured fields.
**5.2: Actor vs lock benchmark.** One `criterion` benchmark comparing your two sharing designs from 4.2. Explain the result.
**5.3: Concurrent reads.** Concurrent reads without a global lock (`RwLock`, sharding, or `arc-swap` for the index).
**5.4: Client, integration test, light API review.** A client crate, and an integration test that spins up the real server on port 0. Then a quick pass over your public API: naming, `#[must_use]`, what's `pub` that shouldn't be.

---

## Later: extras

Optional, in no particular order. These are real skills, but easy to pick up on the job. Same review rules apply if you do one.

**X.1: Sealed traits and extension traits.** Seal `Engine` so external crates can't implement it, or decide not to and say why.

**X.2: Declarative macros (`macro_rules!`).** Two macros, each TDD'd:
1. `engine_tests!(name, constructor)`, which expands into a test module running your generic contract suite from 2.1 as *separately named* `#[test]` functions, so a failure says exactly which engine and which case broke.
2. A `kvs!{ "a" => 1, "b" => 2 }` literal that builds a store, accepts a trailing comma, and fails to compile on an odd number of tokens.
- Use `cargo expand` to look at what you generated.
- Questions: What are macro hygiene's rules for local variables versus items? Why can't you follow an `expr` fragment with any token you like? When is a macro the wrong tool compared to a generic function?

**X.3: Full benchmarks.** `criterion` benchmarks for BST vs `BTreeMap` and `get` path allocation counts. Remove allocations you didn't mean to make. Optionally use `loom` to test a small concurrent piece.

**X.4: Docs and CI.** API review against the Rust API Guidelines checklist, rustdoc with runnable examples (doctests are tests), and `cargo deny` / `cargo audit` in CI.

**X.5: Procedural macros.** By now you have hand-written encode/decode code for every protocol command in 4.1. Replace it with a derive, which is how `serde` and `clap` work.
1. **The macro crate.** `crates/kvs-macros` with `proc-macro = true`, depending on `syn`, `quote` and `proc-macro2`. Start with a derive that does nothing but compile, and a `trybuild` test proving it.
2. **`#[derive(Command)]`.** Generates encode and decode impls for a protocol command struct or enum. Support a field attribute such as `#[command(rename = "SET")]`. Your codec tests from 4.1 must still pass unchanged.
3. **Errors as compile errors.** Unsupported input (a union, a generic you can't handle, an unknown attribute) must produce a `syn::Error` with a span pointing at the offending token, never a panic. Test with `trybuild` compile-fail tests and committed `.stderr` snapshots.
4. **An attribute macro.** `#[instrumented]` that wraps a function in a `tracing` span, on both sync and `async fn`.
- Questions: Why must a proc macro live in its own crate? Why is panicking in a proc macro a bad user experience? What do spans control, and what does `quote_spanned!` change?

---

## Rust for Gophers: habits to break

| Go reflex | Rust equivalent |
|---|---|
| Interface + mock in tests | Generic `T: Trait` + a hand-written fake type. `dyn` only when you need it. |
| `nil` | `Option<T>`. A null pointer is not a valid state. |
| `if err != nil { return err }` | `?` plus `From` conversions. Errors are values with types. |
| Goroutine + share memory | Tasks + message passing, or `Arc` + lock. The compiler checks which one you picked. |
| Pointers everywhere for mutation | `&mut` is an *exclusive* borrow, not "a pointer". Think in terms of who is allowed to see it right now. |
| Embedding / OOP-ish reuse | Composition plus traits with default methods. No inheritance, and `Deref` is not a substitute. |
| Zero values | No zero values. `Default` is opt-in and explicit. |
| `context.Context` cancellation | Dropping the future *is* cancellation. That's both more powerful and more dangerous. |

---

## Reading list (read alongside the relevant phase)

- *Learning Rust With Entirely Too Many Linked Lists* (phase 1)
- *Rust for Rustaceans*, Jon Gjengset (phases 2–5, chapters 2, 3, 6, 8)
- *Rust Atomics and Locks*, Mara Bos (phase 5)
- The Rust API Guidelines (phase 2 onward)
- The Tokio tutorial, especially "Shared state" and "Select" (phase 4)
- Alice Ryhl's blog posts "Actors with Tokio" and "Async: What is blocking?" (phase 4)
- The Rustonomicon, chapters on ownership, subtyping and variance (phase 1.6)
- *The Little Book of Rust Macros* (extra X.2)
- David Tolnay's `proc-macro-workshop` repo on GitHub, the `builder` and `debug` projects (extra X.5)
