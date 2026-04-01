struct BtcAccount {
    amount: f32,
}
    
struct EthAccount {
    amount: f32,
}
    
struct SolAccount {
    amount: f32,
}
    
struct NearAccount {
    amount: f32,
}
    
struct UsdtAccount {
    amount: f32,
}
    
struct Balances {
    btc: BtcAccount,
    eth: EthAccount,
    sol: SolAccount,
    near: NearAccount,
    usdt: UsdtAccount,
}
    
pub struct Wallet {
    owner: String,
    balances: Balances,
}

impl BtcAccount {
    fn new(amount: f32) -> Self {
        Self { amount }
    }
}

impl EthAccount {
    fn new(amount: f32) -> Self {
        Self { amount }
    }
}

impl SolAccount {
    fn new(amount: f32) -> Self {
        Self { amount }
    }
}

impl NearAccount {
    fn new(amount: f32) -> Self {
        Self { amount }
    }
}

impl UsdtAccount {
    fn new(amount: f32) -> Self {
        Self { amount }
    }
}

impl Balances {
    fn new() -> Self {
        Self {
            btc: BtcAccount::new(0.0),
            eth: EthAccount::new(0.0),
            sol: SolAccount::new(0.0),
            near: NearAccount::new(0.0),
            usdt: UsdtAccount::new(0.0),
        }
    }
}

impl Wallet {
    pub fn new(owner: &str) -> Self {
        Self {
            owner: String::from(owner).to_lowercase(),
            balances: Balances::new(),
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
            balances.usdt.amount,
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
            panic!("Adding money failed. Invalid command");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new_crypto() {
        let btc_wallet = BtcAccount::new(1.1);
        let eth_wallet = EthAccount::new(3.7);
        let sol_wallet = SolAccount::new(4.34);
        let near_wallet = NearAccount::new(2.5);
        
        assert_eq!(btc_wallet.amount, 1.1);
        assert_eq!(eth_wallet.amount, 3.7);
        assert_eq!(sol_wallet.amount, 4.34);
        assert_eq!(near_wallet.amount, 2.5);
    }
    
    #[test]
    fn wallet_creation() {
        let user1 = Wallet::new("JOHN");
        let user2 = Wallet::new("John");
        let user3 = Wallet::new("jOhn");
        
        assert_eq!(user1.owner, "john".to_string());
        assert_eq!(user2.owner, "john".to_string());
        assert_eq!(user3.owner, "john".to_string());
    }
}
