use crate::accounts::Account;
use std::io;
pub fn transfer(accounts: &mut [Account], from_index: usize) {
    let mut to_account = String::new();
    let mut amount = String::new();

    println!("Enter the account number to transfer to:");
    io::stdin()
        .read_line(&mut to_account)
        .expect("Failed to read input");
    let to_account = to_account.trim();

    let to_index = accounts
        .iter()
        .position(|account| account.acc_no == to_account);
    match to_index {
        Some(to_index) => {
            println!("Enter the amount to transfer:");
            io::stdin()
                .read_line(&mut amount)
                .expect("Failed to read input");
            let amount: u64 = match amount.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("{}", "Error: Please enter a valid amount!");
                    return;
                }
            };

            if accounts[from_index].balance < amount {
                eprintln!("Insufficient funds.");
                return;
            }

            accounts[from_index].balance -= amount;
            accounts[to_index].balance += amount;

            let transaction = format!("Transferred: ${} to account {}", amount, to_account);
            accounts[from_index].transactions.push(transaction.clone());
            accounts[to_index].transactions.push(transaction);
            println!("Transferred ${} to account {}", amount, to_account);
        }
        None => {
            eprintln!("Account not found.");
        }
    }
}
