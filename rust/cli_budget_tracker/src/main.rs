use std::io::Write;
use std::io;

enum Category {
    Salary,
    Groceries,
    Rent,
    Personal,
    Other,
}

struct Transaction {
    amount: i64,
    category: Category,
    desc: String,
}

struct Account {
    holder: String, //name of the account holder
    history: Vec<Transaction>,
    balance: i64,
}

impl Category {
    fn as_string(&self) -> String {
        use Category::*;
        match self {
            Salary => String::from("Salary"),
            Groceries => String::from("Groceries"),
            Rent => String::from("Rent"),
            Personal => String::from("Personal"),
            Other => String::from("Other"),
        }
    }
}

impl Transaction {
    fn new(amount: i64, category: Category, desc: &str) -> Transaction {
        let desc = desc.to_string();
        Transaction {
            amount,
            category,
            desc
        }
    }
}

impl Account {
    fn new(holder: &str) -> Account {
        let holder = holder.to_string();
        Account {
            holder,
            history: Vec::new(),
            balance: 0,
        }
    }

    fn add_transaction(&mut self, t: Transaction) {
        self.balance += t.amount;
        self.history.push(t);
    }

    fn make_transaction(&mut self, amount: i64, category: Category, desc: &str) {
        let t = Transaction::new(amount, category, desc);
        self.add_transaction(t);
    }

    fn check_history(&self) {
        println!();
        if self.history.is_empty() {
            println!("{} has no transactions!", self.holder);
            return;
        }

        println!("{}'s transaction history:", self.holder);
        for t in &self.history {
            println!("Amount: {}  --  Category: {}  --  Description: {}",
                     t.amount, t.category.as_string(), t.desc);
        }
    }

    fn check_balance(&self) {
        println!("\n{}'s balance: {}", self.holder, self.balance);
    }

    fn delete_transaction(&mut self, i: usize) {
        self.history.remove(i);
    }
}

enum Command {
    Add,
    Remove,
    History,
    Balance,
    Exit,
}

fn main() {
    println!("~~~~~Budget Tracker~~~~~\nEnter Q at any point to quit\n");
    let Some(mut acc) = init_account() else { return };
    println!("\n=====[{}'s Account]=====", acc.holder);
    loop {
        let command = get_command();
        let success = match command {
            Command::Add => create_add_transaction(&mut acc),
            Command::Remove => remove_transaction(&mut acc),
            Command::History => check_history(&acc),
            Command::Balance => check_balance(&acc),
            Command::Exit => false,
        };
        if !success { break; }
    }
    println!("\nExiting program");
}

fn create_add_transaction(acc: &mut Account) -> bool {
    let amount = loop {
        let temp = get_input("Type the amount: ");
        let temp = temp.trim();
        if temp == "q" || temp == "Q" { return false; }
        match temp.parse::<i64>() {
            Ok(n) => break n,
            _ => println!("Invalid input, try again"),
        }
    };

    let category = loop {
        println!("\nA category is one of:");
        println!("1. Salary");
        println!("2. Groceries");
        println!("3. Rent");
        println!("4. Personal");
        println!("5. Other");
        let temp = get_input("Input the category: ").to_lowercase();
        let temp = temp.trim();
        if temp == "q" { return false; }
        match temp {
            "1" | "salary" | "sal" => break Category::Salary,
            "2" | "groceries" => break Category::Groceries,
            "3" | "rent" => break Category::Rent,
            "4" | "personal" => break Category::Personal,
            "5" | "other" => break Category::Other,
            _ => println!("Invalid input, try again"),
        }
    };

    let desc = get_input("Input a description for this transaction: ");
    let desc = desc.trim();
    if desc == "q" || desc == "Q" { return false; }
    acc.make_transaction(amount, category, desc);
    println!("Transaction added successfully\n");
    true
}

fn remove_transaction(acc: &mut Account) -> bool {
    let max_id = acc.history.len();
    let i: usize = loop {
        println!("Which transaction do you wish to remove?");
        for (i, t) in acc.history.iter().enumerate() {
            println!("[Trans ID = {}] Amount: {} | Category: {} | Description: {}",
                     i, t.amount, t.category.as_string(), t.desc);
        }
        let idx = get_input("Enter a valid ID (Q to exit program, C to cancel selection): ").to_lowercase();
        let idx = idx.trim();
        if idx == "q" { return false; }
        if idx == "c" { return true; }
        let Ok(idx) = idx.parse::<usize>() else {
            println!("Invalid input, try again");
            continue;
        };
        if idx >= max_id {
            println!("Input must be a valid ID (number between 0 and {})", max_id);
        } else {
            break idx;
        }
    };
    acc.delete_transaction(i);
    true
}

fn check_history(acc: &Account) -> bool {
    acc.check_history();
    true
}

fn check_balance(acc: &Account) -> bool {
    acc.check_balance();
    true
}

fn get_command() -> Command {
    loop {
        println!("MENU OPTIONS");
        println!("1. Add a transaction");
        println!("2. Remove a transaction");
        println!("3. Check transaction history");
        println!("4. Check current balance");
        println!("Q. Exit program");
        let comm = get_input("Input an option: ");
        match comm.to_lowercase().trim() {
            "1" | "add" => return Command::Add,
            "2" | "remove" | "rm" => return Command::Remove,
            "3" | "check" | "history" | "his" | "hist" => return Command::History,
            "4" | "balance" => return Command::Balance,
            "q" | "exit" => return Command::Exit,
            _ => println!("Invalid input, try again\n"),
        }
    }
}

fn init_account() -> Option<Account> {
    loop {
        let name = get_input("Enter the name of the account holder: ");
        let name = name.trim();
        match name {
            "" => println!("Name cannot be empty"),
            "q" => return None,
            _ => return Some(Account::new(name)),
        }
    }
}

fn get_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Error reading line");
    temp
}

/*
fn old_main_tests() {
    println!("~~~~~Budget Tracker~~~~~");
    let mut yuis_account = Account::new("Yui");
    let t1 = Transaction::new(100, Category::Salary,"Received check for 100 euros");
    yuis_account.add_transaction(t1);
    yuis_account.make_transaction(-10, Category::Personal, "Sent money to friend");
    yuis_account.check_history();
    yuis_account.check_balance();
}*/