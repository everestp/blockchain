pub mod  blockchain;
use blockchain::{BlockSearchResult};
use crate::blockchain::{Block, BlockChain, Serialization, transaction::Transaction};

fn get_block_search_result(result: BlockSearchResult) {
    match result {
        BlockSearchResult::Success(block) => {
            println!("Found the block:");
            block.print();
        }
        BlockSearchResult::FailOfIndex(idx) => {
            println!("No block found at index: {}", idx);
        }
        BlockSearchResult::FailOfPreviousHash(hash) => {
            println!("No block found with previous hash: {:?}", hash);
        }
        BlockSearchResult::FailOfBlockHash(hash) => {
            println!("No block found with block hash: {:?}", hash);
        }
        BlockSearchResult::FailOfNonce(nonce) => {
            println!("No block found with nonce: {}", nonce);
        }
        BlockSearchResult::FailOfTimeStamp(ts) => {
            println!("No block found with timestamp: {}", ts);
        }
        BlockSearchResult::FailOfTransaction(tx) => {
            println!("No block found containing transaction: {:?}", tx);
        }
        BlockSearchResult::FailOfEmptyBlocks => {
            println!("The blockchain has no blocks.");
        }
    }
}

fn main() {

    // // Create blockchain
    // let mut blockchain = BlockChain::new();

    // println!("Initial Blockchain:");
    // blockchain.print();

    // // ------------------------------------------------
    // // Create some empty blocks
    // // ------------------------------------------------
    // let prev_hash = blockchain.last_block().hash().clone();
    // blockchain.create_block(1, prev_hash);

    // let prev_hash = blockchain.last_block().hash().clone();
    // blockchain.create_block(2, prev_hash);

    // println!("\nBlockchain after creating 2 blocks:");
    // blockchain.print();

    // // ------------------------------------------------
    // // Create Transaction
    // // ------------------------------------------------
    // let tx = Transaction::new(
    //     b"Alice".to_vec(),
    //     b"Bob".to_vec(),
    //     250,
    // );

    // println!("\nTransaction:");
    // println!("{}", tx);

    // // Serialize
    // let tx_bin = tx.serialization();
    // println!("Serialized: {:?}", tx_bin);

    // // Deserialize
    // let tx_from_bin = Transaction::deserialization(tx_bin);
    // println!("Deserialized: {}", tx_from_bin);

    // // ------------------------------------------------
    // // Add transaction to pool (MEMPOOL)
    // // ------------------------------------------------
    // blockchain.add_transaction(&tx_from_bin);

    // println!("\nTransaction added to pool.");
    // println!("Blockchain BEFORE mining:");
    // blockchain.print();   // still no transaction in block

    // // ------------------------------------------------
    // // Mine new block (VERY IMPORTANT STEP)
    // // ------------------------------------------------
    // let prev_hash = blockchain.last_block().hash().clone();
    // blockchain.create_block(3, prev_hash);

    // println!("\nBlockchain AFTER mining:");
    // blockchain.print();   //  transaction now visible
let my_block_chain_adress = "my blockchain_Address";
let mut block_chain = BlockChain::new(my_block_chain_adress.into());
block_chain.print();
block_chain.add_transaction(&Transaction::new("A".into(), "B".into(), 10));
block_chain.mining();
block_chain.print();
block_chain.add_transaction(&Transaction::new("C".into(), "D".into(), 10));
block_chain.add_transaction(&Transaction::new("X".into(), "Y".into(), 10));
block_chain.mining();
block_chain.print();
println!("value for miner :{}",block_chain.calculate_total_amount(my_block_chain_adress.to_string()));
println!("value for C :{}",block_chain.calculate_total_amount("C".to_string()));
println!("value for D :{}",block_chain.calculate_total_amount("D".to_string()));

}