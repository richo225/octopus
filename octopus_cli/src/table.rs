use cli_table::{Cell, CellStruct, Style, Table};
use octopus_common::{
    tx::Tx,
    types::{PartialOrder, Receipt, Side},
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

pub fn print_receipt_table(receipt: Receipt) {
    let rows: Vec<Vec<CellStruct>> = receipt
        .matches
        .iter()
        .map(|po: &PartialOrder| {
            vec![
                po.ordinal.cell(),
                match po.side {
                    Side::Buy => Green.paint("BUY").cell(),
                    Side::Sell => Red.paint("sell").cell(),
                },
                po.price.cell(),
                po.amount.cell(),
                po.remaining.cell(),
            ]
        })
        .collect();

    let table = rows
        .table()
        .title(vec![
            "Ordinal".cell().bold(true),
            "Side".cell().bold(true),
            "Price".cell().bold(true),
            "Amount".cell().bold(true),
            "Remaining".cell().bold(true),
        ])
        .bold(true);

    println!("{}", table.display().unwrap());
}

fn generate_tx_row(tx: Tx) -> Vec<CellStruct> {
    match tx {
        Tx::Withdraw { account, amount } => {
            vec![
                Red.paint("WITHDRAW").cell(),
                Cyan.paint(account).cell(),
                Cyan.paint(amount).cell(),
            ]
        }
        Tx::Deposit { account, amount } => {
            vec![
                Green.paint("DEPOSIT").cell(),
                Cyan.paint(account).cell(),
                Cyan.paint(amount).cell(),
            ]
        }
    }
}
