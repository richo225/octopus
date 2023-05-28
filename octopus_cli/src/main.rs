use octopus_common::{
    tx::Tx,
    types::{DepositArgs, OrderArgs, PartialOrder, Receipt, SendArgs, Side, WithdrawArgs},
};
use std::{error::Error, io, process};

fn main() {
    let client = reqwest::blocking::Client::new();

    loop {
        let input = read_from_stdin(
            "Select operation:
                -> deposit
                -> withdraw
                -> send
                -> submit_order
                -> orderbook
                -> account
                -> txlog
                -> quit",
        );
        process_actions(&client, &input)
    }
}

fn read_from_stdin(label: &str) -> String {
    println!("{label}");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Sorry please try again");

    user_input.trim().to_owned()
}

fn process_actions(client: &reqwest::blocking::Client, action: &str) {
    match action {
        "deposit" | "DEPOSIT" => match deposit(client) {
            Ok(tx) => {
                println!("Deposit successful");
                println!("{:?}", tx)
            }
            Err(e) => eprintln!("Something went wrong: {:?}", e),
        },
        "withdraw" | "WITHDRAW" => {
            let signer = read_from_stdin("What is the signer account name?");
            let amount = read_from_stdin("What is the amount?")
                .parse()
                .expect("Please input a valid number");

            println!("Withdrawing {} from {}", amount, signer);

            let body = WithdrawArgs { signer, amount };

            let response = client
                .post("http://localhost:8080/account/withdraw")
                .json(&body)
                .send();

            match response {
                Ok(res) => {
                    println!("Withdraw successfull!");
                    match res.json::<Tx>() {
                        Ok(tx) => println!("{:?}", tx),
                        Err(e) => eprintln!("Something went wrong: {:?}", e),
                    }
                }
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            }
        }
        "send" | "SEND" => {
            let signer = read_from_stdin("What is the sender account name?");
            let recipient = read_from_stdin("What is the recipient account name?");
            let amount = read_from_stdin("What is the amount?")
                .parse()
                .expect("Please input a valid number");

            println!("Sending {} from {} to {}", amount, signer, recipient);

            let body = SendArgs {
                signer,
                amount,
                recipient,
            };

            let response = client
                .post("http://localhost:8080/account/send")
                .json(&body)
                .send();

            match response {
                Ok(res) => {
                    println!("Send successfull!");
                    match res.json::<Tx>() {
                        Ok(tx) => println!("{:?}", tx),
                        Err(e) => eprintln!("Something went wrong: {:?}", e),
                    }
                }
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            }
        }
        "submit_order" => {
            println!("Please provide the following order details:");
            let signer: String = read_from_stdin("What is your account name?");

            let side: Side =
                match read_from_stdin("What is the order type? Buy/Sell? (default is Sell)")
                    .as_str()
                {
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

            println!("Submitting order...");

            let body = OrderArgs {
                price,
                amount,
                side,
                signer,
            };

            let response = client
                .post("http://localhost:8080/order")
                .json(&body)
                .send();

            match response {
                Ok(res) => {
                    println!("Order submitted successfully! Your receipt is below:");
                    match res.json::<Receipt>() {
                        Ok(receipt) => println!("{:?}", receipt),
                        Err(e) => eprintln!("Something went wrong: {:?}", e),
                    }
                }
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            }
        }
        "orderbook" => {
            println!("Printing orderbook....");
            let response = client.get("http://localhost:8080/orderbook").send();

            match response {
                Ok(res) => match res.json::<Vec<PartialOrder>>() {
                    Ok(orderbook) => orderbook.iter().for_each(|po| println!("{:?}", po)),
                    Err(e) => eprintln!("Something went wrong: {:?}", e),
                },
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            }
        }
        "account" => {
            let signer = read_from_stdin("What is the account name?");

            println!("Checking account balance....");
            let response = client
                .get("http://localhost:8080/account")
                .query(&[("signer", &signer)])
                .send();

            match response {
                Ok(res) => match res.json::<u64>() {
                    Ok(balance) => println!("{balance}"),
                    Err(e) => eprintln!("Something went wrong: {:?}", e),
                },
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            }
        }
        "txlog" => {
            println!("Printing txlog....");
            let response = client.get("http://localhost:8080/transactions").send();

            match response {
                Ok(res) => match res.json::<Vec<Tx>>() {
                    Ok(transactions) => transactions.iter().for_each(|tx| println!("{:?}", tx)),
                    Err(e) => eprintln!("Something went wrong: {:?}", e),
                },
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            }
        }
        "quit" => {
            println!("Exiting program....");
            process::exit(1);
        }
        &_ => {
            eprintln!("Invalid action: {:?}", action)
        }
    }
}

fn deposit(client: &reqwest::blocking::Client) -> Result<Tx, Box<dyn Error>> {
    let signer = read_from_stdin("What is the signer account name?");
    let amount = read_from_stdin("What is the amount?")
        .parse()
        .expect("Please input a valid number");

    println!("Depositing {} to {}", &amount, &signer);

    let body = DepositArgs { signer, amount };

    let response = client
        .post("http://localhost:8080/account/deposit")
        .json(&body)
        .send()?
        .json::<Tx>()?;

    Ok(response)
}
