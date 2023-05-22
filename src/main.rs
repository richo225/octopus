mod accounting;
mod errors;
mod tx;
use crate::accounting::Accounts;
use std::io;

fn read_from_stdin(label: &str) -> String {
    println!("{label}");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Sorry please try again");

    return user_input;
}

fn process_actions(mut ledger: accounting::Accounts, action: &str) {
    let signer = read_from_stdin("What is the signer account name?");
    let amount = read_from_stdin("What is the amount?");
    let amount = amount
        .trim()
        .parse::<u64>()
        .expect("Please input a valid number");

    match action {
        "deposit" | "DEPOSIT" => {
            match ledger.deposit(&signer.trim(), amount) {
                Ok(tx) => {
                    println!("Depositing {} for {}: {:?}", amount, signer, tx)
                }
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            };
        }
        "withdraw" | "WITHDRAW" => {
            match ledger.withdraw(&signer.trim(), amount) {
                Ok(tx) => {
                    println!("Withdrawing {} from {}: {:?}", amount, signer, tx)
                }
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            };
        }
        "send" | "SEND" => {
            let recipient = read_from_stdin("What is the recipient account name?");

            match ledger.send(&signer.trim(), &recipient.trim(), amount) {
                Ok(tx) => {
                    println!(
                        "Sending {} from {} to {}: {:?}",
                        amount, signer, recipient, tx
                    )
                }
                Err(e) => eprintln!("Something went wrong: {:?}", e),
            };
        }
        &_ => todo!(),
    }
}

fn main() {
    let ledger = Accounts::new();
    loop {
        let input = read_from_stdin("What do you want to do?");
        let input = input.trim();

        if input.is_empty() {
            println!("Goodbye!");
            break;
        }

        println!("You selected: {}", input);
        // "Remove the ledger clone later on(use reference instead!!!!!"
        // Add transaction history to Accounts struct as a vec[]
        process_actions(ledger.clone(), &input)
    }

    // // Creates the basic ledger and a tx log container
    // let mut ledger = Accounts::new();
    // let mut tx_log = vec![];

    // // Deposit an amount to each account
    // for signer in &[bob, alice, charlie] {
    //     let status = ledger.deposit(*signer, initial_amount);
    //     println!("Depositing {} for {}: {:?}", signer, initial_amount, status);
    //     // Add the resulting transaction to a list of transactions
    //     // .unwrap() will crash the program if the status is an error.
    //     tx_log.push(status.unwrap());
    // }

    // // Send currency from one account (bob) to the other (alice)
    // let send_amount = 10_u64;
    // let status = ledger.send(bob, alice, send_amount);
    // println!(
    //     "Sent {} from {} to {}: {:?}",
    //     send_amount, bob, alice, status
    // );

    // // Add both transactions to the transaction log
    // let (tx1, tx2) = status.unwrap();
    // tx_log.push(tx1);
    // tx_log.push(tx2);

    // // Withdraw everything from the accounts
    // let tx = ledger.withdraw(charlie, initial_amount).unwrap();
    // tx_log.push(tx);
    // let tx = ledger
    //     .withdraw(alice, initial_amount + send_amount)
    //     .unwrap();
    // tx_log.push(tx);

    // // Here we are withdrawing too much and there won't be a transaction
    // println!(
    //     "Withdrawing {} from {}: {:?}",
    //     initial_amount,
    //     bob,
    //     ledger.withdraw(bob, initial_amount)
    // );
    // // Withdrawing the expected amount results in a transaction
    // let tx = ledger.withdraw(bob, initial_amount - send_amount).unwrap();
    // tx_log.push(tx);

    // // {:?} prints the Debug implementation, {:#?} pretty-prints it
    // println!("Ledger empty: {:?}", ledger);
    // println!("The TX log: {:#?}", tx_log);
}
