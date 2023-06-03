use cli_table::{format::Justify, Cell, CellStruct, Style, Table};
use octopus_common::{
    tx::Tx,
    types::{PartialOrder, Side},
};
use yansi::Color::{Cyan, Green, Red};

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
