use std::u32;

#[derive(Debug)]
struct Account {
    id: i32,
    holder: String,
    balance: u32
}

impl Account {
    fn new(holder: String, balance: u32) -> Self {
        return Account {
            id: 1,
            holder,
            balance
        };
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>
}

impl Bank {
    fn new() -> Bank {
        return Bank {
            accounts: vec![]
        }
    }
}

fn main() {
    let account = Account::new("test".to_owned(), 1000);
    let bank = Bank::new();

    println!("Bank: {:?}, Account: {:?}", bank, account);
}
