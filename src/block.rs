#![allow(unused)]

use std::collections::LinkedList as List;

pub struct BlockChain {
    blocks: List<Block>
}

pub struct Block {
    pub hash: String,
    pub id: u128,
    pub transactions: List<Transaction>,
}

pub struct Transaction {
    inputs: List<TxIn>,
    outputs: List<TxOut>,
    txid: String,
}

struct TxIn {
    prev_txid: String,
    out: usize,
    signature: String, // to spend the output
}

struct TxOut {
    public_address: String,
    satoshis: u64, 
    // 1 btc = 10^8 satoshis, in total 10^8 * 21 * 10^6 = 2.1 * 10^15
    // maximum value of u64 is greater than 10^19
    // so u64 is enough to store all valid satoshis
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: List::new(), 
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push_back(block); 
    }

    pub fn get_block_by_id(&self, id:u128) -> Option<&Block> {
        self.blocks.iter().find(|&block| block.id == id)
    }

    pub fn get_block_by_hash(&self, hash: &str) -> Option<&Block> {
        self.blocks.iter().find(|&block| block.hash == hash)
    }
}
// Try to include bitcoin related functionalities like serialization, computing addresses etc.,
// You can add your own methods for different types and associated unit tests
