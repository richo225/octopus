use crate::core::Side;
use octopus_common::errors::AccountError;

use serde::Deserialize;
use warp::reject::Reject;

#[derive(Debug)]
struct OctopusError(AccountError);

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
    Ok(format!("Hello there!!!!!"))
}

// GET /accounts
pub async fn accounts() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("List of accounts"))
}

// GET /orderbook
pub async fn orderbook() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("List of orders"))
}

// GET /account?signer=
pub async fn account(args: AccountArgs) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Balance of specific account for {}", args.signer))
}

// POST /account/deposit
pub async fn deposit(args: DepositArgs) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!(
        "Depositing {} to account {}",
        args.amount, args.signer
    ))
}

// POST /account/withdraw
pub async fn withdraw(args: WithdrawArgs) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!(
        "Withdrawing {} from account {}",
        args.amount, args.signer
    ))
}

// POST /account/send
pub async fn send(args: SendArgs) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!(
        "Sending {} from account {} to account {}",
        args.amount, args.signer, args.recipient
    ))
}

// POST /order
pub async fn order(_args: OrderArgs) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Submitting order"))
}
