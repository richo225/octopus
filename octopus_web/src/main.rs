mod accounting;
mod core;
mod trading_platform;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::core::{Order, Side};
use octopus_common::{errors, tx};
use trading_platform::TradingPlatform;

fn main() {
    pretty_env_logger::init();

    info!("starting up");
}
