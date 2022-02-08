fn main() {
    println!("Hello, world!");
}

fn triangle_for(n: i32) -> i32 {
    let mut sum = 0;

    for i in 1..=n {
        sum = sum + i;
    }

    sum
}

fn triangle_fold(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

#[test]
fn test_triangle() {
    struct Case {
        n: i32,
        expect: i32,
    }

    let cases = [
        Case { n: 10, expect: 55 },
        Case { n: 0, expect: 0 },
        Case { n: 1, expect: 1 },
    ];

    for c in cases {
        let sfor = triangle_for(c.n);
        let sfold = triangle_fold(c.n);
        assert_eq!(sfor, c.expect);
        assert_eq!(sfold, c.expect);
    }
}

/// for と イテレータによる書き方
#[test]
fn test_for_iter() {
    let v = vec!["a", "b", "c", "d", "e"];

    for c in &v {
        print!("{}", c)
    }

    println!();

    let mut iter = (&v).into_iter();
    while let Some(c) = iter.next() {
        print!("{}", c)
    }

    println!();
}

/// std::path::Path の iter メソッド
#[test]
fn test_path_iter() {
    use std::ffi::OsStr;
    use std::path::Path;

    let path = Path::new("/Users/hoge/projects/programming-rust/chapter15");
    let mut iter = path.iter();
    assert_eq!(iter.next(), Some(OsStr::new("/")));
    assert_eq!(iter.next(), Some(OsStr::new("Users")));
    assert_eq!(iter.next(), Some(OsStr::new("hoge")));
    assert_eq!(iter.next(), Some(OsStr::new("projects")));
    assert_eq!(iter.next(), Some(OsStr::new("programming-rust")));
    assert_eq!(iter.next(), Some(OsStr::new("chapter15")));
    assert_eq!(iter.next(), None);
}

/// into_iter の挙動の違い
#[test]
fn test_into_iter() {
    let mut vector = vec!["alpha", "bravo", "charlie"];

    // 共有参照
    let mut imref = (&vector).into_iter();
    assert_eq!(imref.next(), Some(&"alpha"));
    assert_eq!(imref.next(), Some(&"bravo"));
    assert_eq!(imref.next(), Some(&"charlie"));
    assert_eq!(imref.next(), None);
    assert_eq!(format!("{}", &vector.len()), "3");

    let mut mref = (&mut vector).into_iter();
    assert_eq!(mref.next(), Some(&mut "alpha"));
    assert_eq!(mref.next(), Some(&mut "bravo"));
    assert_eq!(mref.next(), Some(&mut "charlie"));
    assert_eq!(mref.next(), None);
    assert_eq!(format!("{}", &vector.len()), "3");

    let mut real = vector.into_iter();
    assert_eq!(real.next(), Some("alpha"));
    assert_eq!(real.next(), Some("bravo"));
    assert_eq!(real.next(), Some("charlie"));
    assert_eq!(real.next(), None);
}

/// 値を生成するだけのイテレータ
#[test]
fn test_gen_rand() {
    use rand::random;
    use std::iter::from_fn;

    let lengths: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(10)
        .collect();

    for l in lengths {
        println!("{}", &l);
    }
}

#[test]
fn test_fibonacci() {
    // successors を使う場合
    use std::iter::successors;
    let limit: usize = 10;

    let init = (0, 1);
    for s in successors(Some(init), |&prev| Some((prev.1, prev.0 + prev.1)))
        .take(limit)
        .map(|(_, n)| n)
    {
        print!("{}", &s)
    }

    println!("");

    // from_fn を使う場合
    fn fibonacci() -> impl Iterator<Item = usize> {
        let mut state = (0, 1);
        std::iter::from_fn(move || {
            state = (state.1, state.0 + state.1);
            Some(state.0)
        })
    }

    for n in fibonacci().take(limit).collect::<Vec<_>>() {
        print!("{}", &n)
    }

    println!("");
}

#[test]
fn test_drain() {
    let mut hoge = "fugahoge".to_string();
    let fuga = String::from_iter(hoge.drain(0..4));

    assert_eq!(hoge, "hoge");
    assert_eq!(fuga, "fuga");
}

#[test]
fn test_filter() {
    let v: Vec<usize> = (0..=10)
        .filter(|n| *n % 2 == 0) // 偶数のみをフィルタするイテレータを生成
        .collect(); // 生成された値をベクタに集める

    assert_eq!(v, [0, 2, 4, 6, 8, 10])
}

#[test]
fn test_filter_map() {
    use std::str::FromStr;

    let text = "one 2 3 four five six 7 8 nine 10";
    let mut v = vec![];
    let iter = text
        .split_whitespace() // 空白で分割
        .filter_map(|n| i64::from_str(n).ok()); // i64 に変換できるものだけを値として生成する

    for num in iter {
        v.push(num)
    }

    assert_eq!(v, [2, 3, 4, 7, 8, 10])
}

#[test]
fn test_flat_map() {
    use std::collections::HashMap;

    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut divisor_map = HashMap::new();
    divisor_map.insert(1, vec![1]);
    divisor_map.insert(2, vec![1, 2]);
    divisor_map.insert(3, vec![1, 3]);
    divisor_map.insert(4, vec![1, 2, 4]);
    divisor_map.insert(5, vec![1, 5]);
    divisor_map.insert(6, vec![1, 2, 3, 6]);
    divisor_map.insert(7, vec![1, 7]);
    divisor_map.insert(8, vec![1, 2, 4, 8]);
    divisor_map.insert(9, vec![1, 3, 9]);
    divisor_map.insert(10, vec![1, 2, 5, 10]);

    let iter = nums
        .iter() // 1~10 のベクタのイテレータ
        .flat_map(|n| &divisor_map[n]); // 約数のベクタ内の値をひとつずつ生成する

    let mut v = vec![];
    for d in iter {
        v.push(*d)
    }

    assert_eq!(
        v,
        [1, 1, 2, 1, 3, 1, 2, 4, 1, 5, 1, 2, 3, 6, 1, 7, 1, 2, 4, 8, 1, 3, 9, 1, 2, 5, 10]
    )
}

#[test]
fn test_flatten() {
    // HashMap だと順序が固定されないので、ここでは BTreeMap を使う
    use std::collections::BTreeMap;

    let mut members = BTreeMap::new();
    members.insert("Bob", vec!["カレールウ", "白ごはん", "牛肉"]);
    members.insert("Ken", vec!["ニンジン"]);
    members.insert("Yui", vec!["タマネギ", "ジャガイモ"]);

    let curry_rice: Vec<_> = members.values().flatten().copied().collect();

    assert_eq!(
        curry_rice,
        vec![
            "カレールウ",
            "白ごはん",
            "牛肉",
            "ニンジン",
            "タマネギ",
            "ジャガイモ",
        ]
    )
}

#[test]
fn test_take_while() {
    let mut sum = 0;
    for num in (0..).take_while(|n| *n <= 10) {
        sum += num
    }

    assert_eq!(sum, 55)
}

#[test]
fn test_skip_skip_while() {
    let mut sum_skip = 0;
    for num in (1..=10).skip(8) {
        sum_skip += num
    }

    assert_eq!(sum_skip, 19);

    let mut sum_skip_while = 0;
    for num in (1..=10).skip_while(|n| *n < 5) {
        sum_skip_while += num
    }

    assert_eq!(sum_skip_while, 45);
}

#[test]
fn test_peekable() {
    use std::iter::Peekable;

    fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
    where
        I: Iterator<Item = char>,
    {
        let mut n = 0;
        loop {
            match tokens.peek() {
                // 次の値を見る
                Some(r) if r.is_digit(10) => {
                    // 値が存在して、かつ 0-9 の値かであるかをチェック
                    n = n * 10 + r.to_digit(10).unwrap(); // マッチするなら、 n の桁に追加する
                }
                _ => return n, // マッチしなければその時点の n を返す
            }
            tokens.next(); // 次の値へ
        }
    }

    let mut chars = "12345,67890".chars().peekable();
    assert_eq!(parse_number(&mut chars), 12345);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 67890);
}

