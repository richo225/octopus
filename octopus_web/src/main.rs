mod accounting;
mod core;
mod trading_platform;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::collections::HashMap;

use crate::core::{Order, Side};
use octopus_common::{errors, tx};
use trading_platform::TradingPlatform;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("starting up");

    // GET /hello
    let hello = warp::get()
        .and(warp::path!("hello"))
        .map(|| format!("Hello there!!!!!"));

    // GET /accounts
    let accounts = warp::get()
        .and(warp::path!("accounts"))
        .map(|| format!("List of accounts"));

    // GET /orderbook
    let orderbook = warp::get()
        .and(warp::path!("orderbook"))
        .map(|| format!("List of orders"));

    // GET /account?signer=
    let account = warp::get()
        .and(warp::path!("account"))
        .map(|| format!("Balance of specific order"));

    // POST /account/deposit
    let deposit = warp::post()
        .and(warp::path!("account" / "deposit"))
        .map(|| format!("Depsoiting to account"));

    // POST /account/withdraw
    let withdraw = warp::post()
        .and(warp::path!("account" / "withdraw"))
        .map(|| format!("Withdrawing from account"));

    // POST /account/send
    let send = warp::post()
        .and(warp::path!("account" / "send"))
        .map(|| format!("Sending from account to other account"));

    // POST /order
    let order = warp::post()
        .and(warp::path!("order"))
        .map(|| "Submitting order");

    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
