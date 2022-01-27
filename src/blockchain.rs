use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn append_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn mine_block(&self, block: &mut Block) {
        while !block.validate() {
            block.increase_nonce();
        }
    }

    pub fn get_prev_hash(&self) -> String {
        match self.blocks.last() {
            None => "0".to_owned(),
            Some(latest_block) => latest_block.get_hash(),
        }
    }

    pub fn get_new_block(&self) -> Block {
        Block::new(self.get_prev_hash())
    }
}
