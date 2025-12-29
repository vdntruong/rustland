use std::fmt::format;

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    // Associated function
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        // format macro is a macro, not a function, and we used ! to call it
        format!("{} has a balance {}", self.holder, self.balance)
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account);
}

// print_account take ownership of an account
// fn print_account(account: Account) {
//     println!("{:#?}", account)
// }

// Option 1: Manually move values back and forth between different owners
// fn print_account(account: Account) -> Account {
//     println!("{:#?}", account);
//     account
// }

// Option 2: Borrow an account
// Refs allow us to look at a value without taking ownership of it
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_num_account(bank: &Bank) {
    println!("Number of accounts: {}", bank.accounts.len());
}

impl Bank {
    // Associated function
    fn new() -> Self {
        Bank {
            accounts: Vec::new(),
        }
    }

    // Instance method
    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    // Instance method
    fn print_num_account(&self) {
        println!("Number of accounts: {}", self.accounts.len());
    }

    fn total_balance(&self) -> i32 {
        // iter is a method that returns an iterator of the vector
        // map is a method like lambda function in C#
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    // option 1
    // let mut account = Account::new(1, String::from("Pete"));
    // account = print_account(account);
    // account = print_account(account);

    // option2
    let account = Account::new(1, String::from("Pete"));
    // & operator being used on a owner of a value
    let account_ref = &account;

    // let other_account = account; Rust not allow to move ownership of a value to another variable when it is being borrowed

    print_account(account_ref);
    print_account(&account);
    println!("{:#?}", account);

    let mut bank = Bank::new();
    let mut account1 = Account::new(1, String::from("Pete"));
    account1.deposit(100);
    account1.withdraw(50);
    // println is a macro as well
    println!("{}", account1.summary());

    let account2 = Account::new(2, String::from("David"));
    bank.add_account(account1);
    bank.add_account(account2);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
    // println!("{:#?}", bank);

    // bank.accounts.push(account1);
    // bank.accounts.push(account2);
    bank.print_num_account();
}
