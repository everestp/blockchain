pub  mod  wallet;
pub mod blockchain;
use wallet::Wallet;

use crate::blockchain::BlockChain;


fn main() {
    // -------------------------
    // Create a new wallet
    // -------------------------
    // make wallet mutable because sign_transaction requires &mut self
    let mut wallet = Wallet::new();

    println!("Private key: {}", wallet.private_key_str());
    println!("Public key: {}", wallet.public_key_str());
    println!("Address: {}", wallet.get_adress());

    // -------------------------
    // Create a receiver address
    // -------------------------
    let receiver = String::from("0x12323");

    // -------------------------
    // Sign a transaction
    // -------------------------
    let transaction = wallet.sign_transaction(&receiver, 1);
    println!("Transaction: {:?}", transaction);

    // -------------------------
    // Verify the transaction
    // -------------------------
    let is_valid = Wallet::verify_transaction(&transaction);
    println!("Verifying: {}", is_valid);

    // -------------------------
    // Create a blockchain
    // -------------------------
    let mut blockchain = BlockChain::new(wallet.get_adress());

    println!("\nInitial Blockchain:");
    blockchain.print();

    // -------------------------
    // Add the transaction to the blockchain
    // -------------------------
    if blockchain.add_transaction(&transaction) {
        println!("\nTransaction added to pool.");
    } else {
        println!("\nFailed to add transaction.");
    }

    println!("\nBlockchain BEFORE mining:");
    blockchain.print(); // transaction is in pool, not yet in a block

    // -------------------------
    // Mine a new block
    // -------------------------
    blockchain.mining();

    println!("\nBlockchain AFTER mining:");
    blockchain.print(); // mined block contains transaction

    // -------------------------
    // Check balances
    // -------------------------
    let wallet_balance = blockchain.calculate_total_amount(wallet.get_adress());
    let receiver_balance = blockchain.calculate_total_amount(receiver.clone());

    println!("\nBalances:");
    println!("Wallet balance: {}", wallet_balance);
    println!("Receiver balance: {}", receiver_balance);
}