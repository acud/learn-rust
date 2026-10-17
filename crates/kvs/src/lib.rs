use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    #[error("placeholder for future errors")]
    Placeholder, // added so the type isn't a never type
}

pub type Result<T> = std::result::Result<T, KvsError>;

#[cfg(test)]
mod tests {}
