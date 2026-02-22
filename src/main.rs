pub  mod  wallet;
use wallet::Wallet;


fn main(){
 let  wallet = Wallet::new();
 println!("private key :{}",wallet.private_key_str());
 println!("public key {}",wallet.public_key_str())
}