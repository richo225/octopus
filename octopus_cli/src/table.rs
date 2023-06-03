use cli_table::{format::Justify, Cell, CellStruct, Style, Table};
use octopus_common::{
    tx::Tx,
    types::{PartialOrder, Side},
};
use yansi::Color::{Cyan, Green, Red, RGB};

pub fn print_welcome() {
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

pub fn print_tx_table(tx: Tx) {
    let table = vec![generate_tx_row(tx)]
        .table()
        .title(vec![
            "Operation".cell().bold(true),
            "Account".cell().bold(true),
            "Amount".cell().bold(true),
        ])
        .bold(true);

    println!("{}", table.display().unwrap());
}

pub fn print_send_table(tx: (Tx, Tx)) {
    let table = vec![generate_tx_row(tx.0), generate_tx_row(tx.1)]
        .table()
        .title(vec![
            "Operation".cell().bold(true),
            "Account".cell().bold(true),
            "Amount".cell().bold(true),
        ])
        .bold(true);

    println!("{}", table.display().unwrap());
}

pub fn print_txlog_table(txs: Vec<Tx>) {
    let rows: Vec<Vec<CellStruct>> = txs.iter().map(|tx| generate_tx_row(tx.clone())).collect();

    let table = rows
        .table()
        .title(vec![
            "Operation".cell().bold(true),
            "Account".cell().bold(true),
            "Amount".cell().bold(true),
        ])
        .bold(true);

    println!("{}", table.display().unwrap());
}

pub fn print_partial_orders_table(pos: Vec<PartialOrder>) {
    let rows: Vec<Vec<CellStruct>> = pos
        .iter()
        .map(|po: &PartialOrder| {
            vec![
                match po.side {
                    Side::Buy => Green.paint("BUY").cell(),
                    Side::Sell => Red.paint("SELL").cell(),
                },
                Cyan.paint(po.price).cell().justify(Justify::Center),
                Cyan.paint(po.amount).cell().justify(Justify::Center),
                Cyan.paint(po.remaining).cell().justify(Justify::Center),
                Cyan.paint(po.ordinal).cell().justify(Justify::Center),
            ]
        })
        .collect();

    let table = rows
        .table()
        .title(vec![
            "Side".cell().bold(true),
            "Price".cell().bold(true),
            "Amount".cell().bold(true),
            "Remaining".cell().bold(true),
            "Ordinal".cell().bold(true),
        ])
        .bold(true);

    println!("{}", table.display().unwrap());
}

pub fn print_account_table(balance: u64) {
    let table = vec![vec![Cyan.paint(balance).cell().justify(Justify::Center)]]
        .table()
        .title(vec!["Balance".cell().bold(true)])
        .bold(true);

    println!("{}", table.display().unwrap());
}

fn generate_tx_row(tx: Tx) -> Vec<CellStruct> {
    match tx {
        Tx::Withdraw { account, amount } => {
            vec![
                Red.paint("WITHDRAW").cell().justify(Justify::Center),
                Cyan.paint(account).cell().justify(Justify::Center),
                Cyan.paint(amount).cell().justify(Justify::Center),
            ]
        }
        Tx::Deposit { account, amount } => {
            vec![
                Green.paint("DEPOSIT").cell().justify(Justify::Center),
                Cyan.paint(account).cell().justify(Justify::Center),
                Cyan.paint(amount).cell().justify(Justify::Center),
            ]
        }
    }
}
