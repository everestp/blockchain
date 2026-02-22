use p256::{
    ecdsa::{SigningKey, VerifyingKey},
    elliptic_curve::rand_core::OsRng,
};

pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Wallet {
    pub fn new() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key().clone();

        Self {
            signing_key,
            verifying_key,
        }
    }

    //  PUBLIC KEY STRING
    pub fn public_key_str(&self) -> String {
        let encoded = self.verifying_key.to_encoded_point(false);

        if let (Some(x), Some(y)) = (encoded.x(), encoded.y()) {
            format!(
                "{}{}",
                hex::encode(x),
                hex::encode(y)
            )
        } else {
            String::new()
        }
    }

    //  PRIVATE KEY STRING
    pub fn private_key_str(&self) -> String {
        hex::encode(self.signing_key.to_bytes())
    }
}