phase 0 answers:
1. Why should a library return a typed error enum instead of anyhow::Error?
Because that way we can transform recognized own-domain errors into proper flow control statements in pattern matching instead of having
to do error detection every time. It also puts the error contract into the API itself. Using anyhow error erases the domain specific context of the error.

2. What does #[non_exhaustive] do to your error enum, and when do you want it?
It prohibits external callers of the library to construct the type instance or the variant.
It also forces the pattern matching over the error to always include a wildcard that would force them to handle any future errors gracefully.
You want this whenever you want to limit whether downstream crates can generate the error type or not (when on the type) or whenever a certain error or type can possibly change in the future.

3. What's the difference between a unit test in mod tests and a test in tests/, in terms of what each one can access?
The unit test has access to the entire module tree of the file it is in. it therefore can also do white-box testing.

in tests/ every file is a separate crate and can only see the exported members of a given crate that is being tested, which forces black box testing.

