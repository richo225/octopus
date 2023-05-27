mod accounting;
mod core;
mod handlers;
mod trading_platform;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("starting up");

    // GET /hello
    let hello = warp::get()
        .and(warp::path!("hello"))
        .and_then(handlers::hello);

    // GET /accounts
    let accounts = warp::get()
        .and(warp::path!("accounts"))
        .and_then(handlers::accounts);

    // GET /orderbook
    let orderbook = warp::get()
        .and(warp::path!("orderbook"))
        .and_then(handlers::orderbook);

    // GET /account?signer=
    let account = warp::get()
        .and(warp::path!("account"))
        .and(warp::body::json())
        .and_then(handlers::account);

    // POST /account/deposit
    let deposit = warp::post()
        .and(warp::path!("account" / "deposit"))
        .and(warp::body::json())
        .and_then(handlers::deposit);

    // POST /account/withdraw
    let withdraw = warp::post()
        .and(warp::path!("account" / "withdraw"))
        .and(warp::body::json())
        .and_then(handlers::withdraw);

    // POST /account/send
    let send = warp::post()
        .and(warp::path!("account" / "send"))
        .and(warp::body::json())
        .and_then(handlers::send);

    // POST /order
    let order = warp::post()
        .and(warp::path!("order"))
        .and(warp::body::json())
        .and_then(handlers::order);

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
