use crate::block::Block;

pub struct Blockchain{
    pub blocks: Vec<Block>,
    pub difficulty: usize,
    //add pending transactions array
}

impl Blockchain {
    pub fn new() -> Self{
        Blockchain { blocks: vec![Block::default()], difficulty: 1 }
    }

    pub fn prev_block(&self) -> &Block {
        if let Some(block) = self.blocks.last() {
            return block;
        }
        else{
            panic!("missing previous block");
        }
    }

    pub fn add_block(&mut self, block: Block){ 
        self.blocks.push(block)
    }

    pub fn genesis(&self) -> Option<&Block> {
        self.blocks.first()
    }

    pub fn head(&self) -> Option<&Block> {
        self.blocks.last()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    // test that the blockchain is initialized with one block (the genesis block)
    #[test]
    fn genesis_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.blocks.len(), 1);
    }

    // test that the difficulty is set correctly on initialization
    #[test]
    fn difficulty_is_set() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.difficulty, 1);
    }

    // test that the genesis block is the default block
    #[test]
    fn genesis_block_is_default() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.blocks[0], Block::default());
    }

}

