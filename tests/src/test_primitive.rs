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

#[test]
fn test_string() {
    let tokens = vec![Token::String("abdd#@#4!".to_owned())];
    let r = from_tokens::<String>(&tokens).unwrap();
    assert_eq!(&r, "abdd#@#4!");
}
