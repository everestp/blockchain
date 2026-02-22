use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    ops::{AddAssign, Index},
    time::{Instant, SystemTime},
};
use crate::wallet::{Transaction as WalletTransaction, Wallet};
use crate::blockchain::transaction::Transaction;
pub mod transaction;

/// Trait for serialization/deserialization
pub trait Serialization<T> {
    fn serialization(&self) -> Vec<u8>;
    fn deserialization(byte: Vec<u8>) -> T;
}

/// Block search types
pub enum BlockSearch {
    SearchByIndex(usize),
    SearchByPreviousHash(Vec<u8>),
    SearchByBlockHash(Vec<u8>),
    SearchByTimeStamp(u128),
    SearchByTransaction(Vec<u8>),
    SearchByNonce(i32),
}

/// Block search result
pub enum BlockSearchResult<'a> {
    Success(&'a Block),
    FailOfEmptyBlocks,
    FailOfIndex(usize),
    FailOfPreviousHash(Vec<u8>),
    FailOfBlockHash(Vec<u8>),
    FailOfNonce(i32),
    FailOfTimeStamp(u128),
    FailOfTransaction(Vec<u8>),
}

/// Blockchain block
#[derive(Debug ,Clone ,Serialize ,Deserialize)]
pub struct Block {
    nonce: i32,
    previous_hash: Vec<u8>,
    time_stamps: u128,
    transactions: Vec<Vec<u8>>,
}

/// Allow `*block += 1` to increment nonce
impl AddAssign<i32> for Block {
    fn add_assign(&mut self, rhs: i32) {
        self.nonce += rhs;
    }
}

/// Compare blocks by their hash
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

impl Block {
    /// Create a new block
    pub fn new(nonce: i32, previous_hash: Vec<u8>) -> Self {
        let time_now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        Block {
            nonce,
            previous_hash,
            time_stamps: time_now.as_nanos(),
            transactions: Vec::<Vec<u8>>::new(),
        }
    }

    /// Print block details
    pub fn print(&self) {
        println!("nonce : {}", self.nonce);
        println!("Timestamp : {:x}", self.time_stamps);
        println!("previous hash : {:?}", self.previous_hash);
        for (idx, tx) in self.transactions.iter().enumerate() {
            let transaction = Transaction::deserialization(tx.clone());
            println!("the {}th transaction is: {}", idx, transaction);
        }
    }

    /// Compute SHA256 hash of block
    pub fn hash(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();
        bin.extend(self.nonce.to_be_bytes());
        bin.extend(self.previous_hash.clone());
        bin.extend(self.time_stamps.to_be_bytes());
        for tx in &self.transactions {
            bin.extend(tx.clone());
        }

        let mut hasher = Sha256::new();
        hasher.update(bin);
        hasher.finalize().to_vec()
    }
}

/// Blockchain struct
#[derive(Debug ,Clone)]
pub struct BlockChain {
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>,
    blockhain_address: String,
}

/// Allow indexing into blockchain to get a block
impl Index<usize> for BlockChain {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        self.chain.get(index).expect("index out of range for the chain")
    }
}

impl BlockChain {
    const DIFFICULTY: usize = 3;
    const MINING_SENDER: &str = "THE_BLOCKCHAIN";
    const MINING_REWARD: u64 = 1;

