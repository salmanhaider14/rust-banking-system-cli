use banking_system::accounts::Account;
use banking_system::operations::{acc_info::account_info, deposit::deposit, withdraw::withdraw};
use banking_system::utils::csv_handler::{load_users, update_data};
use std::io;

fn main() {
    let mut accounts = load_users();

    println!("________________Welcom To Our Bank Account Management System________________");
    loop {
        let mut selected_option: String = String::new();
        println!("1.Login");
        println!("2.Register");
        println!("3.Exit");
        println!(" ");
        println!("Please select an option:");

        io::stdin()
            .read_line(&mut selected_option)
            .expect("Plz enter the correct value");

        let selected_option: u8 = selected_option.trim().parse().expect("Invalid value");
        match selected_option {
            1 => {
                if let Some(user_index) = Account::login(&accounts) {
                    main_menu(&mut accounts, user_index);
                } else {
                    println!("Invalid username or password")
                }
            }
            2 => accounts = Account::register(),
            3 => break,
            _ => println!("Please select a number between 1 and 3"),
        }
    }

    // main_menu();
}

pub fn main_menu(accounts: &mut [Account], user_index: usize) {
    loop {
        let mut selected_option: String = String::new();
        println!("________________Main Menu________________");
        println!("1.Deposit");
        println!("2.Withdraw");
        println!("3.Account Info");
        println!("4.Main Menu");
        println!(" ");
        println!("Please select an option:");
        io::stdin()
            .read_line(&mut selected_option)
            .expect("Plz enter the correct value");

        let selected_option: u8 = selected_option.trim().parse().expect("Invalid value");
        match selected_option {
            1 => deposit(&mut accounts[user_index]),
            2 => withdraw(&mut accounts[user_index]),
            3 => account_info(&accounts[user_index]),
            4 => break,
            _ => println!("Please select a value between 1 and 4"),
        }
        update_data(accounts);
    }
}
