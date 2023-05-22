/// An application-specific error type
#[derive(Debug)]
pub enum ApplicationError {
    // Add variants here for account not found, account underfunded and account overfunded
    AccountNotFound(String),
    AccountOverFunded(String, u64),
    AccountUnderFunded(String),
}
