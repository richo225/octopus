mod accounting;
mod core;
mod trading_platform;

use crate::core::{Order, Side};
use octopus_common::{errors, tx};
use trading_platform::TradingPlatform;

fn main() {
    println!("Hello, world!");
}
