//! Implements specific error handling for the Verification System

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AENEASError {
    RuntimeError,
}

impl fmt::Display for AENEASError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AENEASError::RuntimeError => {
                write!(f, "AENEAS Failed to run. Terminating verification")
            }
        }
    }
}

impl Error for AENEASError {}
