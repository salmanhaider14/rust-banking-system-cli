use banking_system::accounts::Account;
use banking_system::operations::{
    acc_info::{account_info, transactions_history},
    deposit::deposit,
    transfer::transfer,
    withdraw::withdraw,
};
use banking_system::utils::csv_handler::{load_users, update_data};
use std::io;

fn main() {
    let mut accounts = load_users();
    println!("============ Welcome To Our Bank Account Management System ============");
    loop {
        let mut selected_option: String = String::new();
        println!("1.Login");
        println!("2.Register");
        println!("3.Exit");
        println!(" ");
        println!("Please select an option:");

        io::stdin()
            .read_line(&mut selected_option)
            .expect("Failed to read input");

        // Handle invalid input
        let selected_option: u8 = match selected_option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid value, please enter a number between 1 and 3");
                continue;
            }
        };

        match selected_option {
            1 => {
                if let Some(user_index) = Account::login(&accounts) {
                    main_menu(&mut accounts, user_index);
                } else {
                    println!("Invalid username or password");
                }
            }
            2 => accounts = Account::register(),
            3 => break,
            _ => println!("Please select a number between 1 and 3"),
        }
    }
}

fn main_menu(accounts: &mut [Account], user_index: usize) {
    loop {
        let mut selected_option: String = String::new();
        println!("=========== Main Menu ===========");
        println!(" ");
        println!("1.Deposit");
        println!("2.Withdraw");
        println!("3.Account Info");
        println!("4.Transactions History");
        println!("5.Transfer Funds");
        println!("6.Main Menu");
        println!(" ");
        println!("Please select an option:");

        io::stdin()
            .read_line(&mut selected_option)
            .expect("Failed to read input");
        // Handle invalid input
        let selected_option: u8 = match selected_option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid value, please enter a number between 1 and 6");
                continue;
            }
        };

        match selected_option {
            1 => deposit(&mut accounts[user_index]),
            2 => withdraw(&mut accounts[user_index]),
            3 => account_info(&accounts[user_index]),
            4 => transactions_history(&accounts[user_index]),
            5 => transfer(accounts, user_index),
            6 => break,
            _ => println!("Please select a number between 1 and 4"),
        }
        update_data(accounts);
    }
}
