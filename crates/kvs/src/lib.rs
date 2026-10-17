use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {}

pub type Result<T> = std::result::Result<T, KvsError>;

#[cfg(test)]
mod tests {}
