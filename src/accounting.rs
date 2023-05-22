use crate::errors::AccountError;
use crate::tx::Tx;
use std::collections::HashMap;

/// A type for managing accounts and their current currency balance
#[derive(Debug, Clone)]
pub struct Accounts {
    accounts: HashMap<String, u64>,
}

impl Accounts {
    /// Returns an empty instance of the [`Accounts`] type
    pub fn new() -> Self {
        Accounts {
            accounts: Default::default(),
        }
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
