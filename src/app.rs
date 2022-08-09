use crate::block::Block;

pub struct App {
    pub blocks: Vec<Block>,
}

impl App {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn genesis(&mut self) {
        let genesis_block = Block::generate_genesis_block();
        self.blocks.push(genesis_block);
    }

    pub fn try_add_block(&mut self, block: Block) {
        let latest_block = self.blocks.last().expect("there is at least one block");
        if Block::is_block_valid(&block, latest_block) {
            self.blocks.push(block);
        } else {
            panic!("could not add block - invalid");
        }
    }
}
