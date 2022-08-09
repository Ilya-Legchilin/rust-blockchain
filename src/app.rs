use crate::block::Block;

#[derive(PartialEq, Clone,)]
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

    pub fn is_chain_valid(&self) -> bool {
        let chain = &self.blocks;
        for i in 0..chain.len() {
            if i == 0 {
                continue;
            }
            let first = chain.get(i - 1).expect("has to exist");
            let second = chain.get(i).expect("has to exist");
            if !Block::is_block_valid(second, first) {
                return false;
            }
        }
        true
    }

    // We always choose the longest valid chain
    pub fn choose_chain(local: App, remote: App) -> App {
        let is_local_valid = local.is_chain_valid();
        let is_remote_valid = remote.is_chain_valid();

        if is_local_valid && is_remote_valid {
            if local.blocks.len() >= remote.blocks.len() {
                local
            } else {
                remote
            }
        } else if is_remote_valid && !is_local_valid {
            remote
        } else if !is_remote_valid && is_local_valid {
            local
        } else {
            panic!("local and remote chains are both invalid");
        }
    }
}
