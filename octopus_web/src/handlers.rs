use crate::{core::Side, trading_platform::TradingPlatform};
use octopus_common::{errors::AccountError, types::Order};

use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::reject::Reject;

#[derive(Debug)]
pub struct OctopusError(AccountError);

impl Reject for OctopusError {}

#[derive(Deserialize)]
pub struct AccountArgs {
    signer: String,
}

#[derive(Deserialize)]
pub struct DepositArgs {
    signer: String,
    amount: u64,
}

#[derive(Deserialize)]
pub struct WithdrawArgs {
    signer: String,
    amount: u64,
}

#[derive(Deserialize)]
pub struct SendArgs {
    signer: String,
    recipient: String,
    amount: u64,
}

#[derive(Deserialize)]
pub struct OrderArgs {
    signer: String,
    side: Side,
    amount: u64,
    price: u64,
}

// GET /hello
pub async fn hello() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("Hello there!!!!!".to_string())
}

// GET /orderbook
pub async fn orderbook(
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut p = platform.lock().await;

    Ok(warp::reply::json(&p.orderbook()))
}

// GET /account?signer=
pub async fn account(
    args: AccountArgs,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut p = platform.lock().await;

    match p.balance_of(&args.signer) {
        Ok(balance) => Ok(warp::reply::json(&balance)),
        Err(e) => Err(warp::reject::custom(OctopusError(e))),
    }
}

// POST /account/deposit
pub async fn deposit(
    args: DepositArgs,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut p = platform.lock().await;

    match p.deposit(&args.signer, args.amount) {
        Ok(tx) => Ok(warp::reply::json(&tx)),
        Err(e) => Err(warp::reject::custom(OctopusError(e))),
    }
}

// POST /account/withdraw
pub async fn withdraw(
    args: WithdrawArgs,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut p = platform.lock().await;

    match p.withdraw(&args.signer, args.amount) {
        Ok(tx) => Ok(warp::reply::json(&tx)),
        Err(e) => Err(warp::reject::custom(OctopusError(e))),
    }
}

// POST /account/send
pub async fn send(
    args: SendArgs,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut p = platform.lock().await;

    match p.send(&args.signer, &args.recipient, args.amount) {
        Ok(tx) => Ok(warp::reply::json(&tx)),
        Err(e) => Err(warp::reject::custom(OctopusError(e))),
    }
}

// POST /order
pub async fn order(
    args: OrderArgs,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut p = platform.lock().await;
    let order = Order {
        signer: args.signer,
        price: args.price,
        amount: args.amount,
        side: args.side,
    };

    match p.order(order) {
        Ok(receipt) => Ok(warp::reply::json(&receipt)),
        Err(e) => Err(warp::reject::custom(OctopusError(e))),
    }
}
