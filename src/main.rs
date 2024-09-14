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

    fn summary(&self) -> String {
        let message = format!("Hi {}, your balance is {}", self.holder, self.balance);
        return message;
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

    fn total_balance(&self) -> u32 {
        return self.accounts.iter()
            .map(|account| account.balance)
            .sum();
    }

    fn summary(&self) -> Vec<String> {
        return self.accounts.iter()
            .map(|account| format!("{}: {}", account.holder, account.balance))
            .collect();
    }
}

fn main() {
    let account = Account::new("Dora".to_owned(), 1000);
    let account2 = Account::new("Tom".to_owned(), 500);
    let mut bank = Bank::new();
    bank.add_account(account);
    bank.add_account(account2);

    println!("Bank: {:#?}", bank.summary());
}
