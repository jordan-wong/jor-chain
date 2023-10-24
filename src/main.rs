// fn main() {
//     println!("Hello, world!");
//     let my_name = "Jordan";
//     println!("My name is {my_name}");
// }

use chrono::Utc;
use log::{error, info, warn};

const DIFFICULTY_PREFIX: &str = "00";
fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}
fn calculate_hash(id: u64, timestamp: i64, prev_hash: &String, block_data: &String, nonce: u64) -> String{
    let mut res: String = String::default();
    res
}

#[derive(Debug)]
pub struct App {
    // our blockchain is stored in an array
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub struct Block {
    pub id: u64,
    pub timestamp: i64,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, previous_hash: String, data: String) {
        // let now = 
    }
    //println("Hello!");
}
impl App {
    fn new() -> Self {
        Self { blocks: vec![]}
        // same as Vec::new()
    }
    fn genesis(&mut self ) {
        let genesis_block = Block {
            id: 0,
            timestamp: Utc::now().timestamp(),
            previous_hash: String::from("genesis"),
            data: String::from("genesis!"),
            nonce: 2836,
            hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string()
        };
        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("there is a block on the chain");
        
        if self.is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        }
        else {
            error!("Could not add block - invalid");
        }

    }

    fn is_block_valid(&self, block: &Block, prev_block: &Block) -> bool{
        if block.previous_hash != prev_block.hash {
            warn!("block with id: {} has wrong prev hash", block.id);
            return false;
        }
        else if !hash_to_binary_representation(
            &hex::decode(&block.hash).expect("can decode from hex"))
            .starts_with(DIFFICULTY_PREFIX)
        {
            warn!("block with id: {} has invalid difficulty", block.id);
            return false;
        }
        else if block.id != prev_block.id + 1{
            warn!("block with id: {} is not the next block after the latest: {}", block.id, prev_block.id);
            return false;
        }
        else if block.hash != hex::encode(calculate_hash(
            block.id,
            block.timestamp,
            &block.previous_hash,
            &block.data,
            block.nonce
        )){
            warn!("block with id: {} has invalid hash", block.id);
            return false;
        }

        return true
    }
}

fn main() {
    let mut blockchain = App::new();
    // println!("my block: {}", blockchain.blocks);

    println!("{:?}", blockchain.blocks);

    blockchain.genesis();
    println!("{:?}", blockchain.blocks);
}
