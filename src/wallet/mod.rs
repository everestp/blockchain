use log::debug;
use p256::{
    ecdsa::{Signature, SigningKey, VerifyingKey, signature::Signer, signature::Verifier},
    elliptic_curve::rand_core::OsRng,
};
use sha2::{Sha256, Digest};
use ripemd160::{Ripemd160, Digest as RipDigest};
use serde::{Serialize};


pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
    address: String,
}

#[derive(Serialize , Debug ,Clone)]
pub struct  WalletData {
    pub public_key :String,
    pub private_key :String,
    pub blockchain_address :String


}
#[derive(Serialize, Clone, Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
    pub public_key: String,
    pub signature: String,
}

/*
1.  do sha256  hash  on the x ,y of public key
2.  do ripemd  hash on the result of strp 1 and we will get 20 bytes result
3.  add a version byte at the head  of the result form step 2 (0x00 for mainnet)
4.  do sha 256 hash on the result  of step3
5.  do sha256 hash on the result of step 4
6.  take the first 4 bute of result of step 5 as checksu,
7.  appen the 4 bytes form strp 6 to the end  of the result from step3
8. encode the result of step 7 by baser 58
*/

impl Wallet {
    pub fn new() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key().clone();

        // closure for generating address
        let gen_address = || {
            let encoded = verifying_key.to_encoded_point(false);

            if let (Some(x), Some(y)) = (encoded.x(), encoded.y()) {
                let mut pub_key_bytes = Vec::with_capacity(x.len() + y.len());
                pub_key_bytes.extend_from_slice(x);
                pub_key_bytes.extend_from_slice(y);

                // STEP 1 — SHA256
                let hash = Sha256::digest(&pub_key_bytes);

                // STEP 2 — RIPEMD160 on the result of step1
                let mut hasher = Ripemd160::new();
                hasher.update(&hash);
                let mut hash_result = hasher.finalize().to_vec();

                // STEP 3 — add byte version at the front of ripemd hash result (0x00 for main net)
                hash_result.insert(0, 0x00);

                // STEP 4 & 5 — double SHA256
                let hash2 = Sha256::digest(&hash_result);
                let hash3 = Sha256::digest(&hash2);

                // STEP 6 — take the first 4 bytes from previous result as checksum
                let checksum = &hash3[0..4];

                // STEP 7 — append checksum to the end of extended ripemd hash result
                let full_hash = [hash_result, checksum.to_vec()].concat();

                // STEP 8 — base58 encoding
                bs58::encode(full_hash).into_string()
            } else {
                String::new()
            }
        };

        // call the closure to generate address
        let address = gen_address();

        Self {
            signing_key,
            verifying_key,
            address,
        }
    }

    pub fn get_wallet_data(&self) -> WalletData {
        WalletData { 
            public_key :self.public_key_str(),
            private_key :self.private_key_str(),
            blockchain_address:self.address.clone(),

        }
    }
    // PUBLIC KEY STRING
    pub fn public_key_str(&self) -> String {
        let encoded = self.verifying_key.to_encoded_point(false);

        if let (Some(x), Some(y)) = (encoded.x(), encoded.y()) {
            format!("{}{}", hex::encode(x), hex::encode(y))
        } else {
            String::new()
        }
    }

    // PRIVATE KEY STRING
    pub fn private_key_str(&self) -> String {
        hex::encode(self.signing_key.to_bytes())
    }

    pub fn get_adress(&self) -> String {
        self.address.clone()
    }

    // SIGN A TRANSACTION
    pub fn sign_transaction(&mut self, receiver: &String, amount: u64) -> Transaction {
        let mut transaction = Transaction {
            sender: self.address.clone(),
            recipient: receiver.clone(),
            amount,
            signature: String::new(),
            public_key: self.public_key_str(),
        };

        let serialize_str = serde_json::to_string(&transaction).unwrap();
        let serialized = serialize_str.as_bytes();

        // Sign using mutable reference to self.signing_key
        let sig: Signature = self.signing_key.sign(serialized);
        transaction.signature = hex::encode(sig.to_bytes());

        transaction
    }

    // VERIFY A TRANSACTION
    pub fn verify_transaction(transaction: &Transaction) -> bool {
        let signature_bin = hex::decode(&transaction.signature).unwrap();

        let mut transaction_clone = transaction.clone();
        transaction_clone.signature = String::new();

        let serialize_str = serde_json::to_string(&transaction_clone).unwrap();
        let serialized = serialize_str.as_bytes();

        // Convert signature from hex string to Signature struct
        let sig_array: [u8; 64] = signature_bin.try_into().unwrap();
        let signature = Signature::from_bytes(&sig_array.into()).unwrap();

        let mut public_key_bin = hex::decode(&transaction.public_key).unwrap();
        public_key_bin.insert(0, 0x04); // sec1 format [0x04 || x || y]

        let public_key = VerifyingKey::from_sec1_bytes(&public_key_bin).unwrap();

        public_key.verify(serialized, &signature).is_ok()
    }
}