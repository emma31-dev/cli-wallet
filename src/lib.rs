pub struct BtcAccount {
    pub amount: f32,
}
    
pub struct EthAccount {
    pub amount: f32,
}
    
pub struct SolAccount {
    pub amount: f32,
}
    
pub struct NearAccount {
    pub amount: f32,
}
    
pub struct UsdtAccount {
    pub amount: f32,
}
    
pub struct Balances {
    pub btc: BtcAccount,
    pub eth: EthAccount,
    pub sol: SolAccount,
    pub near: NearAccount,
    pub usdt: UsdtAccount,
}
    
pub struct Wallet {
    pub owner: String,
    pub balances: Balances,
}
    
#[allow(dead_code)]
pub enum Crypto {
    Btc(BtcAccount),
    Eth(EthAccount),
    Sol(SolAccount),
    Near(NearAccount),
    Usdt(UsdtAccount),
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
    
    pub fn print_balance(&self) {
        let user_name = &self.owner;
        let balances = &self.balances; 
            
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
    
    pub fn add_money(mut self, amount: f32, crypto: &str) {
        if crypto == "btc" {
            self.balances.btc.amount += amount;
        } else if crypto == "eth" {
            self.balances.eth.amount += amount;
        } else if crypto == "sol" {
            self.balances.sol.amount += amount;
        } else if crypto == "near" {
            self.balances.near.amount += amount;
        } else if crypto == "usdt" {
            self.balances.usdt.amount += amount;
        } else {
            panic!("Adding money failed. Invalid command")
        }
    }
}
