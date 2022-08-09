use chrono::Utc;
use rust_blockchain::{self, block::Block, utils::calculate_hash};

#[test]
fn test_generate_genesis() {
    let genersis = Block::generate_genesis_block();
    assert_eq!(genersis.id, 0);
    assert_eq!(
        genersis.hash,
        "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    );
    assert_eq!(genersis.previous_hash, String::from("genesis"));
    assert_eq!(genersis.timestamp, Utc::now().timestamp(),);
    assert_eq!(genersis.data, String::from("genesis!"));
    assert_eq!(genersis.nonce, 2836);
}

#[test]
fn test_is_block_valid() {
    let id = 1;
    let previous_hash =
        "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string();
    let data = String::from("first block!");
    let nonce = 1;
    let timestamp = 1;
    let hash = calculate_hash(id, timestamp, previous_hash.as_str(), data.as_str(), nonce);
    let new_block = Block {
        id,
        hash: hex::encode(hash),
        previous_hash,
        timestamp,
        data,
        nonce,
    };
    let genesis_block = Block::generate_genesis_block();
    assert!(!Block::is_block_valid(&new_block, &genesis_block));
}

#[test]
fn test_new_block() {
    let genesis_block = Block::generate_genesis_block();
    let next_block = Block::new(1, genesis_block.hash.clone(), "next_block".to_string());
    Block::is_block_valid(&next_block, &genesis_block);
}
