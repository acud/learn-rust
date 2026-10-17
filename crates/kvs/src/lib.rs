use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {}

pub type Result<T, E> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {}
