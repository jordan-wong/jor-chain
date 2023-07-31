pub struct App {
    // our "Chain" is just and array
    pub blocks: Vec,
}

pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub nonce: u64,
}