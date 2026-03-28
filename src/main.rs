mod structures;
use crate::structures::{ Wallet, Balances, BtcAccount, EthAccount, SolAccount, NearAccount, UsdtAccount };

fn main() {
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
    pub fn new(owner: &str) -> Self {
        Self {
            owner: String::from(owner),
            balances: Balances::new()
        }
    }
    
    fn print_balance(self) {
        let user_name = self.owner;
        let balances = self.balances; 
        
        println!("Welcome {}", user_name);
        println!(
            "You have {}BTC, {}ETH, {}SOL, {}NEAR AND {}USDT", 
            balances.btc.amount, 
            balances.eth.amount, 
            balances.sol.amount, 
            balances.near.amount, 
            balances.usdt.amount
        );
    }
}

fn wallet() {
    let user1 = Wallet::new("John");
    
    user1.print_balance();
}