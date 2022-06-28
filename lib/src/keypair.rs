use crate::crypto::hash;
use secp256k1::bitcoin_hashes::sha256;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};
extern crate bs58;

#[derive(Debug, Clone)]
pub struct Keypair {
    pub public_key: String,
    pub private_key: String,
}

impl Keypair {
    pub fn address(&self) -> String {
        format!(
            "0x{}",
            hash(hex::decode(&self.public_key).unwrap())[..40].to_string()
        )
    }

    pub fn generate() -> Self {
        let secp = Secp256k1::new();
        let mut rng = rand::OsRng::new().unwrap();
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        Keypair {
            private_key: secret_key.to_string(),
            public_key: public_key.to_string(),
        }
    }
    // self.private_key.as_bytes();
    pub fn sign(&self, message: String) -> Result<String, Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        let a = hex::decode(&self.private_key).unwrap();
        let secretkey = secp256k1::key::SecretKey::from_slice(&a);
        let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        let sig = secp.sign(&msg, &secretkey.unwrap());
        Ok(hex::encode(sig.serialize_der().to_vec()))
    }

    pub fn verify(
        &self,
        message: &String,
        signature: &String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig_bytes = hex::decode(signature)?;
        let signature: Signature = Signature::from_der(&sig_bytes)?;
        let secp = Secp256k1::new();
        let pk_bytes = hex::decode(self.public_key.as_bytes());
        let publickey = PublicKey::from_slice(&pk_bytes.unwrap())?;
        let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        return Ok(secp.verify(&msg, &signature, &publickey).is_ok());
    }

    pub fn from_private_key(pk: String) -> Keypair {
        let pk11 = pk.clone();
        let a = hex::decode(&pk).unwrap();
        let secretkey = secp256k1::key::SecretKey::from_slice(&a);
        let secp = &Secp256k1::new();
        let pki = secp256k1::key::PublicKey::from_secret_key(secp, &secretkey.unwrap());
        Keypair {
            private_key: pk11.to_string(),
            public_key: pki.to_string(),
        }
    }
}
