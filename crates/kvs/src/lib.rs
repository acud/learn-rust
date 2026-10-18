use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum KvsError {}

pub type Result<T> = std::result::Result<T, KvsError>;

pub mod bst;

#[cfg(test)]
mod tests {}
