use crate::accounts::Account;
pub fn account_info(user: &Account) {
    println!("===Account Info===");
    println!("Acc Holder Name: {}", { &user.user_name });
    println!("Acc Number: {}", { &user.acc_no });
    println!("Acc Balance: {}", { &user.balance })
}
