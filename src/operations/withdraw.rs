use crate::accounts::Account;
use std::io;

pub fn withdraw(user: &mut Account) {
    let mut amount = String::new();
    println!("Plz enter an amount to withdraw");
    io::stdin()
        .read_line(&mut amount)
        .expect("Plz enter the correct value");
    let amount: u128 = amount.trim().parse().expect("Invalid value");
    if amount > user.balance {
        println!("Insufficient Balance");
        return;
    }
    user.balance -= amount;
    println!("{}$ withdrawn successfully", amount);
}
