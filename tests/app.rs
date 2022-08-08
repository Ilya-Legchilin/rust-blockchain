use chrono::Utc;
use rust_blockchain::{self, app::App};

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
fn test_try_to_add_block_1() {

}

#[test]
fn test_try_to_add_block_2() {
    
}

// #[test]
// #[should_panic("panic message")]
// fn test_try_to_add_block_3() {
    
// }

// #[test]
// #[should_panic("panic message")]
// fn test_try_to_add_block_4() {

// }

// #[test]
// fn test_is_block_valid_1() {

// }

// #[test]
// fn test_is_block_valid_2() {
    
// }

// #[test]
// #[should_panic("")]
// fn test_is_block_valid_3() {
    
// }

// #[test]
// #[should_panic("")]
// fn test_is_block_valid_4() {
    
// }