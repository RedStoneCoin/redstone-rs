use crate::crypto::hash;
use secp256k1::bitcoin_hashes::sha256;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, Signature};

#[derive(Debug)]
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

    pub fn sign(&self, message: String) -> Result<String, Box<dyn std::error::Error>> {
        let secp = Secp256k1::new();
        let sk_bytes = self.private_key.as_bytes();
        let secretkey = SecretKey::from_slice(&sk_bytes)?;
        let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        let sig = secp.sign(&msg, &secretkey);
        Ok(hex::encode(sig.serialize_der().to_vec()))
    }

    pub fn verify(
        &self,
        message: String,
        signature: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig_bytes = hex::decode(signature)?;
        let signature: Signature = Signature::from_der(&sig_bytes)?;
        let secp = Secp256k1::new();
        let pk_bytes = self.public_key.as_bytes();
        let publickey = PublicKey::from_slice(&pk_bytes)?;
        let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        return Ok(secp.verify(&msg, &signature, &publickey).is_ok());
    }
}
