use octopus_common::{tx::Tx, types::DepositArgs};
use std::{io, process};

fn main() {
    // initialise http client here
    let client = reqwest::blocking::Client::new();

    loop {
        let input = read_from_stdin(
            "Select operation:
                -> deposit
                -> withdraw
                -> send
                -> submit_order
                -> orderbook
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
        "deposit" | "DEPOSIT" => {
            let signer = read_from_stdin("What is the signer account name?");
            let amount = read_from_stdin("What is the amount?")
                .parse()
                .expect("Please input a valid number");

            println!("Depositing {} to {}", &amount, &signer);

            let body = DepositArgs { signer, amount };

            let response = client
                .post("http://localhost:8080/account/deposit")
                .json(&body)
                .send();

            match response {
                Ok(res) => {
                    println!("Deposit successfull!");
                    match res.json::<Tx>() {
                        Ok(tx) => println!("{:?}", tx),
                        Err(e) => eprintln!("Something went wrong: {:?}", e),
                    }
                }
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            }
        }
        // "withdraw" | "WITHDRAW" => {
        //     let signer = read_from_stdin("What is the signer account name?");
        //     let amount = read_from_stdin("What is the amount?")
        //         .parse()
        //         .expect("Please input a valid number");

        //     match trading_platform.withdraw(&signer, amount) {
        //         Ok(tx) => {
        //             println!("Withdrawing {} from {}: {:?}", amount, signer, tx)
        //         }
        //         Err(e) => eprintln!("Something went wrong: {:?}", e),
        //     };
        // }
        // "send" | "SEND" => {
        //     let sender = read_from_stdin("What is the sender account name?");
        //     let recipient = read_from_stdin("What is the recipient account name?");
        //     let amount = read_from_stdin("What is the amount?")
        //         .parse()
        //         .expect("Please input a valid number");

        //     match trading_platform.send(&sender, recipient.trim(), amount) {
        //         Ok(tx) => {
        //             println!(
        //                 "Sending {} from {} to {}: {:?}",
        //                 amount, sender, recipient, tx
        //             )
        //         }
        //         Err(e) => eprintln!("Something went wrong: {:?}", e),
        //     };
        // }
        // "submit_order" => {
        //     println!("Please provide the following order details:");
        //     let signer: String = read_from_stdin("What is your account name?");

        //     let side: Side =
        //         match read_from_stdin("What is the order type? Buy/Sell? (default is sell)")
        //             .as_str()
        //         {
        //             "buy" | "BUY" => Side::Buy,
        //             "sell" | "SELL" => Side::Sell,
        //             &_ => Side::Sell,
        //         };

        //     let price: u64 = read_from_stdin("What is the price?")
        //         .parse()
        //         .expect("Please input a valid number");

        //     let amount: u64 = read_from_stdin("What is the amount?")
        //         .parse()
        //         .expect("Please input a valid number");

        //     match trading_platform.order(Order {
        //         price,
        //         amount,
        //         side,
        //         signer,
        //     }) {
        //         Ok(receipt) => {
        //             println!("Order submitted successfully! Your receipt is below:");
        //             println!("{:?}", receipt)
        //         }
        //         Err(e) => eprintln!("Something went wrong: {:?}", e),
        //     }
        // }
        // "orderbook" => {
        //     println!("Printing orderbook....");
        //     trading_platform
        //         .orderbook()
        //         .iter()
        //         .for_each(|po| println!("{:?}", po))
        // }
        // "accounts" => {
        //     println!("Printing accounts....");
        //     trading_platform
        //         .accounts
        //         .accounts
        //         .iter()
        //         .for_each(|acc| println!("{:?}", acc))
        // }
        // "txlog" => {
        //     println!("Printing txlog....");
        //     trading_platform
        //         .transactions
        //         .iter()
        //         .for_each(|tx| println!("{:?}", tx))
        // }
        // "quit" => {
        //     println!("Exiting program....");
        //     process::exit(1);
        // }
        &_ => {
            eprintln!("Invalid action: {:?}", action)
        }
    }
}
