use rust_blockchain::{self, utils::*};

#[test]
fn test_hash_to_binary_1() {
    let input = &[0];
    assert_eq!(hash_to_binary_representation(input), "0");
}

#[test]
fn test_hash_to_binary_2() {
    let input = &[0, 1, 2, 3, 4, 5];
    assert_eq!(hash_to_binary_representation(input), "011011100101");
}

#[test]
fn test_hash_to_binary_3() {
    let input = &[255];
    assert_eq!(hash_to_binary_representation(input), "11111111");
}

#[test]
fn test_hash_to_binary_4() {
    let input = &[128];
    assert_eq!(hash_to_binary_representation(input), "10000000");
}