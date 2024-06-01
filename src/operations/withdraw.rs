use crate::accounts::Account;
use chrono::Local;
use std::io;
pub fn withdraw(user: &mut Account) {
    loop {
        let mut amount = String::new();
        println!("Plz enter an amount to withdraw");
        io::stdin()
            .read_line(&mut amount)
            .expect("Plz enter the correct value");
        let amount: u64 = match amount.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Errror : Plz enter correct amount!");
                continue;
            }
        };
        if amount > user.balance {
            println!("Insufficient Balance");
            return;
        }
        user.balance -= amount;
        let now = Local::now();
        let transaction = format!(
            "Withdrawn: ${} on {}",
            amount,
            now.format("%Y-%m-%d %H:%M:%S")
        );
        user.transactions.push(transaction);
        println!("{}$ withdrawn successfully", amount);
        break;
    }
}
