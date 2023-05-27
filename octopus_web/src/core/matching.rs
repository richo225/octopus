use std::collections::{BTreeMap, BinaryHeap};

use crate::core::{Order, Receipt, Side};
use octopus_common::errors::AccountError;

use super::PartialOrder;

#[derive(Default, Debug)]
pub struct MatchingEngine {
    /// The last sequence number
    pub ordinal: u64,

    /// The "Bid" or "Buy" side of the order book. Ordered by ordinal number.
    pub bids: BTreeMap<u64, BinaryHeap<PartialOrder>>,
    /// The "Ask" or "Sell" side of the order book. Ordered by ordinal number.
    pub asks: BTreeMap<u64, BinaryHeap<PartialOrder>>,

    /// Previous matches for record keeping
    pub history: Vec<Receipt>,
}

impl MatchingEngine {
    /// Creates a new [`MatchingEngine`] with an ordinal of 0 and empty books
    pub fn new() -> Self {
        MatchingEngine {
            ordinal: 0,
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            history: Vec::new(),
        }
    }

    /// Processes an [`Order`] and returns a [`Receipt`]
    /// This includes matching the order to whatever is in the current books and adding the remainder (if any) to the book for future matching.
    pub fn process(&mut self, order: Order) -> Result<Receipt, AccountError> {
        // Increment the ordinal number for this order
        self.ordinal += 1;
        let ordinal = self.ordinal;

        let original_amount = order.amount;
        let mut partial = order.into_partial_order(ordinal, original_amount);

        // Orders are matched to the opposite side
        let receipt = match &partial.side {
            Side::Buy => {
                // Fetch all sell orders(asks) in the expected price range from the orderbook
                let orderbook_entry = self.asks.range_mut(u64::MIN..=partial.price);

                let receipt = MatchingEngine::match_order(&partial, orderbook_entry, ordinal)?;

                // Sum up all the amount in the matches
                let matched_amount: u64 = receipt.matches.iter().map(|m| m.amount).sum();

                // If order wasn't fully matched
                if matched_amount < original_amount {
                    partial.amount = original_amount - matched_amount;
                    let price = partial.price;
                    // Find any bids of the same price or insert default as a min-heap
                    let bids = self.bids.entry(price).or_insert(vec![].into());
                    bids.push(partial)
                }

                receipt
            }
            Side::Sell => {
                // Fetch all buy orders(bids) in the expected price range from the orderbook
                let orderbook_entry = self.bids.range_mut(partial.price..=u64::MAX);

                // Pass the order to be proccessed and all the buy orders from the orderbook to the matching algorithm
                let receipt = MatchingEngine::match_order(&partial, orderbook_entry, ordinal)?;
                let matched_amount: u64 = receipt.matches.iter().map(|m| m.amount).sum();

                // The order wasn't fully matched
                if matched_amount < original_amount {
                    partial.amount = original_amount - matched_amount;
                    let price = partial.price;
                    let asks = self.asks.entry(price).or_insert(vec![].into());
                    asks.push(partial);
                }
                receipt
            }
        };

        // Cleanup: Remove price entries without orders from the orderbook
        self.asks.retain(|_, orders| !orders.is_empty());
        self.bids.retain(|_, orders| !orders.is_empty());

        // Keep a log of matches
        self.history.push(receipt.clone());
        Ok(receipt)
    }

    /// Matches an order to the provided order book side.
    /// # Parameters
    /// - `order`: the order to match to the book
    /// - `orderbook_entry`: a pre-filtered iterator for order book_entry in the correct price range
    /// - `ordinal` the next ordinal number to use if a position is opened
    fn match_order<'a, T>(
        order: &PartialOrder,
        mut orderbook_entry: T,
        ordinal: u64,
    ) -> Result<Receipt, AccountError>
    where
        T: Iterator<Item = (&'a u64, &'a mut BinaryHeap<PartialOrder>)>,
    {
        let mut remaining_amount: u64 = order.amount;
        let mut matches: Vec<PartialOrder> = vec![];

        // Each matching position's amount is subtracted
        'outer: while remaining_amount > 0 {
            // The iterator contains all orderbook_entry of a price point
            match orderbook_entry.next() {
                Some((_price, orderbook_entry)) => {
                    // store any self matches or partially filled orderbook entries to add back after loop
                    let mut orderbook_returns = vec![];

                    // 1. remove the Order with the lowest sequence nr from the orderbook entry
                    // Orderbook entry is a Min-heap so pop() returns smallest value
                    'pop: while let Some(mut entry) = orderbook_entry.pop() {
                        // 2. skip over if it's their own order
                        if entry.signer == order.signer {
                            orderbook_returns.push(entry.clone());
                            continue 'pop;
                        }

                        // 3. subtract the entry remaining amount from the current order amount
                        //  a. if something left (partial match) add the order book order to the order matches and continue from 1
                        //  b. if 0 is left (exact full match), do the same
                        if (remaining_amount as i64 - entry.remaining as i64) >= 0 {
                            entry.remaining = 0;
                            remaining_amount -= entry.remaining;

                            matches.push(entry);
                        } else {
                            // c. if negative is left (full match), split the Order into two, add original to matches and clone into the orderbook entry
                            orderbook_returns
                                .push(PartialOrder::take_from(&mut entry, remaining_amount));
                            matches.push(entry);
                            remaining_amount = 0;
                        }
                    }

                    orderbook_returns
                        .into_iter()
                        .for_each(|m| orderbook_entry.push(m));
                }
                // Nothing left to match with
                None => break 'outer,
            }
            // 4. repeat until the order has been filled to its fullest (remaining amount is 0)
        }
        Ok(Receipt { ordinal, matches })
    }
}

