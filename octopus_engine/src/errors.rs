use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

/// An application-specific error type
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountError {
    // Add variants here for account not found, account underfunded and account overfunded
    NotFound(String),
    OverFunded(String, u64),
    UnderFunded(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OctopusError(pub AccountError);

impl fmt::Display for OctopusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {:?}", self.0)
    }
}

impl Error for OctopusError {}
