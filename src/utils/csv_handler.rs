use std::{fs::OpenOptions, path::Path};

use crate::accounts::Account;
use csv::{ReaderBuilder, Writer};

pub fn load_users() -> Vec<Account> {
    let file_path = "users.csv";
    if Path::new(&file_path).exists() {
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path)
            .expect("Failed to open file");

        let mut accounts = Vec::new();

        for result in rdr.records() {
            let record = result.expect("Failed to read record");
            let account = Account {
                user_name: record[0].to_string(),
                user_pass: record[1].to_string(),
                acc_no: record[2].to_string(),
                balance: record[3].trim().parse().expect("Invalid Balance"),
            };
            accounts.push(account);
        }

        accounts
    } else {
        Vec::new()
    }
}

pub fn update_data(accounts: &mut [Account]) {
    let file_path = "users.csv";
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open or create file");

    let mut wtr = Writer::from_writer(file);

    // Write the header
    wtr.write_record(&["Name", "Password", "Account Number", "Balance"])
        .expect("Failed to write header");

    for account in accounts {
        wtr.write_record(&[
            &account.user_name,
            &account.user_pass,
            &account.acc_no,
            &account.balance.to_string(),
        ])
        .expect("Failed to write record");
    }

    wtr.flush().expect("Failed to flush writer");
}
