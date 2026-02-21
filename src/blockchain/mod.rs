
use std::{time::{Instant, SystemTime}, vec};
use sha2::{Digest, Sha256 ,};

use crate::blockchain::transaction::Transaction;
pub mod transaction;
 use std::ops::AddAssign;

pub trait  Serilization <T>{
    fn serialization(&self)-> Vec<u8>;
    fn deserialization(byte : Vec<u8>)-> T;
}
pub enum BlockSearch {
    SearchByIndex(usize),
    SearchByPreviousHash(Vec<u8>),
    SearchByBlockHash(Vec<u8>),
    SearchByTimeStamp(u128),
    SearchByTransaction(Vec<u8>),
    SearchByNonce(i32),
}

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

#[derive(Debug)]
pub struct Block {
    nonce: i32,
    previous_hash: Vec<u8>,
    time_stamps: u128,
    transactions: Vec<Vec<u8>>,
}

impl AddAssign<i32>  for Block{
    fn add_assign(&mut self, rhs: i32) {
         self.nonce +=rhs;
    }
}

impl Block {
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

    pub fn print(&self) {
        println!("nonce : {}", self.nonce);
        println!("Timestamp :{:x}", self.time_stamps);
        println!("previous hash :{:?}", self.previous_hash);
        println!("transaction:{:?}", self.transactions);
    }

    pub fn hash(&self) -> Vec<u8> {
        let mut bin = Vec::<u8>::new();
        bin.extend(self.nonce.to_be_bytes());
        bin.extend(self.previous_hash.clone());
        bin.extend(self.time_stamps.to_be_bytes());
        for tx in self.transactions.iter() {
            bin.extend(tx.clone());
        }

        let mut hasher = Sha256::new();
        hasher.update(bin);
        hasher.finalize().to_vec()
    }
}

#[derive(Debug)]
pub struct BlockChain {
    transaction_pool: Vec<Vec<u8>>,
    chain: Vec<Block>,
}

impl BlockChain {
    const DIFFICULTY : usize = 5;
    pub fn new() -> Self {
        let mut bc = BlockChain {
            transaction_pool: Vec::<Vec<u8>>::new(),
            chain: Vec::<Block>::new(),
        };
        bc.create_block(0, vec![0 as u8, 32]);
        bc
    }
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
            hex::encode(&block.previous_hash) // hex format
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
    pub fn create_block(&mut self, nonce: i32, previous_hash: Vec<u8>) {
        let mut b = Block::new(nonce, previous_hash);
        for tx in self.transaction_pool.iter(){
            b.transactions.push(tx.clone());
        }
        self.transaction_pool.clear();

        let now = Instant::now();
        let proof_hash = BlockChain::do_proof_of_work(&mut b);
        let elapsed = now.elapsed();
        self.chain.push(b);
        println!("compute timee :{:?}\n proof for the current block is : {:?}",elapsed ,proof_hash)
    }
  
    pub fn last_block(&self) -> &Block {
        if self.chain.len() > 1 {
            &self.chain[self.chain.len() - 1]
        } else {
            &self.chain[0]
        }
    }

    pub fn search_block(&self, search: BlockSearch) -> BlockSearchResult {
        if self.chain.is_empty() {
            return BlockSearchResult::FailOfEmptyBlocks;
        }

        for (idx, block) in self.chain.iter().enumerate() {
            match search {
                BlockSearch::SearchByIndex(index) => {
                    if idx == index {
                        return BlockSearchResult::Success(block);
                    }
                }

                BlockSearch::SearchByPreviousHash(ref hash) => {
                    if block.previous_hash == *hash {
                        return BlockSearchResult::Success(block);
                    }
                }

                BlockSearch::SearchByBlockHash(ref hash) => {
                    if block.hash() == *hash {
                        return BlockSearchResult::Success(block);
                    }
                }

                BlockSearch::SearchByTimeStamp(ts) => {
                    if block.time_stamps == ts {
                        return BlockSearchResult::Success(block);
                    }
                }

                BlockSearch::SearchByTransaction(ref transaction) => {
                    let mut found = false;
                    for t in block.transactions.iter() {
                        if t == transaction {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        return BlockSearchResult::Success(block);
                    }
                }

                BlockSearch::SearchByNonce(nonce) => {
                    if block.nonce == nonce {
                        return BlockSearchResult::Success(block);
                    }
                }
            }
        }

        // If not found, return the correct Fail variant
        match search {
            BlockSearch::SearchByIndex(idx) => BlockSearchResult::FailOfIndex(idx),
            BlockSearch::SearchByPreviousHash(hash) => {
                BlockSearchResult::FailOfPreviousHash(hash)
            }
            BlockSearch::SearchByBlockHash(hash) => BlockSearchResult::FailOfBlockHash(hash),
            BlockSearch::SearchByTimeStamp(ts) => BlockSearchResult::FailOfTimeStamp(ts),
            BlockSearch::SearchByTransaction(tx) => BlockSearchResult::FailOfTransaction(tx),
            BlockSearch::SearchByNonce(nonce) => BlockSearchResult::FailOfNonce(nonce),
        }
    }

pub fn add_transaction(&mut self, tx: &impl Serilization<Transaction>) {
    for tx_in_pool in self.transaction_pool.iter() {
        if *tx_in_pool == tx.serialization() {
            return;
        }
    }
    self.transaction_pool.push(tx.serialization());
}

fn do_proof_of_work(block: &mut Block)->String{
    loop{
        let hash = block.hash();
        let  hash_str = hex::encode(&hash);
       if hash_str[0..BlockChain::DIFFICULTY] == "0".repeat(BlockChain::DIFFICULTY){
        return hash_str;
       }
       *block +=1;

    }
}


}