pub mod blockchain;
use blockchain::{Block , BlockChain , BlockSearch,BlockSearchResult};
use sha2::{Digest, Sha256};

fn get_block_search_result(result: BlockSearchResult) {
    match result {
        BlockSearchResult::Success(block) => {
            println!(" Found the block:");
            block.print();
        }


        BlockSearchResult::FailOfIndex(idx) => {
            println!(" No block found at index: {}", idx);
        }

        BlockSearchResult::FailOfPreviousHash(hash) => {
            println!(" No block found with previous hash: {:?}", hash);
        }

        BlockSearchResult::FailOfBlockHash(hash) => {
            println!(" No block found with block hash: {:?}", hash);
        }

        BlockSearchResult::FailOfNonce(nonce) => {
            println!(" No block found with nonce: {}", nonce);
        }

        BlockSearchResult::FailOfTimeStamp(ts) => {
            println!(" No block found with timestamp: {}", ts);
        }

        BlockSearchResult::FailOfTransaction(tx) => {
            println!(" No block found containing transaction: {:?}", tx);
        }

        BlockSearchResult::FailOfEmptyBlocks => {
            println!("⚠️ The blockchain has no blocks.");
        }
    }
}
fn main() {
  

    let mut block_chain = BlockChain::new();
    println!("Block chain :{:?}", block_chain);
    block_chain.print();

    let previous_hash = block_chain.last_block().hash();
      let hash_to_find = previous_hash.clone();
    
    block_chain.create_block(1, previous_hash);
    let previous_hash = block_chain.last_block().hash();
    block_chain.create_block(2, previous_hash);
    block_chain.print();
    let  result = block_chain.search_block(BlockSearch::SearchByIndex(1));
    get_block_search_result(result);
     let result1 = block_chain.search_block(BlockSearch::SearchByIndex(10));
    get_block_search_result(result1);
}
