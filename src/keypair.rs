use crate::crypto::hash;
use rand::thread_rng;
use secp256k1::bitcoin_hashes::sha256;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, SerializedSignature, Signature};

pub struct Keypair {
    pub publicKey: String,
    pub privateKey: String,
}

impl Keypair {
    pub fn address(&self) -> String {
        hash(hex::decode(&self.publicKey).unwrap())
    }
    pub fn generate() -> Self {
        let secp = Secp256k1::new();
        let mut rng = thread_rng();
        let (secret_key, public_key) = secp.generate_keypair(&mut rng);
        Keypair {
            privateKey: secret_key,
            publicKey: public_key,
        }
    }
    pub fn sign(&self, message: String) -> Result<String, Box<dyn std::error::Error>> {
        let sk_bytes = hex::decode(message)?;
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(hex::decode(self.privateKey)?)?;
        let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        let sig = secp.sign(msg, sk);
        Ok(hex::encode(sig.serialize_der().to_vec()))
    }
    pub fn verify(
        &self,
        message: String,
        signature: String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let sig_bytes = hex::decode(signature)?;
        let signature: Signature = Signature::from_der(sig_bytes)?;
        let secp = Secp256k1::new();
        let pk_bytes = hex::decode(self.publicKey)?;
        let publickey = PublicKey::from_slice(&pk_bytes)?;
        let msg = Message::from_hashed_data::<sha256::Hash>(message.as_bytes());
        return Ok(secp.verify(msg, signature, &publickey).is_ok());
    }
}
