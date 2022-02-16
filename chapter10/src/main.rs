use std::cmp::Ordering::{self, *};

fn main() {
    println!("Hello, world!");
}

// これは標準ライブラリに含まれている
// enum Ordering {
//     Less,
//     Equial,
//     Greater,
// }

fn compare(n: i32, m: i32) -> Ordering {
    if m < m {
        return Ordering::Less;
    } else if n > m {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}

// コードは短くなるが、明示的ではなくなる
fn compare_2(n: i32, m: i32) -> Ordering {
    if m < m {
        return Less;
    } else if n > m {
        return Greater;
    } else {
        return Equal;
    }
}

#[derive(PartialEq, Debug)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

#[test]
fn test_http_status_from_u32() {
    struct Case {
        n: u32,
        expect: Option<HttpStatus>,
    }

    let cases = [
        Case {
            n: 200,
            expect: Some(HttpStatus::Ok),
        },
        Case {
            n: 304,
            expect: Some(HttpStatus::NotModified),
        },
        Case {
            n: 404,
            expect: Some(HttpStatus::NotFound),
        },
        Case {
            n: 999,
            expect: None,
        },
    ];

    for c in cases {
        let s = http_status_from_u32(c.n);
        assert_eq!(s, c.expect)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RoughTime {
    InThePast(TimeUnit, u32), // タプルヴァリアント
    JustNow,
    InTheFuture(TimeUnit, u32), // タプルヴァリアント
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => {
            format!("{} {} ago", count, units.plural())
        }
        RoughTime::JustNow => {
            format!("just now")
        }
        RoughTime::InTheFuture(units, count) => {
            format!("{} {} from now", count, units.plural())
        }
    }
}

#[test]
fn test_rough_time() {
    let past = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);
    let future = RoughTime::InTheFuture(TimeUnit::Hours, 3);
    let just_now = RoughTime::JustNow;

    assert_eq!(rough_time_to_english(past), "87 years ago");
    assert_eq!(rough_time_to_english(future), "3 hours from now");
    assert_eq!(rough_time_to_english(just_now), "just now")
}

use std::collections::HashMap;

enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn binary_tree() {
    use self::BinaryTree::*;

    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));

    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "mars",
        left: jupiter_tree,
        right: Empty,
    }));

    let tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: Empty,
    }));

    println!("{:?}", tree);
}

#[test]
fn test_binary_tree() {
    binary_tree();
}

#[test]
fn test_match_pattern() {
    struct Case<T1, T2> {
        n: T1,
        expect: Option<T2>,
    }

    let cases = [
        Case {
            n: [0, 0, 0],
            expect: Some([0, 0, 0]),
        },
        Case {
            n: [1, 3, 0],
            expect: Some([1, 1, 1]),
        },
        Case {
            n: [0, 0, 255],
            expect: Some([255, 255, 255]),
        },
        Case {
            n: [1, 3, 255],
            expect: Some([1, 1, 1]),
        },
        Case {
            n: [2, 3, 5],
            expect: None,
        },
    ];

    for c in cases {
        let s = match c.n {
            [1, _, _] => Some([1, 1, 1]),
            [_, _, 0] => Some([0, 0, 0]),
            [_, _, 255] => Some([255, 255, 255]),
            _ => None,
        };
        assert_eq!(s, c.expect)
    }

    #[derive(Debug, PartialEq)]
    struct User {
        id: String,
        name: String,
        age: u8,
    }

    let cases2 = [
        Case {
            n: User {
                id: "id-1".to_string(),
                name: "name-1".to_string(),
                age: 10,
            },
            expect: Some(User {
                id: "id-1".to_string(),
                name: "name-1".to_string(),
                age: 10,
            }),
        },
        Case {
            n: User {
                id: "id-2".to_string(),
                name: "name-2".to_string(),
                age: 10,
            },
            expect: Some(User {
                id: "id-2".to_string(),
                name: "name-2".to_string(),
                age: 10,
            }),
        },
        Case {
            n: User {
                id: "id-3".to_string(),
                name: "name-3".to_string(),
                age: 20,
            },
            expect: None,
        },
    ];

    for c in cases2 {
        let s = match c.n {
            User { age: 10, .. } => Some(c.n),
            _ => None,
        };
        assert_eq!(s, c.expect)
    }

    let cases3 = [
        Case {
            n: 'a',
            expect: Some("char"),
        },
        Case {
            n: 'A',
            expect: Some("char"),
        },
        Case {
            n: '0',
            expect: Some("int"),
        },
        Case {
            n: '#',
            expect: None,
        },
    ];

    for c in cases3 {
        let s = match c.n {
            '0'..='9' => Some("int"),
            'a'..='z' | 'A'..='Z' => Some("char"),
            _ => None,
        };
        assert_eq!(s, c.expect)
    }
}
