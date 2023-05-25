use std::collections::{BTreeMap, BinaryHeap};

use crate::{
    accounting::Accounts,
    core::{MatchingEngine, Order, PartialOrder, Receipt, Side},
    errors::AccountError,
    tx::Tx,
};

pub struct TradingPlatform {
    engine: MatchingEngine,
    accounts: Accounts,
    transactions: Vec<Tx>,
}

impl TradingPlatform {
    pub fn new() -> Self {
        TradingPlatform {
            engine: MatchingEngine::new(),
            accounts: Accounts::new(),
            transactions: Vec::new(),
        }
    }

    /// Fetches the complete order book at this time
    pub fn orderbook(&self) {
        let mut orderbook: BTreeMap<u64, BinaryHeap<PartialOrder>> = BTreeMap::new();
        self.engine.asks.clone_into(&mut orderbook);
        self.engine.bids.clone_into(&mut orderbook);
        // finish off iteration
        println!("{:?}", orderbook);
    }

    /// Fetch total price of user account
    pub fn balance_of(&mut self, signer: &str) -> Result<&u64, AccountError> {
        self.accounts.balance_of(signer)
    }

    /// Deposit funds
    pub fn deposit(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountError> {
        let operation: Result<Tx, AccountError> = self.accounts.deposit(signer, amount);
        operation.map(|tx| {
            self.transactions.push(tx.clone());

            tx
        })
    }

    /// Withdraw funds
    pub fn withdraw(&mut self, signer: &str, amount: u64) -> Result<Tx, AccountError> {
        let operation: Result<Tx, AccountError> = self.accounts.withdraw(signer, amount);
        operation.map(|tx| {
            self.transactions.push(tx.clone());

            tx
        })
    }

    /// Transfer funds between sender and recipient
    pub fn send(
        &mut self,
        sender: &str,
        recipient: &str,
        amount: u64,
    ) -> Result<(Tx, Tx), AccountError> {
        let operation: Result<(Tx, Tx), AccountError> =
            self.accounts.send(sender, recipient, amount);
        operation.map(|tx: (Tx, Tx)| {
            // why clone ahhhh
            // need to work out vec of structs ownership!!!
            // 😭
            // 😭
            // 😭
            let cloned_tx = tx.clone();
            self.transactions.push(cloned_tx.0);
            self.transactions.push(cloned_tx.1);

            tx
        })
    }

    /// Process a given order and apply the outcome to the accounts involved. Note that there are very few safeguards in place.
    pub fn order(&mut self, order: Order) -> Result<Receipt, AccountError> {
        let signer = &order.signer;

        // 1. Check if signer has an account
        let balance = self.balance_of(signer)?;

        // 2. Check if buy order signer has enough money in account
        let total_cost = order.amount * order.price;
        balance
            .checked_sub(total_cost)
            .ok_or(AccountError::UnderFunded(signer.to_string()))?;

        // 3. Process the order by the engine
        let receipt = self.engine.process(order.clone())?;

        match order.side {
            // 4.If the order is BUY, send the total price to each of the matches
            Side::Buy => {
                for po in receipt.clone().matches {
                    let total_realized = po.amount * po.price;
                    self.send(signer, &po.signer, total_realized)?;
                }
            }
            // 5.If the order is SELL, send the total price from each of the matches
            Side::Sell => {
                for po in receipt.clone().matches {
                    let total_realized = po.amount * po.price;
                    self.send(&po.signer, signer, total_realized)?;
                }
            }
        }
        // 4. Return the receipt
        Ok(receipt)
    }
}

#[cfg(test)]
mod tests {
    // reduce the warnings for naming tests
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    #[ignore]
    fn test_TradingPlatform_order_requires_deposit_to_order() {
        let mut trading_platform = TradingPlatform::new();

        assert_eq!(
            trading_platform.order(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            }),
            Err(AccountError::NotFound("ALICE".to_string()))
        );
        assert!(trading_platform.engine.asks.is_empty());
        assert!(trading_platform.engine.bids.is_empty());
    }

