use blake2::{Blake2b, Digest};
pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash_item(&self) -> String {
        // First we calculate the bytes of the object being passed to us
        let bytes = self.bytes();

        let mut hasher = Blake2b::new();

        // write input message
        hasher.update(bytes);

        // read hash digest and consume hasher
        let res: Vec<u8> = hasher.finalize().into_iter().collect();

        // Finally we base 58 encode the result
        let hash: String = hex::encode(res);
        hash
    }
}
