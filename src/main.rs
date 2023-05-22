mod accounting;
mod errors;
mod tx;
use crate::accounting::Accounts;
use std::{io, process};

fn read_from_stdin(label: &str) -> String {
    println!("{label}");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Sorry please try again");

    user_input.trim().to_owned()
}

fn process_actions(mut ledger: Accounts, action: &str) {
    match action {
        "deposit" | "DEPOSIT" => {
            let signer = read_from_stdin("What is the signer account name?");
            let amount = read_from_stdin("What is the amount?")
                .parse()
                .expect("Please input a valid number");

            match ledger.deposit(&signer, amount) {
                Ok(tx) => {
                    println!("Depositing {} for {}: {:?}", amount, signer, tx)
                }
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            };
        }
        "withdraw" | "WITHDRAW" => {
            let signer = read_from_stdin("What is the signer account name?");
            let amount = read_from_stdin("What is the amount?")
                .parse()
                .expect("Please input a valid number");

            match ledger.withdraw(&signer, amount) {
                Ok(tx) => {
                    println!("Withdrawing {} from {}: {:?}", amount, signer, tx)
                }
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            };
        }
        "send" | "SEND" => {
            let sender = read_from_stdin("What is the sender account name?");
            let recipient = read_from_stdin("What is the recipient account name?");
            let amount = read_from_stdin("What is the amount?")
                .parse()
                .expect("Please input a valid number");

            match ledger.send(&sender, &recipient.trim(), amount) {
                Ok(tx) => {
                    println!(
                        "Sending {} from {} to {}: {:?}",
                        amount, sender, recipient, tx
                    )
                }
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            };
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

fn main() {
    let ledger = Accounts::new();
    loop {
        let input = read_from_stdin("What do you want to do?");
        println!("Processing the {} action", input);

        // "Remove the ledger clone later on(use reference instead!!!!!"
        process_actions(ledger.clone(), &input)
    }
}
