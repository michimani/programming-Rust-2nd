fn main() {
    println!("Hello, world!");
}

#[test]
fn test_vec_1() {
    let mut numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let first = numbers[1]; // just Copy
    assert_eq!(first, 1);
    assert_eq!(numbers, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    {
        let first_5_nums = &numbers[0..5];
        let first_5_nums_2 = numbers[0..5].to_vec();
        assert_eq!(first_5_nums, [0, 1, 2, 3, 4]);
        assert_eq!(first_5_nums_2, [0, 1, 2, 3, 4]);
        assert_eq!(numbers, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    // first()
    assert_eq!(numbers.first(), Some(&0));
    // last()
    assert_eq!(numbers.last(), Some(&9));
    // get()
    assert_eq!(numbers.get(5), Some(&5));
    assert_eq!(numbers.get(100), None);
    // first_mut()
    assert_eq!(numbers.first_mut(), Some(&mut 0));
    // last_mut()
    assert_eq!(numbers.last_mut(), Some(&mut 9));
    // get_mut()
    assert_eq!(numbers.get_mut(5), Some(&mut 5));
    assert_eq!(numbers.get_mut(100), None);
}

#[test]
fn test_vec_2() {
    // len, capacity
    let mut numbers = vec![0, 1, 2];
    assert_eq!(numbers.len(), 3);
    assert_eq!(numbers.capacity(), 3);
    // pop all item
    numbers.pop();
    numbers.pop();
    numbers.pop();
    assert_eq!(numbers.len(), 0);
    assert!(numbers.is_empty());
    assert_eq!(numbers.capacity(), 3);
    // fit capacity
    numbers.shrink_to_fit();
    assert_eq!(numbers.capacity(), 0);
    // push 5 items
    for _ in 0..5 {
        numbers.push(1);
    }
    assert_eq!(numbers.len(), 5);
    assert!(numbers.capacity() >= 5);

    // reserve capacity
    numbers.reserve(10); // 今の要素数 5 + 確保したいサイズ 10 = 15 以上のキャパシティになる
    assert!(numbers.capacity() >= 15);
    // add more 10 items
    for _ in 0..10 {
        numbers.push(1);
    }
    assert_eq!(numbers.len(), 15);
    assert!(numbers.capacity() >= 15); // push しているなかで拡張されている可能性がある

    // reserve exact capacity
    numbers.reserve_exact(15); // 今の要素数 15 + 確保したいサイズ 15 = 30 キャパシティになる (余分は持たない)
    assert_eq!(numbers.capacity(), 30);
    // add more 15 items
    for _ in 0..15 {
        numbers.push(1);
    }
    assert_eq!(numbers.len(), 30);
    assert_eq!(numbers.capacity(), 30); // 途中で拡張されず、30 まで
}

#[test]
fn test_vec_3() {
    // insert, remove, resize, truncate, clear, extend, split_off
    let mut numbers = vec![0, 0, 0];
    numbers.insert(1, 1);
    assert_eq!(numbers, [0, 1, 0, 0]);
    numbers.remove(2);
    assert_eq!(numbers, [0, 1, 0]);

    numbers.resize(5, 2);
    assert_eq!(numbers, [0, 1, 0, 2, 2]);

    numbers.resize_with(7, || 2 * 2);
    assert_eq!(numbers, [0, 1, 0, 2, 2, 4, 4]);

    numbers.truncate(2);
    assert_eq!(numbers, [0, 1]);

    numbers.clear();
    assert!(numbers.is_empty());

    numbers.extend((0..=10).filter(|n| n % 2 == 0));
    assert_eq!(numbers, [0, 2, 4, 6, 8, 10]);

    let mut off = numbers.split_off(3);
    assert_eq!(numbers, [0, 2, 4]);
    assert_eq!(off, [6, 8, 10]);

    numbers.append(&mut off);
    assert_eq!(numbers, [0, 2, 4, 6, 8, 10]);
    assert!(off.is_empty());
    assert_eq!(off.capacity(), 3);
}

#[test]
fn test_vec_4() {
    // drain, retain, dedup
    let mut numbers = vec![1, 2, 2, 3, 3, 3, 9, 9, 9, 9, 9, 4, 4, 4, 4, 5, 5, 5, 5, 5];

    let nines: Vec<i32> = numbers.drain(6..=10).collect();
    assert_eq!(nines, [9, 9, 9, 9, 9]);
    assert_eq!(numbers, [1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5]);

    numbers.retain(|n| *n < 5);
    assert_eq!(numbers, [1, 2, 2, 3, 3, 3, 4, 4, 4, 4]);

    numbers.dedup();
    assert_eq!(numbers, [1, 2, 3, 4]);
}

#[test]
fn test_slice_1() {
    let s1 = [1, 1];
    let s2 = [2, 2];
    let s3 = [3, 3];

    assert_eq!([s1, s2, s3].concat(), [1, 1, 2, 2, 3, 3]);
    assert_eq!([s1, s2, s3].join(&9), [1, 1, 9, 2, 2, 9, 3, 3])
}

#[test]
fn test_slice_2() {
    // split_at
    let s = [0, 1, 2, 3, 4, 5];
    let res_split_at = s.split_at(3);
    assert_eq!(res_split_at.0, &[0, 1, 2]);
    assert_eq!(res_split_at.1, &[3, 4, 5]);

    // split_first
    let res_split_first = s.split_first();
    assert_eq!(res_split_first.unwrap().0, &0);
    assert_eq!(res_split_first.unwrap().1, &[1, 2, 3, 4, 5]);

    // split
    let mut res_split = s.split(|n| *n % 2 == 0);
    assert_eq!(res_split.next().unwrap(), &[]);
    assert_eq!(res_split.next().unwrap(), &[1]);
    assert_eq!(res_split.next().unwrap(), &[3]);
    assert_eq!(res_split.next().unwrap(), &[5]);
    assert_eq!(res_split.next(), None);

    // split_inclusive
    let mut res_split_inclusive = s.split_inclusive(|n| *n % 2 == 0);
    assert_eq!(res_split_inclusive.next().unwrap(), &[0]);
    assert_eq!(res_split_inclusive.next().unwrap(), &[1, 2]);
    assert_eq!(res_split_inclusive.next().unwrap(), &[3, 4]);
    assert_eq!(res_split_inclusive.next().unwrap(), &[5]);
    assert_eq!(res_split_inclusive.next(), None);
}

#[test]
fn test_slice_3() {
    let s = [0, 1, 2, 3, 4, 5, 6];

    // chunks
    let mut res_chunks = s.chunks(2);
    assert_eq!(res_chunks.next().unwrap(), &[0, 1]);
    assert_eq!(res_chunks.next().unwrap(), &[2, 3]);
    assert_eq!(res_chunks.next().unwrap(), &[4, 5]);
    assert_eq!(res_chunks.next().unwrap(), &[6]);
    assert_eq!(res_chunks.next(), None);

    // chunks_exact
    let mut res_chunks_exact = s.chunks_exact(2);
    assert_eq!(res_chunks_exact.next().unwrap(), &[0, 1]);
    assert_eq!(res_chunks_exact.next().unwrap(), &[2, 3]);
    assert_eq!(res_chunks_exact.next().unwrap(), &[4, 5]);
    assert_eq!(res_chunks_exact.next(), None);
    assert_eq!(res_chunks_exact.remainder(), &[6]);
    assert_eq!(res_chunks_exact.next(), None);

    // windows
    let mut res_windows = s.windows(3);
    assert_eq!(res_windows.next().unwrap(), &[0, 1, 2]);
    assert_eq!(res_windows.next().unwrap(), &[1, 2, 3]);
    assert_eq!(res_windows.next().unwrap(), &[2, 3, 4]);
    assert_eq!(res_windows.next().unwrap(), &[3, 4, 5]);
    assert_eq!(res_windows.next().unwrap(), &[4, 5, 6]);
    assert_eq!(res_windows.next(), None);
}

#[test]
fn test_slice_4() {
    let mut s = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    s.swap(0, 6);
    assert_eq!(s, [6, 1, 2, 3, 4, 5, 0, 7, 8, 9]);

    s.swap_remove(3);
    assert_eq!(s, [6, 1, 2, 9, 4, 5, 0, 7, 8]);

    s.fill(0);
    assert_eq!(s, [0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn test_sort() {
    // sort
    let mut n = vec![1, 0, 4, 5, 2, 3, 9, 8, 7];
    n.sort();
    assert_eq!(n, [0, 1, 2, 3, 4, 5, 7, 8, 9]);

    // sort_by
    #[derive(Debug, PartialEq)]
    struct User {
        name: String,
        age: i32,
    }

    let mut u = vec![
        User {
            name: "aaa".to_string(),
            age: 42,
        },
        User {
            name: "bbb".to_string(),
            age: 12,
        },
        User {
            name: "ccc".to_string(),
            age: 30,
        },
    ];
    // DESC
    u.sort_by(|b, a| a.name.cmp(&b.name));
    assert_eq!(
        u,
        vec![
            User {
                name: "ccc".to_string(),
                age: 30,
            },
            User {
                name: "bbb".to_string(),
                age: 12,
            },
            User {
                name: "aaa".to_string(),
                age: 42,
            },
        ]
    );

    u.sort_by_key(|a| a.age);
    assert_eq!(
        u,
        vec![
            User {
                name: "bbb".to_string(),
                age: 12,
            },
            User {
                name: "ccc".to_string(),
                age: 30,
            },
            User {
                name: "aaa".to_string(),
                age: 42,
            },
        ]
    );

    assert!(u.contains(&User {
        name: "aaa".to_string(),
        age: 42,
    }));
}

#[test]
fn test_vec_deque() {
    use std::collections::VecDeque;

    let v = vec![0, 1, 2, 3, 4];
    let mut d = VecDeque::from(v);
    assert_eq!(d.pop_front(), Some(0));
    assert_eq!(d.pop_back(), Some(4));
    d.push_front(5);
    d.push_back(6);
    assert_eq!(d, [5, 1, 2, 3, 6]);
    let v2 = Vec::from(d);
    assert_eq!(v2, [5, 1, 2, 3, 6]);
}

#[test]
fn test_binary_heap() {
    use std::collections::BinaryHeap;

    let mut bh = BinaryHeap::from(vec![0, 9, 1, 2, 3, 8, 4, 7, 5, 6]);
    let iter = bh.iter();
    println!("BinaryHeap iterator: start");
    for n in iter {
        print!("{}", n)
    }
    println!("\nBinaryHeap iterator: end");

    assert_eq!(bh.peek(), Some(&9));
    let mut prev = bh.pop().unwrap();
    while let Some(p) = bh.pop() {
        assert!(p <= prev);
        prev = p;
    }
    assert_eq!(bh.len(), 0);
}

#[test]
fn test_set() {
    use std::collections::HashSet;

    let mut hs: HashSet<i32> = HashSet::new();
    assert_eq!(hs.insert(1), true);
    assert_eq!(hs.insert(2), true);
    assert_eq!(hs.insert(3), true);
    assert_eq!(hs.insert(3), false);
    assert_eq!(hs.contains(&2), true);
    assert_eq!(hs.remove(&2), true);
    assert_eq!(hs.remove(&100), false);
}

#[test]
fn test_set_2() {
    use std::collections::HashSet;
    let hs3: HashSet<i32> = (1..=30).filter(|n| *n % 3 == 0).collect();
    let hs5: HashSet<i32> = (1..=30).filter(|n| *n % 5 == 0).collect();

    // 積集合
    let hs15 = &hs3 & &hs5;
    // 下記は上記と同じ
    // let mut hs15: HashSet<i32> = HashSet::new();
    // for n in hs3.intersection(&hs5) {
    //     hs15.insert(*n);
    // }
    assert_eq!(hs15.len(), 2);
    assert!(hs15.contains(&15));
    assert!(hs15.contains(&30));

    // 和集合
    let hs35 = &hs3 | &hs5;
    assert_eq!(hs35.len(), 14);

    // 差集合 (h1 のみに含まれるもの)
    let hs33 = &hs3 - &hs5;
    assert_eq!(hs33.len(), 8);

    // 対称差集合
    let hs3355 = &hs3 ^ &hs5;
    assert_eq!(hs3355.len(), 12);
}

#[test]
fn test_set_3() {
    use std::collections::HashSet;

    let a: HashSet<i32> = (2..20).filter(|n| *n % 2 == 0).collect();
    let b: HashSet<i32> = (2..20).filter(|n| *n % 8 == 0).collect();
    let c: HashSet<i32> = (2..20).filter(|n| *n % 11 == 0).collect();

    // a ∩ c = ∅
    assert!(a.is_disjoint(&c));

    // b ⊂ a
    assert!(b.is_subset(&a));

    // a ⊃ b
    assert!(a.is_superset(&b));
}

#[test]
fn test_hash() {
    use std::collections::HashSet;
    use std::hash::{Hash, Hasher};

    struct User {
        id: String,
        name: String,
        age: i32,
    }

    // ID が等しければ同じユーザーとみなす
    impl PartialEq for User {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Eq for User {}

    // ID をもとにハッシュ値を生成する
    impl Hash for User {
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            self.id.hash(hasher)
        }
    }

    let mut us = HashSet::<User>::new();
    assert!(us.insert(User {
        id: "111".to_string(),
        name: "user111".to_string(),
        age: 10,
    }));
    assert!(us.insert(User {
        id: "222".to_string(),
        name: "user222".to_string(),
        age: 20,
    }));
    assert!(!us.insert(User {
        id: "111".to_string(),
        name: "user111".to_string(),
        age: 10,
    }));
    assert_eq!(us.len(), 2);
    assert!(us.contains(
        &(User {
            id: "111".to_string(),
            name: "user111".to_string(),
            age: 10,
        })
    ))
}