    #[test]
    fn test_TradingPlatform_order_partially_match_order_updates_accounts() {
        let mut trading_platform = TradingPlatform::new();

        // Set up accounts
        assert!(trading_platform.accounts.deposit("ALICE", 100).is_ok());
        assert!(trading_platform.accounts.deposit("BOB", 100).is_ok());

        let alice_receipt = trading_platform
            .order(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let bob_receipt = trading_platform
            .order(Order {
                price: 10,
                amount: 2,
                side: Side::Buy,
                signer: "BOB".to_string(),
            })
            .unwrap();

        assert_eq!(
            bob_receipt.matches,
            vec![PartialOrder {
                price: 10,
                amount: 1,
                remaining: 0,
                side: Side::Sell,
                signer: "ALICE".to_string(),
                ordinal: 1
            }]
        );
        assert!(trading_platform.engine.asks.is_empty());
        assert_eq!(trading_platform.engine.bids.len(), 1);

        // Check the account balances
        assert_eq!(trading_platform.accounts.balance_of("ALICE"), Ok(&110));
        assert_eq!(trading_platform.accounts.balance_of("BOB"), Ok(&90));
    }

    #[test]
    fn test_TradingPlatform_order_fully_match_order_updates_accounts() {
        let mut trading_platform = TradingPlatform::new();

        // Set up accounts
        assert!(trading_platform.accounts.deposit("ALICE", 100).is_ok());
        assert!(trading_platform.accounts.deposit("BOB", 100).is_ok());

        let alice_receipt = trading_platform
            .order(Order {
                price: 10,
                amount: 2,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let bob_receipt = trading_platform
            .order(Order {
                price: 10,
                amount: 2,
                side: Side::Buy,
                signer: "BOB".to_string(),
            })
            .unwrap();

        assert_eq!(
            bob_receipt.matches,
            vec![PartialOrder {
                price: 10,
                amount: 2,
                remaining: 0,
                side: Side::Sell,
                signer: "ALICE".to_string(),
                ordinal: 1
            }]
        );

        // A fully matched order doesn't remain in the book
        assert!(trading_platform.engine.asks.is_empty());
        assert!(trading_platform.engine.bids.is_empty());

        // Check the account balances
        assert_eq!(trading_platform.accounts.balance_of("ALICE"), Ok(&120));
        assert_eq!(trading_platform.accounts.balance_of("BOB"), Ok(&80));
    }

    #[test]
    fn test_TradingPlatform_order_fully_match_order_multi_match_updates_accounts() {
        let mut trading_platform = TradingPlatform::new();

        // Set up accounts
        assert!(trading_platform.accounts.deposit("ALICE", 100).is_ok());
        assert!(trading_platform.accounts.deposit("BOB", 100).is_ok());
        assert!(trading_platform.accounts.deposit("CHARLIE", 100).is_ok());

        let alice_receipt = trading_platform
            .order(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let charlie_receipt = trading_platform
            .order(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "CHARLIE".to_string(),
            })
            .unwrap();
        assert_eq!(charlie_receipt.matches, vec![]);
        assert_eq!(charlie_receipt.ordinal, 2);

        let bob_receipt = trading_platform
            .order(Order {
                price: 10,
                amount: 2,
                side: Side::Buy,
                signer: "BOB".to_string(),
            })
            .unwrap();

        assert_eq!(
            bob_receipt.matches,
            vec![
                PartialOrder {
                    price: 10,
                    amount: 1,
                    remaining: 0,
                    side: Side::Sell,
                    signer: "ALICE".to_string(),
                    ordinal: 1
                },
                PartialOrder {
                    price: 10,
                    amount: 1,
                    remaining: 0,
                    side: Side::Sell,
                    signer: "CHARLIE".to_string(),
                    ordinal: 2
                }
            ]
        );
        // A fully matched order doesn't remain in the book
        assert!(trading_platform.engine.asks.is_empty());
        assert!(trading_platform.engine.bids.is_empty());

        // Check account balances
        assert_eq!(trading_platform.accounts.balance_of("ALICE"), Ok(&110));
        assert_eq!(trading_platform.accounts.balance_of("BOB"), Ok(&80));
        assert_eq!(trading_platform.accounts.balance_of("CHARLIE"), Ok(&110));
    }

    #[test]
    fn test_TradingPlatform_order_fully_match_order_no_self_match_updates_accounts() {
        let mut trading_platform = TradingPlatform::new();

        // Set up accounts
        assert!(trading_platform.accounts.deposit("ALICE", 100).is_ok());
        assert!(trading_platform.accounts.deposit("CHARLIE", 100).is_ok());

        let alice_receipt = trading_platform
            .order(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let charlie_receipt = trading_platform
            .order(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "CHARLIE".to_string(),
            })
            .unwrap();
        assert_eq!(charlie_receipt.matches, vec![]);
        assert_eq!(charlie_receipt.ordinal, 2);

        let bob_receipt = trading_platform
            .order(Order {
                price: 10,
                amount: 2,
                side: Side::Buy,
                signer: "ALICE".to_string(),
            })
            .unwrap();

        assert_eq!(
            bob_receipt.matches,
            vec![PartialOrder {
                price: 10,
                amount: 1,
                remaining: 0,
                side: Side::Sell,
                signer: "CHARLIE".to_string(),
                ordinal: 2
            }]
        );
        // A fully matched order doesn't remain in the book
        assert_eq!(trading_platform.engine.asks.len(), 1);
        assert_eq!(trading_platform.engine.bids.len(), 1);
        // Check account balances
        assert_eq!(trading_platform.accounts.balance_of("ALICE"), Ok(&90));
        assert_eq!(trading_platform.accounts.balance_of("CHARLIE"), Ok(&110));
    }

    // #[test]
    // fn test_TradingPlatform_order_no_match_updates_accounts() {
    //     let mut trading_platform = TradingPlatform::new();

    //     // Set up accounts
    //     assert!(trading_platform.accounts.deposit("ALICE", 100).is_ok());
    //     assert!(trading_platform.accounts.deposit("BOB", 100).is_ok());

    //     let alice_receipt = trading_platform
    //         .order(Order {
    //             price: 10,
    //             amount: 2,
    //             side: Side::Sell,
    //             signer: "ALICE".to_string(),
    //         })
    //         .unwrap();
    //     assert_eq!(alice_receipt.matches, vec![]);
    //     assert_eq!(alice_receipt.ordinal, 1);

    //     let bob_receipt = trading_platform
    //         .order(Order {
    //             price: 11,
    //             amount: 2,
    //             side: Side::Sell,
    //             signer: "BOB".to_string(),
    //         })
    //         .unwrap();

    //     assert_eq!(bob_receipt.matches, vec![]);
    //     assert_eq!(trading_platform.orderbook().len(), 2);

    //     // Check the account balances
    //     assert_eq!(trading_platform.accounts.balance_of("ALICE"), Ok(&100));
    //     assert_eq!(trading_platform.accounts.balance_of("BOB"), Ok(&100));
    // }
}
