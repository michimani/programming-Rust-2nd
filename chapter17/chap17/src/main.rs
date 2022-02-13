fn main() {
    println!("Hello, world!");
}

#[test]
fn test_char() {
    let t = "うどん".to_string();
    let mut c = t.chars();
    assert_eq!(c.next(), Some('う'));
    assert_eq!(c.next(), Some('ど'));
    assert_eq!(c.next(), Some('ん'));
    assert_eq!(c.next(), None);

    let mut c2 = "+A1d あ".chars();
    assert!(c2.next().unwrap().is_ascii()); // +
    assert!(c2.next().unwrap().is_uppercase()); // A
    assert!(c2.next().unwrap().is_numeric()); // 1
    assert!(c2.next().unwrap().is_lowercase()); // d
    assert!(c2.next().unwrap().is_whitespace()); //
    assert!(!c2.next().unwrap().is_ascii()); // あ

    assert_eq!('8'.to_digit(10), Some(8)); // '8' は 10進数で 8
    assert_eq!('8'.to_digit(2), None); // '8' は 2進数では数字でない
    assert_eq!('f'.to_digit(10), None); // 'f' は 10進数では数字ではない
    assert_eq!('f'.to_digit(16), Some(15)); // 'f' は 16進数では 15
    assert_eq!(std::char::from_digit(8, 10), Some('8')); // 8 は 10進数で '8'
    assert_eq!(std::char::from_digit(10, 16), Some('a')); // 10 は 16進数で 'a'
}

#[test]
fn test_string_1() {
    let s1 = String::new();
    assert_eq!(s1.len(), 0);
    assert_eq!(s1.capacity(), 0);

    let s2 = String::with_capacity(10);
    assert_eq!(s2.len(), 0);
    assert_eq!(s2.capacity(), 10);

    let mut s3 = "sample text".to_string();
    assert_eq!(s3.len(), 11);
    println!("{}", s3.capacity());
    assert!(s3.capacity() == 11);
    s3.push('.');
    assert_eq!(s3.len(), 12);
    println!("{}", s3.capacity());
    assert!(s3.capacity() > 12);
    assert_eq!(s3, "sample text.")
}

#[test]
fn test_string_2() {
    let s1 = "こんにちは".to_string();
    assert_eq!(s1.len(), 15);

    assert!(!s1.is_char_boundary(7));
    // let sp = s1.split_at(7); // インデックス 7 は文字の境界ではないため panic
    assert!(s1.is_char_boundary(6));
    let sp = s1.split_at(6);
    assert_eq!(sp.0, "こん");
    assert_eq!(sp.1, "にちは");
}

#[test]
fn test_string_3() {
    use std::fmt::Write;

    let mut s1 = String::new();
    write!(s1, "Hello").unwrap();
    write!(s1, " Rust").unwrap();
    assert_eq!(s1, "Hello Rust");
    s1.insert_str(5, " World, and");
    assert_eq!(s1, "Hello World, and Rust");
}

#[test]
fn test_string_4() {
    let mut s = "Hello world.".to_string();
    s.replace_range(6..11, "Rust");
    assert_eq!(s, "Hello Rust.");

    let period = s.pop();
    assert_eq!(period, Some('.'));
    assert_eq!(s, "Hello Rust");
}

#[test]
fn test_pattern() {
    let s = "Hello Great Lang Rust.".to_string();

    assert!(s.contains('R'));
    assert!(s.contains("Rust"));
    assert!(!s.contains("Go")); // 文字列 "Go" としてのパターン
    assert!(s.contains(['G', 'o'].as_ref())); // 文字 'G' と 'o' としてのパターン
    assert!(s.contains(&['G', 'o'][..])); // ↑と同じ

    assert_eq!(s.find('t'), Some(10));
    assert_eq!(s.rfind('t'), Some(20));

    let s1 = s.replace(" Great Lang", "");
    assert_eq!(s1, "Hello Rust.");

    assert_eq!("xxxxx".replacen('x', "X", 2), "XXxxx");
}

