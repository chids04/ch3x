use std::time::SystemTime;
use std::fmt;
use crate::keys::Wallet;
use crate::utils::Hash32;



#[derive(Debug, PartialEq)]
pub struct Block{
    pub hash: Hash32,
    pub prev_hash: Hash32,
    pub nonce: u64,
    pub index: u64,
    //pub merkle_root: Hash32,
    pub miner_addr: String,
}

impl Block {
    //creates first block
    fn mine_genesis() -> Self {
        Block { 
            hash: Hash32::default(), 
            prev_hash: Hash32::default(), 
            nonce: 0, 
            index: 0,
            miner_addr: String::new(),
        }
    }

    //need to get transactions
    //compute block reward
    //add transactions to array

    pub fn mine(prev_block: &Block, miner_addr: &str, difficulty: usize) -> Self {
        let prev_hash = prev_block.hash;
        let index = prev_block.index+1;
        
        let mut nonce = 0;
        let mut hash = Hash32::from_string(format!("{}{}{}", prev_hash, index, nonce).as_str());

        let target = vec![7u8; difficulty];

        println!("target {:#?}", target);

        if target == hash.get_substr(difficulty){
            return Block{
                hash,
                prev_hash,
                nonce,
                index,
                miner_addr: miner_addr.to_owned(),
            };
        }

        //since only nonce changes, we can concat hash and index, before loop
        //this will help when threading
        let to_hash = format!("{prev_hash}{index}");

        loop {
            nonce+=1;

            hash = Hash32::from_string(format!("{to_hash}{nonce}").as_str());

            if target == hash.get_substr(difficulty){
                break;
            }   
            
        }

        Block { hash, prev_hash, nonce, index, miner_addr: miner_addr.to_owned() }

    }
}

impl Default for Block {
    fn default() -> Self {
        Block { 
            hash: Hash32::default(), 
            prev_hash: Hash32::default(), 
            nonce: 0, 
            index: 0,
            miner_addr: String::new(),
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\
        Block Index: {}\n\
        Block Hash: {}\n\
        Previous Block Hash: {}\n\
        Miner Address: {}\n\
        Nonce: {}\n", self.index, self.hash.to_string(), self.prev_hash.to_string(), self.miner_addr, self.nonce)
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn genesis_create(){
        let genesis = Block::mine_genesis();

        let block = Block {
            hash: Hash32::default(),
            prev_hash: Hash32::default(),
            nonce: 0,
            index: 0,
            miner_addr: "".to_string(),
        };

        assert_eq!(genesis, block);
    }

    // test mining a new block with a given difficulty
    #[test]
    fn mine_block_creates_valid_block() {
        let prev_block = Block::mine_genesis();
        let miner_addr = "miner1";
        let difficulty = 2; // low difficulty for test

        let block = Block::mine(&prev_block, miner_addr, difficulty);

        // check that the block's prev_hash matches the previous block's hash
        assert_eq!(block.prev_hash, prev_block.hash);

        // check that the block's index is incremented
        assert_eq!(block.index, prev_block.index + 1);

        // check that the miner address is set correctly
        assert_eq!(block.miner_addr, miner_addr);

        // check that the hash meets the difficulty requirement
        let target = vec![7u8; difficulty];
        assert_eq!(block.hash.get_substr(difficulty), target);
    }

    


}