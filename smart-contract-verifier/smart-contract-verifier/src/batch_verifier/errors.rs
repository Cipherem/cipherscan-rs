use std::fmt::{Display, Formatter};
use thiserror::Error;

/// Enumerates errors that may occur during a single contract verification.
#[derive(Error, Debug)]
pub enum VerificationErrorKind {
    #[error("internal error: {0:#}")]
    InternalError(#[from] anyhow::Error),
    #[error("neither creation nor runtime code do not have a match")]
    CodeMismatch,
    #[error("invalid constructor arguments: {0}")]
    InvalidConstructorArguments(blockscout_display_bytes::Bytes),
}

/// Error obtained as a result of a single contract verification.
/// Is used to return more details about verification process to the caller.
///
/// Returns the contract for which error occurred and the error itself.
#[derive(Error, Debug)]
pub struct VerificationError {
    pub file_path: String,
    pub contract_name: String,
    #[source]
    pub kind: VerificationErrorKind,
}

impl Display for VerificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} - {}",
            self.file_path, self.contract_name, self.kind
        )
    }
}

impl VerificationError {
    pub const fn new(
        file_path: String,
        contract_name: String,
        kind: VerificationErrorKind,
    ) -> Self {
        Self {
            file_path,
            contract_name,
            kind,
        }
    }
}
