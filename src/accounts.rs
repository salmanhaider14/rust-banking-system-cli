use csv::Writer;

use crate::utils::csv_handler::load_users;
use crate::utils::random_code::generate_random_16_digit_code;
use std::fs::OpenOptions;
use std::io;
pub struct Account {
    pub user_name: String,
    pub user_pass: String,
    pub acc_no: String,
    pub balance: u128,
}
impl Account {
    pub fn login(accounts: &Vec<Self>) -> Option<usize> {
        println!("Login");

        let mut name = String::new();
        let mut pass = String::new();

        println!("Enter your name:");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read name");
        name = name.trim().to_string();

        println!("Enter your password:");
        io::stdin()
            .read_line(&mut pass)
            .expect("Failed to read password");
        pass = pass.trim().to_string();

        for (index, account) in accounts.iter().enumerate() {
            if account.user_name == name && account.user_pass == pass {
                println!("Login successful!");
                return Some(index);
            }
        }
        None
    }
    pub fn register() -> Vec<Self> {
        let mut name = String::new();
        let mut pass = String::new();

        println!("Enter your name:");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read name");
        name = name.trim().to_string();

        println!("Enter your password:");
        io::stdin()
            .read_line(&mut pass)
            .expect("Failed to read password");
        pass = pass.trim().to_string();

        let account_num = generate_random_16_digit_code();

        // Open the file in append mode, create if it doesn't exist
        let file_path = "users.csv";
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)
            .expect("Unable to open or create file");

        let mut wtr = Writer::from_writer(file);

        // Write the header if the file is empty
        let metadata = std::fs::metadata(file_path).expect("Unable to read file metadata");
        if metadata.len() == 0 {
            wtr.write_record(&["Name", "Password", "Account Number", "Balance"])
                .expect("Failed to write header");
        }

        // Write the user's data
        wtr.write_record(&[name, pass, account_num, "100".to_string()])
            .expect("Failed to write record");

        wtr.flush().expect("Failed to flush writer");
        println!(" ");
        println!("User registered successfully.");
        println!("An initial deposit of 100$ was made");
        println!(" ");
        load_users()
    }
}
