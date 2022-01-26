use crate::blockchain::{
    block::Block, hash::Hash, signature::Signature, transaction::Transaction, wallet::Wallet,
};

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blocks = Vec::new();
        let satoshi = Wallet::new();
        let first = Wallet::new();
        let trans = Transaction {
            amount: 100.0,
            receiver_pk: satoshi.key_pair.public_key.clone(),
            sender_pk: first.key_pair.public_key.clone(),
        };
        let hash = Hash::new(String::from("first"));

        let block = Block::new(trans, hash);
        blocks.push(block);

        Blockchain { blocks }
    }

    pub fn add_block(&mut self, transaction: Transaction, signature: &Signature) -> bool {
        if transaction.valid(signature) {
            let block = Block::new(transaction, self.last_block_hash());
            self.mine(&block);
            self.blocks.push(block);

            return true;
        }

        false
    }

    pub fn last_block_hash(&self) -> Hash {
        match self.blocks.last() {
            Some(block) => block.hash(),
            None => panic!("Cannot add block, chain is empty!"),
        }
    }

    fn mine(&self, block: &Block) {
        println!("Mining");

        let mut solution: u32 = 0;

        loop {
            let hash = md5::compute(format!("{}{}", block.nonce, solution));
            let hash = format!("{:x}", hash);

            if &hash[0..4] == "0000" {
                println!("Mined: {}", hash);
                break;
            }

            solution += 1;
        }
    }
}
