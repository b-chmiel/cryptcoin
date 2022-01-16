use cryptcoin::blockchain::Blockchain;
use cryptcoin::wallet::Wallet;

fn main() {
    let mut chain = Blockchain::new();
    let satoshi = Wallet::new();
    let user1 = Wallet::new();
    let user2 = Wallet::new();

    println!("Satoshi: {:#?}", satoshi);
    println!("User1: {:#?}", user1);
    println!("User2: {:#?}", user2);

    satoshi.send(&mut chain, 10.0, &user1.key_pair.public_key);
    user1.send(&mut chain, 5.0, &user2.key_pair.public_key);
    user2.send(&mut chain, 2.0, &user1.key_pair.public_key);

    println!("{:#?}", chain);
}
