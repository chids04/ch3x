mod keys;
mod block;
mod blockchain;
mod utils;
mod transaction;


use block::Block;
use blockchain::Blockchain;
use keys::Wallet;

fn main() {

    let mut bc = Blockchain::new();
    let miner = Wallet::new().expect("failed to create wallet");
    let miner_addr = miner.get_addr().expect("error getting miner address");
    println!("Miner address: {}", miner_addr);


    let block = Block::mine(&bc.prev_block(), &miner_addr, bc.difficulty);
    bc.add_block(block);


    if let Some(genesis) = bc.genesis() {
        println!("genesis block:\n{genesis}")
    }
    else{
        println!("no blocks found")
    }
    
    if let Some(first) = bc.head() {
        println!("first block:\n{first}")
    }
    else{
        println!("no blocks found")
    }



}
