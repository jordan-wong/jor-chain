pub struct App {
    // our blockchain is stored in an array
    pub blocks: Vec,
}

pub struct Block {
    pub id: u64,
    pub timestamp: u64,
    pub hash: String,
    pub previous_hash: String,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    println("Hello!")
}

impol