#[cfg(test)]
mod tests {
    // reduce the warnings for naming tests
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn test_MatchingEngine_process_partially_match_order() {
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let bob_receipt = matching_engine
            .process(Order {
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

        // Bob's order was only partially filled
        assert_eq!(matching_engine.asks.len(), 0);
        assert_eq!(matching_engine.bids.len(), 1);
    }

    #[test]
    fn test_MatchingEngine_process_fully_match_order() {
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 2,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let bob_receipt = matching_engine
            .process(Order {
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
        assert!(matching_engine.asks.is_empty());
        assert!(matching_engine.bids.is_empty());
    }

    #[test]
    fn test_MatchingEngine_process_fully_match_order_overfill() {
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 3,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let bob_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Buy,
                signer: "BOB".to_string(),
            })
            .unwrap();

        assert_eq!(
            bob_receipt.matches,
            vec![PartialOrder {
                price: 10,
                amount: 3,
                remaining: 2,
                side: Side::Sell,
                signer: "ALICE".to_string(),
                ordinal: 1
            }]
        );

        // A sell order with an updated remaining is added to the book
        assert_eq!(matching_engine.asks.len(), 1);
        assert!(matching_engine.bids.is_empty());
    }

    #[test]
    fn test_MatchingEngine_process_fully_match_order_multi_match() {
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let charlie_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "CHARLIE".to_string(),
            })
            .unwrap();
        assert_eq!(charlie_receipt.matches, vec![]);
        assert_eq!(charlie_receipt.ordinal, 2);

        let bob_receipt = matching_engine
            .process(Order {
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
        assert!(matching_engine.asks.is_empty());
        assert!(matching_engine.bids.is_empty());
    }

    #[test]
    fn test_MatchingEngine_process_fully_match_order_no_self_match() {
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let charlie_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Sell,
                signer: "CHARLIE".to_string(),
            })
            .unwrap();
        assert_eq!(charlie_receipt.matches, vec![]);
        assert_eq!(charlie_receipt.ordinal, 2);

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 2,
                side: Side::Buy,
                signer: "ALICE".to_string(),
            })
            .unwrap();

        assert_eq!(
            alice_receipt.matches,
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
        assert_eq!(matching_engine.asks.len(), 1);
        assert_eq!(matching_engine.bids.len(), 1);
    }

    #[test]
    fn test_MatchingEngine_process_no_match() {
        let mut matching_engine = MatchingEngine::new();

        let alice_receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 2,
                side: Side::Sell,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(alice_receipt.matches, vec![]);
        assert_eq!(alice_receipt.ordinal, 1);

        let bob_receipt = matching_engine
            .process(Order {
                price: 11,
                amount: 2,
                side: Side::Sell,
                signer: "BOB".to_string(),
            })
            .unwrap();

        assert_eq!(bob_receipt.matches, vec![]);
        assert_eq!(matching_engine.asks.len(), 2);
    }

    #[test]
    fn test_MatchingEngine_process_increment_ordinal_matching_engine() {
        let mut matching_engine = MatchingEngine::new();

        assert_eq!(matching_engine.ordinal, 0);

        let receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Buy,
                signer: "ALICE".to_string(),
            })
            .unwrap();
        assert_eq!(receipt.ordinal, matching_engine.ordinal);

        let receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Buy,
                signer: "BOB".to_string(),
            })
            .unwrap();
        assert_eq!(receipt.ordinal, matching_engine.ordinal);

        let receipt = matching_engine
            .process(Order {
                price: 10,
                amount: 1,
                side: Side::Buy,
                signer: "CHARLIE".to_string(),
            })
            .unwrap();
        assert_eq!(receipt.ordinal, matching_engine.ordinal);
        assert_eq!(matching_engine.ordinal, 3);
    }
}
