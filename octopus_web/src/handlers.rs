use octopus_engine::{
    engine::MatchingEngine,
    errors::AccountError,
    trading_platform::TradingPlatform,
    types::{
        AccountArgs, DepositArgs, MatchArgs, MatchResponse, Order, OrderArgs, SendArgs,
        WithdrawArgs,
    },
};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{reject::Reject, Rejection, Reply};

#[derive(Debug, Serialize)]
pub struct OctopusError(AccountError);

impl Reject for OctopusError {}

#[derive(Debug, Serialize)]
struct ServerError(String);

// Custom rejection handler that maps rejections into responses.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if let Some(e) = err.find::<OctopusError>() {
        Ok(warp::reply::with_status(
            warp::reply::json(e),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&ServerError("Server error".to_string())),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

// GET /
pub async fn status() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_status(
        "Up".to_string(),
        warp::http::StatusCode::OK,
    ))
}

// GET /orderbook
pub async fn orderbook(platform: Arc<Mutex<TradingPlatform>>) -> Result<impl Reply, Rejection> {
    let mut p = platform.lock().await;

    Ok(warp::reply::json(&p.orderbook()))
}

// GET /transactions
pub async fn transactions(platform: Arc<Mutex<TradingPlatform>>) -> Result<impl Reply, Rejection> {
    let p = platform.lock().await;

    Ok(warp::reply::json(&p.transactions))
}

// GET /account?signer=
pub async fn account(
    args: AccountArgs,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl Reply, Rejection> {
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
) -> Result<impl Reply, Rejection> {
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
) -> Result<impl Reply, Rejection> {
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
) -> Result<impl Reply, Rejection> {
    let mut p = platform.lock().await;

    match p.send(&args.signer, &args.recipient, args.amount) {
        Ok(tx) => Ok(warp::reply::json(&tx)),
        Err(e) => Err(warp::reject::custom(OctopusError(e))),
    }
}

// POST /submit_order
pub async fn submit_order(
    args: OrderArgs,
    platform: Arc<Mutex<TradingPlatform>>,
) -> Result<impl Reply, Rejection> {
    let mut p = platform.lock().await;
    let order = Order {
        signer: args.signer,
        price: args.price,
        amount: args.amount,
        side: args.side,
    };

    match p.submit_order(order) {
        Ok(receipt) => Ok(warp::reply::json(&receipt)),
        Err(e) => Err(warp::reject::custom(OctopusError(e))),
    }
}

// POST /match_order
pub async fn match_order(args: MatchArgs) -> Result<impl Reply, Rejection> {
    let mut engine = MatchingEngine::new_with_orderbook(args.asks, args.bids);

    match engine.process(args.order) {
        Ok(receipt) => {
            let body = MatchResponse {
                receipt,
                orderbook: engine.vectorised_orderbook(),
            };

            Ok(warp::reply::json(&body))
        }
        Err(e) => Err(warp::reject::custom(OctopusError(e))),
    }
}
