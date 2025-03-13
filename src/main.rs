fn main() {
    println!("Hello, world!");
}
extern crate rand;
use rand::Rng;
use std::collections::HashMap;

struct User {
    id: u128,
    name: String,
}

struct Bank {
    id: u128,
    name: String,
}

struct Account {
    id: u128,
    owner_id: u128,
    bank_id: u128,
    balance: f64,
}

struct BankingSystem {
    users: HashMap<u128, User>,
    banks: HashMap<u128, Bank>,
    accounts: HashMap<u128, Account>,
}
impl BankingSystem {
    fn new() -> Self {
        BankingSystem {
            users: HashMap::new(),
            banks: HashMap::new(),
            accounts: HashMap::new(),
        }
    }

    fn create_user(&mut self, id:u128, name:&str){
        self.users.insert(id,Bank{id,name:name.to_string() });
    }

    fn create_account(&mut self, user_id:u128, bank_id:u128, initial_balance:f64){
        let mut rng = rand::thread_rng();
        let account_id = rng.gen_range(1000.9999) as u128;
        self.accounts.insert(account_id, Account { id:account_id, owner_id: user_id, bank_id,balance: initial_balance });
    }

    fn transfer_money(&mut self, from_id:u128, to_id:u128, amount: f64) {
        let from_acc_exists = self.accounts.contains_key(&from_id);
        let to_acc_exists = self.account.contains_key(&to_id);

        if from_acc_exists && to_acc_exists {
            let(from_balance, _to_balance) = {
                let from_acc = self.accounts.get(&from_id)unwrap();
                let to_acc = self.accounts.get(&to_id).unwrap();
                (from_acc.balance, to_acc.balance)
            };

            if from_balance >= amount {
                self.accounts.get_mut(&from_id).unwrap().balance -= amount;
                self.accounts.get_mut(&to_id).unwrap().balance += amount;
                println!("Transferred {:.2} from Account {} to Account {}", amount, from_id, to_id);
            }
            else{
                println!("Insufficient balance for transfer!");
            }
            else{
            println!("Invalid account(s)!");
        }
    }
}

fn main() {
    let mut system = BankingSystem::new();

    system.create_user(1, "Betül");
    system.create_user(2, "Gülçin");
    system.create_user(3, "Mehmet");
    system.create_user(4, "Emine");

    system.create_bank(1, "Bank A");
    system.create_bank(2, "Bank B");

    system.create_account(1,1,1500.0);
    system.create_account(2,2,1000.0);
    system.create_account(3,1,2000.0);
    system.create_account(4,2,500.0);

    let accounts: Vec<u128> = system.accounts.keys().cloned().collect();
    system.transfer_money(accounts[0], accounts[1], 300.0);
}
        
