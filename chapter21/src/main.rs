#![allow(unused)]
fn main() {
    println!("Hello, world!");
}

#[test]
fn test_built_in_macro() {
    assert_eq!(concat!("a", "bc"), "abc");
    assert_eq!(option_env!("UNKNOWN"), None);
    assert_eq!(option_env!("CARGO_PKG_VERSION"), Some("0.1.0"));

    let sample_text = include_str!("../testdata/sample_text");
    assert_eq!(sample_text, "sample text");

    assert!(matches!((sample_text), (s)));
}

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

// $other:tt パターンにマッチする (文字列、数値、真偽値) のための定義
impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

// 数値型は一気に定義する
macro_rules! impl_from_num_for_json {
    ($($t:ident)*) => {
        $(
            impl From<$t> for Json {
                fn from(n: $t) -> Json {
                    Json::Number(n as f64)
                }
            }
        )*
    };
}
impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

#[recursion_limit = "256"]
macro_rules! json {
    (null) => {
        Json::Null
    };
    // これだと JSON としては正しくてもマッチしないものがある => トークンツリー `tt` を使う
    // ([$($element:expr), *]) => {
    //     Json::Array(vec![$($element), *])
    // };
    ([$($element:tt), *]) => {
        Json::Array(vec![$(json!($element)), *])
    };
    ([$($key:tt : $value:tt), *]) => {
        Json::Object(Box::new(vec![
            $(($key.to_string(), json!($value))), *
        ].into_iter().collect()))
    };
    ($other:tt) => {
        Json::from($other)
    };
}

#[test]
fn json_null() {
    assert_eq!(json!(null), Json::Null);
}

#[test]
fn test_array_with_json_element() {
    let macro_generated_valued = json!([{
        "pitch" : 440.0
    }]);

    let hand_coded_value = Json::Array(vec![Json::Object(Box::new(
        vec![("pitch".to_string(), Json::Number(440.0))]
            .into_iter()
            .collect(),
    ))]);

    assert_eq!(macro_generated_valued, hand_coded_value);
}
