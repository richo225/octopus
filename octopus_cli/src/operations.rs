use octopus_engine::{
    errors::{OctopusError},
    tx::Tx,
    types::{DepositArgs, OrderArgs, PartialOrder, Receipt, SendArgs, Side, WithdrawArgs},
};
use reqwest::Url;

use std::{error::Error, io};
use yansi::Color::{Blue, Cyan};

pub fn read_from_stdin(label: &str) -> String {
    println!("{}", Blue.paint(label));

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Sorry please try again");

    user_input.trim().to_owned()
}

pub fn deposit(client: &reqwest::blocking::Client, host: &Url) -> Result<Tx, Box<dyn Error>> {
    let signer = read_from_stdin("What is the signer account name?");
    let amount = read_from_stdin("What is the amount?")
        .parse()
        .expect("Please input a valid number");

    println!(
        "Depositing {} to {}",
        Cyan.paint(&amount),
        Cyan.paint(&signer)
    );

    let body = DepositArgs { signer, amount };

    let response = client
        .post(host.join("/account/deposit")?)
        .json(&body)
        .send()?;

    if response.status() == reqwest::StatusCode::OK {
        Ok(response.json::<Tx>()?)
    } else {
        Err(Box::new(response.json::<OctopusError>()?))
    }
}

pub fn withdraw(client: &reqwest::blocking::Client, host: &Url) -> Result<Tx, Box<dyn Error>> {
    let signer = read_from_stdin("What is the signer account name?");
    let amount = read_from_stdin("What is the amount?")
        .parse()
        .expect("Please input a valid number");

    println!(
        "Withdrawing {} from {}",
        Cyan.paint(&amount),
        Cyan.paint(&signer)
    );

    let body = WithdrawArgs { signer, amount };

    let response = client
        .post(host.join("/account/withdraw")?)
        .json(&body)
        .send()?;

    if response.status() == reqwest::StatusCode::OK {
        Ok(response.json::<Tx>()?)
    } else {
        Err(Box::new(response.json::<OctopusError>()?))
    }
}

pub fn send(client: &reqwest::blocking::Client, host: &Url) -> Result<(Tx, Tx), Box<dyn Error>> {
    let signer = read_from_stdin("What is the sender account name?");
    let recipient = read_from_stdin("What is the recipient account name?");
    let amount = read_from_stdin("What is the amount?")
        .parse()
        .expect("Please input a valid number");

    println!(
        "Sending {} from {} to {}",
        Cyan.paint(&amount),
        Cyan.paint(&signer),
        Cyan.paint(&recipient)
    );

    let body = SendArgs {
        signer,
        amount,
        recipient,
    };

    let response = client
        .post(host.join("/account/send")?)
        .json(&body)
        .send()?;

    if response.status() == reqwest::StatusCode::OK {
        Ok(response.json::<(Tx, Tx)>()?)
    } else {
        Err(Box::new(response.json::<OctopusError>()?))
    }
}

pub fn submit_order(
    client: &reqwest::blocking::Client,
    host: &Url,
) -> Result<Receipt, Box<dyn Error>> {
    println!(
        "{}",
        Blue.paint("Please provide the following order details:")
    );
    let signer: String = read_from_stdin("What is your account name?");

    let side: Side =
        match read_from_stdin("What is the order type? Buy/Sell? (default is Sell)").as_str() {
            "buy" | "BUY" => Side::Buy,
            "sell" | "SELL" => Side::Sell,
            &_ => Side::Sell,
        };

    let price: u64 = read_from_stdin("What is the price?")
        .parse()
        .expect("Please input a valid number");

    let amount: u64 = read_from_stdin("What is the amount?")
        .parse()
        .expect("Please input a valid number");

    println!("{}", Cyan.paint("Submitting order....."));

    let body = OrderArgs {
        price,
        amount,
        side,
        signer,
    };

    let response = client
        .post(host.join("/submit_order")?)
        .json(&body)
        .send()?;

    if response.status() == reqwest::StatusCode::OK {
        Ok(response.json::<Receipt>()?)
    } else {
        Err(Box::new(response.json::<OctopusError>()?))
    }
}

pub fn orderbook(
    client: &reqwest::blocking::Client,
    host: &Url,
) -> Result<Vec<PartialOrder>, Box<dyn Error>> {
    println!("{}", Cyan.paint("Printing orderbook....."));

    let response = client.get(host.join("/orderbook")?).send()?;

    if response.status() == reqwest::StatusCode::OK {
        Ok(response.json::<Vec<PartialOrder>>()?)
    } else {
        Err(Box::new(response.json::<OctopusError>()?))
    }
}

pub fn account(client: &reqwest::blocking::Client, host: &Url) -> Result<u64, Box<dyn Error>> {
    let signer = read_from_stdin("What is the account name?");

    println!("{}", Cyan.paint("Checking account balance....."));

    let response = client
        .get(host.join("/account")?)
        .query(&[("signer", &signer)])
        .send()?;

    if response.status() == reqwest::StatusCode::OK {
        Ok(response.json::<u64>()?)
    } else {
        Err(Box::new(response.json::<OctopusError>()?))
    }
}

pub fn txlog(client: &reqwest::blocking::Client, host: &Url) -> Result<Vec<Tx>, Box<dyn Error>> {
    println!("{}", Cyan.paint("Printing txlog....."));

    let response = client.get(host.join("/transactions")?).send()?;

    if response.status() == reqwest::StatusCode::OK {
        Ok(response.json::<Vec<Tx>>()?)
    } else {
        Err(Box::new(response.json::<OctopusError>()?))
    }
}
