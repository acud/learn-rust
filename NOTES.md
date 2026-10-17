phase 0 answers:
1. Why should a library return a typed error enum instead of anyhow::Error?
Because that way we can transform recognized own-domain errors into proper flow control statements in pattern matching instead of having
to do error detection every time. It also puts the error contract into the API itself. Using anyhow error erases the domain specific context of the error.

2. What does #[non_exhaustive] do to your error enum, and when do you want it?
  - On the enum: the only effect outside your crate is on matching. Downstream code must include a wildcard arm. Constructing existing variants
    from outside is still allowed. The guarantee you get is that adding a new variant later isn't a breaking change.
  - On a single variant: outside crates can't construct that variant, and when they match on its fields they must use ..
  - On a struct: the same idea. No struct-literal construction from outside, and destructuring needs ... This is the reason libraries give such
    structs constructor functions.

3. What's the difference between a unit test in mod tests and a test in tests/, in terms of what each one can access?
The unit test has access to the entire module tree of the file it is in. it therefore can also do white-box testing.

in tests/ every file is a separate crate and can only see the exported members of a given crate that is being tested, which forces black box testing.

