phase 0 answers:
1. Why should a library return a typed error enum instead of anyhow::Error?
Because that way we can transform recognized own-domain errors into proper flow control statements in pattern matching instead of having
to do error detection every time. It also puts the error contract into the API itself. Using anyhow error erases the domain specific context of the error.

2. What does #[non_exhaustive] do to your error enum, and when do you want it?
  - On the enum: forces downstream matchers to add a wildcard on the enum so that they catch errors gracefully
  - On a single variant: downstream users can't create the variant instance
  - On a struct: no struct literal construction downstream, and deconstructing needs the "..".

WHEN DO YOU WANT IT: on an enum - when you want to allow to add variants without them being breaking changes.
on the single variant - when you don't want to give downstream callers the ability to create the instance (error is losing its domain semantics)
on a struct - forcing callers to use constructors and create gracefully failing implementations (using "..")

3. What's the difference between a unit test in mod tests and a test in tests/, in terms of what each one can access?
The unit test has access to the entire module tree of the file it is in. it therefore can also do white-box testing.

in tests/ every file is a separate crate and can only see the exported members of a given crate that is being tested, which forces black box testing.


Step 1.1 answers:


1. Why can't a node be Node { left: Node, right: Node }?
because the compiler tries to determine the type's size at compile time, and recursive types cannot have a deterministic size at compile time, that's why we use Box - to allocate the pointer to the type on the heap as an intermediate value, which would allow the compiler to resolve any recursive reference as just usize.

2. Which trait bound does K need, and why exactly that one and no more?
It needs Ord, because we want to know if the key that we compare against is < = > than the provided value. The other trait bounds for comparing provide us with semantics we don't need for this implementation.

3. Write-up: the borrow-checker error you hit while writing iterative insert, what it said, and whether your code was wrong or the checker was too
conservative.

I hit the "first borrow used here, in later iteration of loop" E0499 (NLL problem case #3) while writing the insert.
I worked through it like this:
initially we had an as_mut() call in the first match, this caused that the reborrow was created before any of the match arms were entered. but also that because as_mut takes &self mut, then it would try to reborrow again on the mutable borrow using references to that reborrow and that would cause the compiler to not assert correctly whether we're trying to reborrow again a mutable ref.
The problem was in the None arm that modified the None value and the Equals arm that also mutated the value.
I went around it by matching directly on cur (the place itself) instead of creating intermediate references (as_mut) and then writing the logic that actually modifies the data in the right places.
That allowed the compiler to borrow mutably just when needed (when the match arms bind) instead of when the initial match case happened.

NOTE: the last clippy pending change - i tried to apply it, but it won't compile, not sure why.
