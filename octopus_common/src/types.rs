use std::cmp::{Ordering, Reverse};

/// Simplified side of a position as well as order.
#[derive(Clone, PartialOrd, PartialEq, Eq, Debug, Ord)]
pub enum Side {
    /// Want to buy
    Buy,
    /// Want to sell
    Sell,
}

/// An order for a specified symbol to buy or sell an amount at a given price.
#[derive(Clone, PartialEq, Eq)]
pub struct Order {
    /// Max/min price (depending on the side)
    pub price: u64,
    /// Number of units to trade
    pub amount: u64,
    /// The side of the order book (buy or sell)
    pub side: Side,
    /// The account signer
    pub signer: String,
}

impl Order {
    /// Convert an [`Order`] into a [`PartialOrder`] with the added parameters
    pub fn into_partial_order(self, ordinal: u64, remaining: u64) -> PartialOrder {
        let Order {
            price,
            amount,
            side,
            signer,
        } = self;
        PartialOrder {
            price,
            amount,
            remaining,
            side,
            signer,
            ordinal,
        }
    }
}

/// An unfilled order that is kept in the system for later filling.
#[derive(Clone, PartialEq, Debug, Eq, Ord)]
pub struct PartialOrder {
    /// Price per unit
    pub price: u64,
    /// Initial number of units in the order
    pub amount: u64,
    /// Remaining number of units after potential matches
    pub remaining: u64,
    /// Buy or sell side of the book
    pub side: Side,
    /// Signer of the order
    pub signer: String,
    /// Sequence number for order prioritisation
    pub ordinal: u64,
}

impl PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // this reverses the comparison to create a min heap
        Reverse(self.ordinal).partial_cmp(&Reverse(other.ordinal))
    }
}

/// A receipt issued to the caller for accepting an [`Order`]
#[derive(Clone, PartialOrd, PartialEq, Eq, Debug)]
pub struct Receipt {
    /// Sequence number
    pub ordinal: u64,

    /// Matches that happened immediately
    pub matches: Vec<PartialOrder>,
}

impl PartialOrder {
    /// Splits one [`PartialOrder`] into two by taking a defined `take` amount
    pub fn take_from(pos: &mut PartialOrder, take: u64) -> PartialOrder {
        pos.remaining -= take;
        let mut new = pos.clone();
        new.amount = take;
        new
    }
}
