use std::collections::HashMap;

use crate::{errors::AccountError, tx::Tx};

/// A type for managing accounts and their current currency balance
#[derive(Debug, Clone)]
pub struct Accounts {
    pub accounts: HashMap<String, u64>,
}

impl Accounts {
    /// Returns an empty instance of the [`Accounts`] type
    pub fn new() -> Self {
        Accounts {
            accounts: Default::default(),
        }
    }

    pub fn balance_of(&mut self, signer: &str) -> Result<&u64, AccountError> {
        self.accounts
            .get(signer)
            .ok_or(AccountError::NotFound(signer.to_string()))
    }

    /// Either deposits the `amount` provided into the `signer` account or adds the amount to the existing account.
    /// # Errors
    /// Attempted overflow
    pub fn deposit(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountError> {
        if let Some(account) = self.accounts.get_mut(signer) {
            (*account)
                .checked_add(amount)
                .map(|r| {
                    *account = r;
                    r
                })
                .ok_or(AccountError::OverFunded(signer.to_string(), amount))
                // Using map() here is an easy way to only manipulate the non-error result
                .map(|_| Tx::Deposit {
                    account: signer.to_string(),
                    amount,
                })
        } else {
            self.accounts.insert(signer.to_string(), amount);
            Ok(Tx::Deposit {
                account: signer.to_string(),
                amount,
            })
        }
    }

    /// Withdraws the `amount` from the `signer` account.
    /// # Errors
    /// Attempted overflow
    pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountError> {
        // check if signer exists inside accounts hashmap
        match self.accounts.get_mut(signer) {
            // If account exists, start withdrawal:
            Some(account_balance) => {
                // subtract amount from account balance
                (*account_balance)
                    .checked_sub(amount)
                    // if it's successful, update new account_balance to be subtraction result
                    .map(|r| {
                        *account_balance = r;
                        r
                    })
                    .ok_or(
                        // if it fails, then return AccountError::UnderFunded
                        AccountError::UnderFunded(signer.to_string()),
                    )
                    .map(|_| Tx::Withdraw {
                        account: signer.to_string(),
                        amount,
                    })
            }
            // If account doesn't exist, return AccountError::NotFound
            None => Err(AccountError::NotFound(signer.to_string())),
        }
    }

    /// Withdraws the amount from the sender account and deposits it in the recipient account.
    ///
    /// # Errors
    /// The account doesn't exist
    pub fn send(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<(Tx, Tx), AccountError> {
        // withdraw amount from sender
        let w_tx = self.withdraw(sender, amount)?;
        // deposit amount to recipient
        let d_tx = self.deposit(recipient, amount)?;

        Ok((w_tx, d_tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), AccountError>;

    // unit tests for Accounts.withdraw()
    // =========================================================================================================
    #[test]
    fn test_accounts_withdraw_successful() -> TestResult {
        let mut ledger = Accounts::new();
        ledger.deposit("test_account", 50)?;

        let actual = ledger.withdraw("test_account", 10);
        assert_eq!(
            actual,
            Ok(Tx::Withdraw {
                account: "test_account".to_string(),
                amount: 10,
            })
        );

        Ok(())
    }

    #[test]
    fn test_accounts_withdraw_missing() -> TestResult {
        let mut ledger = Accounts::new();

        let actual = ledger.withdraw("non_existant_account", 10);
        assert_eq!(
            actual,
            Err(AccountError::NotFound("non_existant_account".to_string()))
        );

        Ok(())
    }

    #[test]
    fn test_accounts_withdraw_underfunded() -> TestResult {
        let mut ledger = Accounts::new();
        ledger.deposit("test_account", 50)?;

        let actual = ledger.withdraw("test_account", 60);
        assert_eq!(
            actual,
            Err(AccountError::UnderFunded("test_account".to_string()))
        );

        Ok(())
    }

    // unit tests for Accounts.deposit()
    // =========================================================================================================
    #[test]
    fn test_accounts_deposit_successful() -> TestResult {
        let mut ledger = Accounts::new();

        let actual = ledger.deposit("test_account", 50);
        assert_eq!(
            actual,
            Ok(Tx::Deposit {
                account: "test_account".to_string(),
                amount: 50
            })
        );

        Ok(())
    }

    #[test]
    fn test_accounts_deposit_overfunded() -> TestResult {
        let mut ledger = Accounts::new();
        ledger.deposit("test_account", 10)?;

        let actual = ledger.deposit("test_account", u64::MAX);
        assert_eq!(
            actual,
            Err(AccountError::OverFunded(
                "test_account".to_string(),
                u64::MAX
            ))
        );

        Ok(())
    }

    // unit tests for Accounts.send()
    // =========================================================================================================
    #[test]
    fn test_accounts_send_successful() -> TestResult {
        let mut ledger = Accounts::new();
        ledger.deposit("sender", 50)?;
        ledger.deposit("recipient", 10)?;

        let (actual_tx_1, actual_tx_2) = ledger.send("sender", "recipient", 30)?;

        assert_eq!(
            actual_tx_1,
            Tx::Withdraw {
                account: "sender".to_string(),
                amount: 30
            }
        );

        assert_eq!(
            actual_tx_2,
            Tx::Deposit {
                account: "recipient".to_string(),
                amount: 30
            }
        );

        Ok(())
    }

    #[test]
    fn test_accounts_send_missing_sender() -> TestResult {
        let mut ledger = Accounts::new();
        ledger.deposit("recipient", 10)?;

        let actual = ledger.send("non_existant_account", "recipient", 30);
        assert_eq!(
            actual,
            Err(AccountError::NotFound("non_existant_account".to_string()))
        );

        Ok(())
    }
}
