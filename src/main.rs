use std::{collections::HashMap, io};

fn main() {
    println!("Hello, world!");
    wallet();
}

struct Balance {
    btc: f32,
    eth: f32,
    sol: f32,
    near: f32,
    usdt: f32,
}

struct Wallet {
    owner: String,
    balances: HashMap<Crypto, f32>
}

enum Crypto {
    Btc,
    Eth,
    Sol,
    Near,
    Usdt,
}

impl Wallet {
    fn print_balance(self) {
        println!(
            "You have {}BTC, {}ETH, {}SOL, {}NEAR, {}USDT",
            self.balances.btc, self.balances.eth, self.balances.sol, self.balances.near, self.balances.usdt
        );
    }
    
    fn transfer(mut self, amount: f32, mut receiver: User, crypto: Crypto) {
        self.btc = self.btc - amount;
        let mut sender_balance = receiver.get(crypto)
    }
}

fn wallet() {
    let john = Wallet;
    
    let paul = User {
        btc: 8.9,
        eth: 76.1,
        sol: 211.1,
        near: 401.5,
        usdt: 1000.5,
    };

    let mut input_command = String::new();
    io::stdin()
        .read_line(&mut input_command)
        .expect("Enter command to continue");

    let command = input_command.as_str();

    if command == "/balance" {
        john.print_balance()
    } else if command == "/transfer" {
        let mut crypto = String::new();
        io::stdin()
            .read_line(&mut crypto)
            .expect("Invalid crypto name");
        
        john.transfer(1.1, paul, crypto);
    } else {
        println!("Invalid command")
    }
}
