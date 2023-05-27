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

    // GET /accounts
    let accounts = warp::get()
        .and(warp::path!("accounts"))
        .and(trading_platform_state.clone())
        .and_then(accounts);

    // GET /orderbook
    let orderbook = warp::get()
        .and(warp::path!("orderbook"))
        .and(trading_platform_state.clone())
        .and_then(orderbook);

    // GET /account?signer=
    let account = warp::get()
        .and(warp::path!("account"))
        .and(warp::body::json())
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

    // POST /order
    let order = warp::post()
        .and(warp::path!("order"))
        .and(warp::body::json())
        .and(trading_platform_state.clone())
        .and_then(order);

    let routes = hello
        .or(accounts)
        .or(orderbook)
        .or(account)
        .or(deposit)
        .or(withdraw)
        .or(send)
        .or(order);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
