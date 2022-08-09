use chrono::Utc;
use rust_blockchain::{self, app::App, block::Block, utils::calculate_hash};

#[test]
fn test_new() {
    let app = App::new();
    assert_eq!(app.blocks, vec![]);
}

#[test]
fn test_genesis() {
    let mut app = App::new();
    app.genesis();
    assert_eq!(app.blocks.len(), 1);
    let first = app.blocks.first().unwrap(); 
    assert_eq!(first.id, 0);
    assert_eq!(first.hash, "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),);
    assert_eq!(first.previous_hash, String::from("genesis"));
    assert_eq!(first.timestamp, Utc::now().timestamp(),);
    assert_eq!(first.data, String::from("genesis!"));
    assert_eq!(first.nonce, 2836);
}

#[test]
#[should_panic(expected = "could not add block - invalid")]
fn test_try_to_add_block() {
    let mut app = App::new();
    app.genesis();
    let id = 1;
    let previous_hash = "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string();
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
    app.try_add_block(new_block);
}
