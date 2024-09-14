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

    fn deposit(&mut self, amount: u32) -> u32 {
        self.balance += amount;
        return self.balance;
    }

    fn withdraw(&mut self, amount: u32) -> u32 {
        self.balance -= amount;
        return self.balance;
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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn main() {
    let mut account = Account::new("Dora".to_owned(), 1000);
    let mut bank = Bank::new();
    account.deposit(100);
    account.withdraw(50);
    bank.add_account(account);

    println!("Bank: {:#?}", bank);
}
