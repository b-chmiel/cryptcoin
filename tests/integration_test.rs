use cryptcoin::blockchain::Blockchain;
use cryptcoin::wallet::Wallet;

#[test]
fn send_balance() {
	let mut chain = Blockchain::new();
	let satoshi = Wallet::new();
	let user1 = Wallet::new();
	let user2 = Wallet::new();

	satoshi.send(&mut chain, 10.0, &user1.key_pair.public_key);

	assert_eq!(user1.balance(&chain), 10.0);
	assert_eq!(user2.balance(&chain), 0.0);

	user1.send(&mut chain, 5.0, &user2.key_pair.public_key);

	assert_eq!(user1.balance(&chain), 5.0);
	assert_eq!(user2.balance(&chain), 5.0);

	user2.send(&mut chain, 2.0, &user1.key_pair.public_key);

	assert_eq!(user1.balance(&chain), 7.0);
	assert_eq!(user2.balance(&chain), 3.0);
}

#[test]
fn no_sufficient_funds() {
	let mut chain = Blockchain::new();
	let user1 = Wallet::new();
	let user2 = Wallet::new();

	assert_eq!(user1.balance(&chain), 0.0);
	assert_eq!(user2.balance(&chain), 0.0);

	user1.send(&mut chain, 20.0, &user2.key_pair.public_key);

	assert_eq!(user1.balance(&chain), 0.0);
	assert_eq!(user2.balance(&chain), 0.0);
}

#[test]
fn double_spending() {
	let mut chain = Blockchain::new();
	let satoshi = Wallet::new();
	let user1 = Wallet::new();
	let user2 = Wallet::new();
	let user3 = Wallet::new();

	satoshi.send(&mut chain, 10.0, &user1.key_pair.public_key);

	user1.send(&mut chain, 10.0, &user2.key_pair.public_key);
	user1.send(&mut chain, 10.0, &user3.key_pair.public_key);

	assert_eq!(user1.balance(&chain), 0.0);
	assert_eq!(user2.balance(&chain), 10.0);
	assert_eq!(user3.balance(&chain), 0.0);
}
