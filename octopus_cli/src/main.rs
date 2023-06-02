use octopus_common::{
    tx::Tx,
    types::{DepositArgs, OrderArgs, PartialOrder, Receipt, SendArgs, Side, WithdrawArgs},
};
use reqwest::Url;
use std::{env, error::Error, io, process};
use yansi::{
    Color::{Blue, Cyan, Green, Red},
    Style,
};

const DEFAULT_HOST: &str = "https://octopus-web.up.railway.app";

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = args.into_iter().nth(1).unwrap_or(DEFAULT_HOST.to_string());

    let host = Url::parse(&url).expect("Please input a valid url");
    let client = reqwest::blocking::Client::new();

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
               .---.         ,,
    ,,        /     \       ;,,'        
   ;, ;      (  o  o )      ; ;
     ;,';,,,  \  \/ /      ,; ;
  ,,,  ;,,,,;;,`   '-,;'''',,,'
 ;,, ;,, ,,,,   ,;  ,,,'';;,,;''';
    ;,,,;    ~~'  '';,,''',,;''''  
                       '''
    "#;

    let left_pad = octopus_image.lines().map(|l| l.len()).max().unwrap_or(0);
    for (octopus_image, octopus_text) in octopus_image.lines().zip(octopus_text.lines()) {
        println!(
            "{:width$} {}",
            octopus_image,
            octopus_text,
            width = left_pad
        );
    }

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

fn read_from_stdin(label: &str) -> String {
    println!("{}", Blue.paint(label));

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Sorry please try again");

    user_input.trim().to_owned()
}

fn process_actions(client: &reqwest::blocking::Client, host: &Url, action: &str) {
    let success = Style::new(Green).italic().underline();
    let alert = Style::new(Red).italic();

    match action {
        "deposit" | "DEPOSIT" => match deposit(client, host) {
            Ok(tx) => {
                println!("{}", success.paint("Deposit successful"));
                println!("{:?}", tx)
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
                println!("{:?}", tx)
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
                println!("{:?}", tx)
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

fn deposit(client: &reqwest::blocking::Client, host: &Url) -> Result<Tx, Box<dyn Error>> {
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

    let response: Tx = client
        .post(host.join("/account/deposit")?)
        .json(&body)
        .send()?
        .json::<Tx>()?;

    Ok(response)
}

fn withdraw(client: &reqwest::blocking::Client, host: &Url) -> Result<Tx, Box<dyn Error>> {
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

    let response: Tx = client
        .post(host.join("/account/withdraw")?)
        .json(&body)
        .send()?
        .json::<Tx>()?;

    Ok(response)
}

fn send(client: &reqwest::blocking::Client, host: &Url) -> Result<(Tx, Tx), Box<dyn Error>> {
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

    let response: (Tx, Tx) = client
        .post(host.join("/account/send")?)
        .json(&body)
        .send()?
        .json::<(Tx, Tx)>()?;

    Ok(response)
}

fn submit_order(client: &reqwest::blocking::Client, host: &Url) -> Result<Receipt, Box<dyn Error>> {
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

    let response: Receipt = client
        .post(host.join("/submit_order")?)
        .json(&body)
        .send()?
        .json::<Receipt>()?;

    Ok(response)
}

fn orderbook(
    client: &reqwest::blocking::Client,
    host: &Url,
) -> Result<Vec<PartialOrder>, Box<dyn Error>> {
    println!("{}", Cyan.paint("Printing orderbook....."));

    let response: Vec<PartialOrder> = client
        .get(host.join("/orderbook")?)
        .send()?
        .json::<Vec<PartialOrder>>()?;

    Ok(response)
}

fn account(client: &reqwest::blocking::Client, host: &Url) -> Result<u64, Box<dyn Error>> {
    let signer = read_from_stdin("What is the account name?");

    println!("{}", Cyan.paint("Checking account balance....."));

    let response: u64 = client
        .get(host.join("/account")?)
        .query(&[("signer", &signer)])
        .send()?
        .json::<u64>()?;

    Ok(response)
}

fn txlog(client: &reqwest::blocking::Client, host: &Url) -> Result<Vec<Tx>, Box<dyn Error>> {
    println!("{}", Cyan.paint("Printing txlog....."));

    let response: Vec<Tx> = client
        .get(host.join("/transactions")?)
        .send()?
        .json::<Vec<Tx>>()?;

    Ok(response)
}
