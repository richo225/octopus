/// An application-specific error type
#[derive(Debug)]
pub enum AccountError {
    // Add variants here for account not found, account underfunded and account overfunded
    NotFound(String),
    OverFunded(String, u64),
    UnderFunded(String),
}
