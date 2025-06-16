use std::{error::Error, fmt};

#[derive(Debug)]
pub struct PoolCreationError{}

impl Error for PoolCreationError {}
impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "La création du pool a échoué")
    }
}