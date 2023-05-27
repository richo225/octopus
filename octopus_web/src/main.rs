mod accounting;
mod core;
mod trading_platform;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use crate::core::Side;
use octopus_common::{errors, tx};

use serde::Deserialize;
use warp::Filter;

#[derive(Deserialize)]
struct AccountArgs {
    signer: String,
}

#[derive(Deserialize)]
struct DepositArgs {
    signer: String,
    amount: u64,
}

#[derive(Deserialize)]
struct WithdrawArgs {
    signer: String,
    amount: u64,
}

#[derive(Deserialize)]
struct SendArgs {
    signer: String,
    recipient: String,
    amount: u64,
}

#[derive(Deserialize)]
struct OrderArgs {
    signer: String,
    side: Side,
    amount: u64,
    price: u64,
}

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
        .and(warp::query::<AccountArgs>())
        .map(|args: AccountArgs| format!("Balance of specific account for {}", args.signer));

    // POST /account/deposit
    let deposit = warp::post()
        .and(warp::path!("account" / "deposit"))
        .and(warp::query::<DepositArgs>())
        .map(|args: DepositArgs| format!("Depositing {} to account {}", args.amount, args.signer));

    // POST /account/withdraw
    let withdraw = warp::post()
        .and(warp::path!("account" / "withdraw"))
        .and(warp::query::<WithdrawArgs>())
        .map(|args: WithdrawArgs| {
            format!("Withdrawing {} from account {}", args.amount, args.signer)
        });

    // POST /account/send
    let send = warp::post()
        .and(warp::path!("account" / "send"))
        .and(warp::query::<SendArgs>())
        .map(|args: SendArgs| {
            format!(
                "Sending {} from account {} to account {}",
                args.amount, args.signer, args.recipient
            )
        });

    // POST /order
    let order = warp::post()
        .and(warp::path!("order"))
        .and(warp::query::<OrderArgs>())
        .map(|_args: OrderArgs| "Submitting order");

    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
