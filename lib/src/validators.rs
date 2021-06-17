use crate::block::Block;

struct Validator {
    address: String,
    age: u64,
    delegated_to: String,
    online: bool
}

pub fn choose_next_proposer(block: Block) {
    // take the VRF hash from the 
}