#[test]
fn test_next_back_rev() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut numiter = numbers.iter();

    assert_eq!(numiter.next(), Some(&1));
    assert_eq!(numiter.next(), Some(&2));
    assert_eq!(numiter.next_back(), Some(&5));
    assert_eq!(numiter.next_back(), Some(&4));
    assert_eq!(numiter.next_back(), Some(&3));
    assert_eq!(numiter.next(), None);
    assert_eq!(numiter.next_back(), None);

    let mut revnum = numbers.iter().rev();
    assert_eq!(revnum.next(), Some(&5));
    assert_eq!(revnum.next(), Some(&4));
    assert_eq!(revnum.next_back(), Some(&1));
    assert_eq!(revnum.next_back(), Some(&2));
    assert_eq!(revnum.next_back(), Some(&3));
    assert_eq!(revnum.next(), None);
    assert_eq!(revnum.next_back(), None);
}

#[test]
fn test_inspect() {
    let mut v = vec![];

    for num in (1..=5)
        .inspect(|n| println!("before: {}", *n))
        .map(|n| n * n)
        .inspect(|n| println!("after: {}", *n))
    {
        v.push(num)
    }

    assert_eq!(v, [1, 4, 9, 16, 25])
}

#[test]
fn test_chain() {
    let v: Vec<i32> = (1..=5).chain(vec![10, 20, 30, 40, 50]).collect();
    assert_eq!(v, [1, 2, 3, 4, 5, 10, 20, 30, 40, 50])
}

#[test]
fn test_enumerate() {
    let numbers = [0, 1, 4, 9, 16, 25];
    let mut num_with_index = vec![];

    for ni in numbers.iter().enumerate() {
        num_with_index.push((ni.0, *ni.1))
    }

    assert_eq!(
        num_with_index,
        [(0, 0), (1, 1), (2, 4), (3, 9), (4, 16), (5, 25)]
    )
}

#[test]
fn test_zip() {
    let numbers = [0, 1, 4, 9, 16, 25];
    let index_iter = 0..;
    let mut num_with_index = vec![];

    for ni in index_iter.zip(numbers.iter()) {
        num_with_index.push((ni.0, *ni.1))
    }

    assert_eq!(
        num_with_index,
        [(0, 0), (1, 1), (2, 4), (3, 9), (4, 16), (5, 25)]
    )
}

#[test]
fn test_by_ref() {
    let numbers: Vec<i32> = (0..5).collect();
    let mut iter = numbers.iter();
    for n in iter.by_ref().take(2) {
        println!("{}", n);
    }

    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_cycle() {
    let index = 0..9;
    let day = ["おはよう", "こんにちは", "おやすみ"];
    let mut greet = vec![];

    for d in index.zip(day.iter().cycle()) {
        greet.push(*d.1)
    }

    assert_eq!(
        greet,
        [
            "おはよう",
            "こんにちは",
            "おやすみ",
            "おはよう",
            "こんにちは",
            "おやすみ",
            "おはよう",
            "こんにちは",
            "おやすみ"
        ]
    )
}