#[test]
fn test_parse() {
    use std::str::FromStr;

    assert_eq!(bool::from_str("true"), Ok(true));
    assert!(bool::from_str("TRUE").is_err());
    assert_eq!(usize::from_str("12345"), Ok(12345));
    assert!(usize::from_str("hoge").is_err());
}

#[test]
fn test_parse_2() {
    assert_eq!(format!("{}", 120), "120");
    assert_eq!(120.to_string(), "120");
    assert_eq!(format!("{:?}", vec![1, 2, 3]), "[1, 2, 3]");
}

#[test]
fn test_parse_3() {
    use std::str;

    assert_eq!(str::from_utf8(&[2, 4, 0][..]), Ok("\u{2}\u{4}\u{0}"));

    assert_eq!(str::from_utf8(&[240, 159, 141, 155][..]), Ok("🍛"));
    assert!(str::from_utf8(&[0, 188][..]).is_err());

    assert_eq!(
        String::from_utf8(vec![240, 159, 141, 163]),
        Ok("🍣".to_string())
    );
    assert!(String::from_utf8(vec![240, 159, 141, 163, 222]).is_err());

    assert_eq!(String::from_utf8_lossy(&[222][..]), "�");
}

#[test]
fn test_format() {
    assert_eq!(format!("{:15}", "Hello Rust."), "Hello Rust.    ");
    assert_eq!(format!("{:.<15}", "Hello Rust."), "Hello Rust.....");
    assert_eq!(format!("{:>15}", "Hello Rust."), "    Hello Rust.");
    assert_eq!(format!("{:.5}", "Hello Rust."), "Hello");

    assert_eq!(format!("{:0>8}", 9.987), "0009.987");
    assert_eq!(format!("{:0>8.2}", 9.987), "00009.99");
    assert_eq!(format!("{:0>8.1}", 9.987), "000010.0");
    assert_eq!(format!("{:0>8.0}", 9.987), "00000010");

    assert_eq!(format!("{:x}", 255), "ff");
    assert_eq!(format!("{:X}", 255), "FF");
    assert_eq!(format!("{:#x}", 255), "0xff");
    assert_eq!(format!("{:#X}", 255), "0xFF");

    let s = "sample text";
    assert_eq!(s, "sample text");
    println!("{:p}", s);

    assert_eq!(format!("{2} {1} {0} {0} {1} {2}", 0, 1, 2), "2 1 0 0 1 2");
}

#[test]
fn test_format_2() {
    use std::fmt;

    struct User {
        name: String,
        age: i32,
    }

    impl fmt::Display for User {
        fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
            write!(dest, "User {{name: {0}, age: {1}}}", self.name, self.age)
        }
    }

    assert_eq!(
        format!(
            "{}",
            User {
                name: "user-1".to_string(),
                age: 20
            }
        ),
        "User {name: user-1, age: 20}"
    );
}

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SEMVER: Regex = Regex::new(r"([a-z0-9]+)\.(jpg|png|svg)$").expect("error");
}

fn is_image(path: &str) -> Result<bool, regex::Error> {
    Ok(SEMVER.is_match(path))
}

fn name_and_ex(path: &str) -> Option<(String, String)> {
    match is_image(path) {
        Ok(b) => {
            if b {
                let cap = SEMVER.captures(path).unwrap();
                Some((cap[1].to_string(), cap[2].to_string()))
            } else {
                None
            }
        }
        _ => None,
    }
}

#[test]
fn test_regex() {
    assert!(is_image("hoge.jpg").unwrap());
    assert!(!is_image("hoge.txt").unwrap());

    let o1 = name_and_ex("fuga.jpg");
    let o = o1.unwrap();
    assert_eq!(o.0, "fuga");
    assert_eq!(o.1, "jpg");

    let o2 = name_and_ex("fuga.txt");
    assert_eq!(o2, None);
}
