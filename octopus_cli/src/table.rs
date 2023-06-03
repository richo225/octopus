use cli_table::{Cell, CellStruct, Style, Table};
use octopus_common::tx::Tx;
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
