use demo_minimini_serde::*;

#[derive(Debug, Deserialize, Serialize)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_deserialize_point() {
    // { x: 123, y: 456 }
    let tokens = vec![
        Token::LB,
        Token::String("x".to_owned()),
        Token::Colon,
        Token::I32(123),
        Token::Comma,
        Token::String("y".to_owned()),
        Token::Colon,
        Token::I32(456),
        Token::RB,
    ];
    let r: Point = from_tokens(&tokens).unwrap();
    assert_eq!(r.x, 123);
    assert_eq!(r.y, 456);
}

#[test]
fn test_serialize_point() {
    let r = Point { x: 123, y: 456 };
    let res = to_tokens(&r).unwrap();
    let tokens = vec![
        Token::LB,
        Token::String("x".to_owned()),
        Token::Colon,
        Token::I32(123),
        Token::Comma,
        Token::String("y".to_owned()),
        Token::Colon,
        Token::I32(456),
        Token::RB,
    ];
    assert_eq!(res, tokens);
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    vip: bool,
    id: i32,
    address: Address,
}

#[derive(Debug, Deserialize, Serialize)]
struct Address {
    nation: String,
    city: String,
}

#[test]
fn test_deserialize_user() {
    let tokens = vec![
        Token::LB,
        Token::String("vip".to_owned()),
        Token::Colon,
        Token::Boolean(true),
        Token::Comma,
        Token::String("id".to_owned()),
        Token::Colon,
        Token::I32(100),
        Token::Comma,
        Token::String("address".to_owned()),
        Token::Colon,
        Token::LB,
        Token::String("nation".to_owned()),
        Token::Colon,
        Token::String("China".to_owned()),
        Token::Comma,
        Token::String("city".to_owned()),
        Token::Colon,
        Token::String("Shanghai".to_owned()),
        Token::RB,
        Token::RB,
    ];
    let user: User = from_tokens(&tokens).unwrap();
    assert_eq!(user.id, 100);
    assert_eq!(user.vip, true);
    assert_eq!(user.address.nation, "China");
    assert_eq!(user.address.city, "Shanghai");
}

#[test]
fn test_serialize_user() {
    let user = User {
        vip: true,
        id: 100,
        address: Address {
            nation: "China".to_owned(),
            city: "Shanghai".to_owned(),
        },
    };

    let tokens = vec![
        Token::LB,
        Token::String("vip".to_owned()),
        Token::Colon,
        Token::Boolean(true),
        Token::Comma,
        Token::String("id".to_owned()),
        Token::Colon,
        Token::I32(100),
        Token::Comma,
        Token::String("address".to_owned()),
        Token::Colon,
        Token::LB,
        Token::String("nation".to_owned()),
        Token::Colon,
        Token::String("China".to_owned()),
        Token::Comma,
        Token::String("city".to_owned()),
        Token::Colon,
        Token::String("Shanghai".to_owned()),
        Token::RB,
        Token::RB,
    ];
    let res = to_tokens(&user).unwrap();
    assert_eq!(res, tokens);
}
