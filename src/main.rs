mod structures;
use crate::structures::{ Wallet, Balances, BtcAccount, EthAccount, SolAccount, NearAccount, UsdtAccount };

fn main() {
    println!("Hello, world!");
    wallet();
}

impl BtcAccount {
    fn new(amount: f32) -> Self {
        Self { amount: amount }
    }
}

impl EthAccount {
    fn new(amount: f32) -> Self {
        Self { amount: amount }
    }
}

impl SolAccount {
    fn new(amount: f32) -> Self {
        Self { amount: amount }
    }
}

impl NearAccount {
    fn new(amount: f32) -> Self {
        Self { amount: amount }
    }
}

impl UsdtAccount {
    fn new(amount: f32) -> Self {
        Self { amount: amount }
    }
}

impl Balances {
    pub fn new() -> Self {
        Self {
            btc: BtcAccount::new(0.0),
            eth: EthAccount::new(0.0),
            sol: SolAccount::new(0.0),
            near: NearAccount::new(0.0),
            usdt: UsdtAccount::new(0.0)
        }
    }
}

impl Wallet {
    pub fn new(owner: String, balances: Balances) -> Self {
        Self {
            owner,
            balances
        }
    }
    
    fn print_balance(&self) {
        let btc_balance = self.balances.btc.amount; 
        
        println!("You have {}BTC", btc_balance);
    }
}

fn wallet() {
    let user1 = Wallet::new(String::from("John"), Balances::new());
    
    user1.print_balance();
}