mod operations;
mod table;

use octopus_common::tx::Tx;
use operations::*;
use reqwest::Url;
use std::{env, process};
use table::*;
use yansi::Color::{Cyan, Green, Red, RGB};

const DEFAULT_HOST: &str = "https://octopus-web.up.railway.app";

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = args.into_iter().nth(1).unwrap_or(DEFAULT_HOST.to_string());

    let host = Url::parse(&url).expect("Please input a valid url");
    let client = reqwest::blocking::Client::new();

    print_welcome();

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
        process_actions(&client, &host, &input)
    }
}

fn process_actions(client: &reqwest::blocking::Client, host: &Url, action: &str) {
    let success = yansi::Style::new(Green).italic().underline();
    let alert = yansi::Style::new(Red).italic();

    match action {
        "deposit" | "DEPOSIT" => match deposit(client, host) {
            Ok(tx) => {
                println!("{}", success.paint("Deposit successful"));
                print_tx_table(tx);
            }
            Err(e) => eprintln!(
                "{}: {:?}",
                alert.paint("Something went wrong"),
                alert.paint(e)
            ),
        },
        "withdraw" | "WITHDRAW" => match withdraw(client, host) {
            Ok(tx) => {
                println!("{}", success.paint("Withdraw successful"));
                print_tx_table(tx);
            }
            Err(e) => eprintln!(
                "{}: {:?}",
                alert.paint("Something went wrong"),
                alert.paint(e)
            ),
        },
        "send" | "SEND" => match send(client, host) {
            Ok(tx) => {
                println!("{}", success.paint("Send successful"));
                print_send_table(tx);
            }
            Err(e) => eprintln!(
                "{}: {:?}",
                alert.paint("Something went wrong"),
                alert.paint(e)
            ),
        },
        "submit_order" | "SUBMIT_ORDER" => match submit_order(client, host) {
            Ok(receipt) => {
                println!(
                    "{}",
                    success.paint("Order submitted successfully! Your receipt is below:")
                );
                println!("{:?}", receipt)
            }
            Err(e) => eprintln!(
                "{}: {:?}",
                alert.paint("Something went wrong"),
                alert.paint(e)
            ),
        },
        "orderbook" | "ORDERBOOK" => match orderbook(client, host) {
            Ok(orderbook) => orderbook.iter().for_each(|po| println!("{:?}", po)),
            Err(e) => eprintln!(
                "{}: {:?}",
                alert.paint("Something went wrong"),
                alert.paint(e)
            ),
        },
        "account" | "ACCOUNT" => match account(client, host) {
            Ok(balance) => {
                println!("{}", Cyan.paint(balance))
            }
            Err(e) => eprintln!(
                "{}: {:?}",
                alert.paint("Something went wrong"),
                alert.paint(e)
            ),
        },
        "txlog" | "TXLOG" => match txlog(client, host) {
            Ok(txs) => txs.iter().for_each(|tx| println!("{:?}", tx)),
            Err(e) => eprintln!(
                "{}: {:?}",
                alert.paint("Something went wrong"),
                alert.paint(e)
            ),
        },
        "quit" | "QUIT" | "q" | "Q" => {
            println!("{}", Cyan.paint("Exiting program....."));
            process::exit(1);
        }
        &_ => {
            eprintln!(
                "{}: {:?}",
                alert.paint("Invalid action"),
                alert.paint(action)
            )
        }
    }
}

fn print_welcome() {
    let octopus_text = r#"





               _                        
              | |                       
     ___   ___| |_ ___  _ __  _   _ ___ 
    / _ \ / __| __/ _ \| '_ \| | | / __|
   | (_) | (__| || (_) | |_) | |_| \__ \
    \___/ \___|\__\___/| .__/ \__,_|___/
                       | |              
                       |_|   





    "#;

    let octopus_image = r#"
                        ___
                     .-'   `'.
                    /         \
                    |         ;
                    |         |           ___.--,
           _.._     |0) ~ (0) |    _.---'`__.-( (_.
    __.--'`_.. '.__.\    '--. \_.-' ,.--'`     `""`
   ( ,.--'`   ',__ /./;   ;, '.__.'`    __
   _`) )  .---.__.' / |   |\   \__..--""  """--.,_
  `---' .'.''-._.-'`_./  /\ '.  \ _.-~~~````~~~-._`-.__.'
        | |  .' _.-' |  |  \  \  '.               `~---`
         \ \/ .'     \  \   '. '-._)
          \/ /        \  \    `=.__`~-.
          / /\         `) )    / / `"".`\
    , _.-'.'\ \        / /    ( (     / /
     `--~`   ) )    .-'.'      '.'.  | (
            (/`    ( (`          ) )  '-;
             `      '-;         (-'
    "#;

    let left_pad = octopus_image.lines().map(|l| l.len()).max().unwrap_or(0);
    for (octopus_image, octopus_text) in octopus_image.lines().zip(octopus_text.lines()) {
        println!(
            "{:width$} {}",
            RGB(255, 117, 24).paint(octopus_image),
            RGB(255, 117, 24).paint(octopus_text),
            width = left_pad
        );
    }
}
