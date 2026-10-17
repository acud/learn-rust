phase 0 answers:
1. Why should a library return a typed error enum instead of anyhow::Error?
Because that way we can transform recognized own-domain errors into proper flow control statements in pattern matching instead of having
to do error detection every time.

2. What does #[non_exhaustive] do to your error enum, and when do you want it?
It prohibits any crate which is outside the defining crate to create an instance. You probably want it when you want to limit which component/crate can actually create the error type instance.

3. What's the difference between a unit test in mod tests and a test in tests/, in terms of what each one can access?
The unit test has access to everything in the file it is exported in (usually because they have the super::* import), it therefore can also do white-box testing.
in tests/ one is forced to write black-box tests and private members cannot be seen by the test.