    /// Create new blockchain and automatically mine genesis block
    pub fn new(address: String) -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
            blockhain_address: address,
        };

        // create genesis block
        let genesis_block = Block::new(0, vec![0, 32]);
        bc.chain.push(genesis_block);

        // mine the genesis block
        bc.mining();

        bc
    }

    /// Print the entire blockchain
    pub fn print(&self) {
        if self.chain.is_empty() {
            println!("Blockchain is empty!");
            return;
        }

        println!("{}", "=".repeat(80));
        println!("Blockchain contains {} block(s)", self.chain.len());
        println!("{}", "=".repeat(80));

        for (i, block) in self.chain.iter().enumerate() {
            println!("\nBlock #{} {}", i, "-".repeat(50));
            println!("Nonce       : {}", block.nonce);
            println!("Timestamp   : {}", block.time_stamps);
            println!(
                "PreviousHash: {}",
                hex::encode(&block.previous_hash)
            );
            println!("BlockHash   : {}", hex::encode(block.hash()));
            println!("Transactions:");
            if block.transactions.is_empty() {
                println!("  No transactions");
            } else {
                for (j, tx) in block.transactions.iter().enumerate() {
                    println!("  [{}] {}", j, hex::encode(tx));
                }
            }
            println!("{}", "-".repeat(80));
        }
        println!("\nEnd of Blockchain\n{}", "=".repeat(80));
    }

    /// Create a new block and add transactions
    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let mut block = Block::new(nonce, previous_hash);
        for tx in &self.transaction_pool {
            block.transactions.push(tx.clone());
        }
        self.transaction_pool.clear();

        let now = Instant::now();
        let proof_hash = BlockChain::do_proof_of_work(&mut block);
        let elapsed = now.elapsed();
        self.chain.push(block);

        println!(
            "compute time: {:?}\nproof for the current block is: {}",
            elapsed, proof_hash
        );
    }

    /// Get the last block in the chain
    pub fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /// Add a transaction to the pool
    pub fn add_transaction(&mut self, tx: &WalletTransaction) -> bool {
        // miners cannot send money to themselves
        if tx.sender == self.blockhain_address {
            println!("miner cannot send money to himself");
            return false;
        }

        // Check if it's a miner reward transaction
        let is_miner_reward = tx.sender == BlockChain::MINING_SENDER;

        if !is_miner_reward {
            // Normal transaction: verify signature
            if !Wallet::verify_transaction(tx) {
                println!("invalid transaction");
                return false;
            }
            // Normal transaction: check sender balance
            if self.calculate_total_amount(tx.sender.clone()) < tx.amount as i64 {
                println!("Sender does not have enough balance");
                return false;
            }
        }

        // Serialize transaction
        let transaction = Transaction::new(
            tx.sender.as_bytes().to_vec(),
            tx.recipient.as_bytes().to_vec(),
            tx.amount,
        );

        // Prevent duplicates
        if self.transaction_pool.iter().any(|t| *t == transaction.serialization()) {
            return false;
        }

        self.transaction_pool.push(transaction.serialization());
        true
    }

    /// Proof-of-Work algorithm
    fn do_proof_of_work(block: &mut Block) -> String {
        let target = "0".repeat(BlockChain::DIFFICULTY);
        loop {
            let hash = block.hash();
            let hash_str = hex::encode(&hash);
            if &hash_str[0..BlockChain::DIFFICULTY] == target {
                return hash_str;
            }
            *block += 1;
        }
    }

    /// Mine a new block with reward transaction
    pub fn mining(&mut self) -> bool {
        /*
        When a block is minted, a transaction is created to reward the miner.
        Miner reward does not require signature verification or balance check.
        */
        let tx = WalletTransaction {
            sender: BlockChain::MINING_SENDER.to_string(),
            recipient: self.blockhain_address.clone(),
            amount: BlockChain::MINING_REWARD,
            public_key: "".to_string(),
            signature: "".to_string(),
        };

        self.add_transaction(&tx); // always succeeds
        self.create_block(0, self.last_block().hash());
        true
    }

    /// Calculate the total balance for an address
    pub fn calculate_total_amount(&self, address: String) -> i64 {
        let mut total_amount: i64 = 0;
        let address_bytes = address.as_bytes();

        // iterate over all blocks
        for block in &self.chain {
            for t in &block.transactions {
                let tx = Transaction::deserialization(t.clone());

                if tx.recipient_address == address_bytes {
                    total_amount += tx.value as i64;
                }

                if tx.sender_address == address_bytes {
                    total_amount -= tx.value as i64;
                }
            }
        }

        total_amount
    }
}