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
