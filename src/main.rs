mod linked_list;
mod block;
mod mresult;

fn main() {
    println!("Hello, world!");
    let mut blockchain = block::BlockChain::new(); 
    let block = block::Block {
        hash: "0000000000000000000".to_string(),
        id: 1,
        transactions: std::collections::LinkedList::new(),
    };
    blockchain.add_block(block); 
    if let Some(block) = blockchain.get_block_by_id(1) {
        println!("Block found: {:?}", block.hash);
    } else {
        println!("Block not found");
    }

}
