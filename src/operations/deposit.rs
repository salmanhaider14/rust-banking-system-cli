use crate::accounts::Account;
use std::io;
pub fn deposit(user: &mut Account) {
    let mut amount = String::new();
    println!("Plz enter an amount to deposit");
    io::stdin()
        .read_line(&mut amount)
        .expect("Plz enter the correct value");
    let amount: u128 = amount.trim().parse().expect("Invalid value");
    user.balance += amount;
    println!("{}$ deposited successfully", amount);
}
