pub  mod  wallet;
use wallet::Wallet;


fn main() {
    // make wallet mutable because sign_transaction requires &mut self
    let mut wallet = Wallet::new();

    println!("Private key: {}", wallet.private_key_str());
    println!("Public key: {}", wallet.public_key_str());
    println!("Address: {}", wallet.get_adress());

    // receiver as String
    let receiver = String::from("0x12323");

    // sign a transaction
    let transaction = wallet.sign_transaction(&receiver, 1);

    println!("Transaction: {:?}", transaction);

    // verify transaction
    let is_valid = Wallet::verify_transaction(&transaction);
    println!("Verifying: {}", is_valid);
}