use crate::accounts::Account;
pub fn account_info(user: &Account) {
    println!("===Account Info===");
    println!("Acc Holder Name: {}", { &user.user_name });
    println!("Acc Number: {}", { &user.acc_no });
    println!("Acc Balance: {}", { &user.balance });
}
pub fn transactions_history(user: &Account) {
    println!("===Transaction History===");
    for transaction in &user.transactions {
        println!("{}", transaction);
    }
}
