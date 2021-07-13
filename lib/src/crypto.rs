use crate::keypair::Keypair;
use blake2::{Blake2b, Digest};
use vrf::{
    openssl::{CipherSuite, ECVRF},
    VRF,
};

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash_item(&self) -> String {
        // First we calculate the bytes of the object being passed to us
        let bytes = self.bytes();
        hash(bytes)
    }
}

pub fn hash(bytes: Vec<u8>) -> String {
    let mut hasher = Blake2b::new();

    // write input message
    hasher.update(bytes);

    // read hash digest and consume hasher
    let res: Vec<u8> = hasher.finalize().into_iter().collect();

    // Finally we base 58 encode the result
    let hash: String = hex::encode(res);
    hash
}

#[derive(Debug, Clone)]
pub struct Vrf {
    pub proof: String, // Hex encoded proof
    pub hash: String,  // hex encoded hash of the proof
}
impl Vrf {
    /// # Numerical
    /// Takes a refrence to this struct and returns the two numerical values (u1282) derived from its hash
    pub fn numerical(&self) -> Result<(u64, u64), Box<dyn std::error::Error>> {
        // split the hash into two 32 bit chunks
        let chunk_one = self.hash[..32].to_string();
        let chunk_two = self.hash[32..].to_string();
        let chunk_one_numerical = u64::from_str_radix(&chunk_one, 16)?;
        let chunk_two_numerical = u64::from_str_radix(&chunk_two, 16)?;
        Ok((chunk_one_numerical, chunk_two_numerical))
    }

    /// # Generate
    /// takes in a secp256k1 keypair and a seed and outputs a proof and a hash
    pub fn generate(keypair: &Keypair, seed: String) -> Result<Vrf, Box<dyn std::error::Error>> {
        let mut ctx = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        let sk = hex::decode(&keypair.private_key)?;
        let msg: &[u8] = seed.as_bytes();
        let proof = ctx.prove(&sk, &msg).unwrap();
        let hash = ctx.proof_to_hash(&proof).unwrap();
        Ok(Vrf {
            proof: hex::encode(proof),
            hash: hex::encode(hash),
        })
    }
    pub fn from_proof(proof: &String) -> Result<Vrf, Box<dyn std::error::Error>> {
        let mut ctx = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        let proof_bytes = hex::decode(&proof)?;
        let hash = ctx.proof_to_hash(&proof_bytes).unwrap();
        Ok(Vrf {
            proof: proof.to_owned(),
            hash: hex::encode(hash),
        })
    }
    pub fn valid(
        &self,
        keypair: Keypair,
        message: &String,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut ctx = ECVRF::from_suite(CipherSuite::SECP256K1_SHA256_TAI).unwrap();
        let proof_bytes = hex::decode(&self.proof)?;
        let msg = message.as_bytes();
        if let Ok(_) = ctx.verify(&hex::decode(&keypair.public_key)?, &proof_bytes, &msg) {
            return Ok(true);
        }
        Ok(false)
    }
}

pub mod tests {
    use super::*;
    #[test]
    pub fn test_length() {
        let keypair = Keypair::generate();
        let vrf = Vrf::generate(&keypair, "TEST_STRING".to_string()).unwrap();
        println!("{:#?}, len: {}", vrf, vrf.hash.len());
    }
    #[test]
    pub fn test_max_number() {
        let vrf = Vrf {
            proof: String::from(""),
            hash: String::from("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"),
        };
        println!("{:?}", vrf.numerical())
    }
}
