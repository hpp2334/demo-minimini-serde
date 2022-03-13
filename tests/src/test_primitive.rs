use demo_minimini_serde::{from_tokens, Token};

#[test]
fn test_boolean() {
    let tokens = vec![Token::Boolean(false)];
    let r = from_tokens::<bool>(&tokens).unwrap();
    assert_eq!(r, false);
}

#[test]
fn test_i32() {
    let tokens = vec![Token::I32(1324457)];
    let r = from_tokens::<i32>(&tokens).unwrap();
    assert_eq!(r, 1324457);
}
