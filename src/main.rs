extern crate structures;

use crate::structures::{ Wallet };

fn main() {
    let user1 = Wallet::new("John");
    let user2 = Wallet::new("Jake");
    
    user1.print_balance();
    user2.print_balance();
    
    user1.add_money(1.0, "btc");
}