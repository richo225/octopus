mod accounting;
mod core;
mod handlers;
mod trading_platform;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use handlers::*;
use trading_platform::TradingPlatform;

use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("starting up");

    let trading_platform = Arc::new(Mutex::new(TradingPlatform::new()));
    let trading_platform_state = warp::any().map(move || trading_platform.clone());

    // GET /hello
    let hello = warp::get().and(warp::path!("hello")).and_then(hello);

    // GET /orderbook
    let orderbook = warp::get()
        .and(warp::path!("orderbook"))
        .and(trading_platform_state.clone())
        .and_then(orderbook);

    // GET /transactions
    let transactions = warp::get()
        .and(warp::path!("transactions"))
        .and(trading_platform_state.clone())
        .and_then(transactions);

    // GET /account?signer=
    let account = warp::get()
        .and(warp::path!("account"))
        .and(warp::query::query())
        .and(trading_platform_state.clone())
        .and_then(account);

    // POST /account/deposit
    let deposit = warp::post()
        .and(warp::path!("account" / "deposit"))
        .and(warp::body::json())
        .and(trading_platform_state.clone())
        .and_then(deposit);

    // POST /account/withdraw
    let withdraw = warp::post()
        .and(warp::path!("account" / "withdraw"))
        .and(warp::body::json())
        .and(trading_platform_state.clone())
        .and_then(withdraw);

    // POST /account/send
    let send = warp::post()
        .and(warp::path!("account" / "send"))
        .and(warp::body::json())
        .and(trading_platform_state.clone())
        .and_then(send);

    // POST /submit_order
    let submit_order = warp::post()
        .and(warp::path!("submit_order"))
        .and(warp::body::json())
        .and(trading_platform_state.clone())
        .and_then(submit_order);

    // POST /match_order
    let match_order = warp::post()
        .and(warp::path!("match_order"))
        .and(warp::body::json())
        .and_then(match_order);

    let routes = hello
        .or(orderbook)
        .or(transactions)
        .or(account)
        .or(deposit)
        .or(withdraw)
        .or(send)
        .or(submit_order)
        .or(match_order);

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
