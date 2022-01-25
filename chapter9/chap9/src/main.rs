fn main() {
    println!("Hello, world!");
}

/// FIFO Queue
pub struct Queue {
    older: Vec<char>,   // 古い要素
    younger: Vec<char>, // 新しい要素
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    /// キューの末尾の要素を push する
    pub fn push(&mut self, c: char) {
        self.younger.push(c)
    }

    /// キューの先端から要素を pop する
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // younger の要素を older に移し、順番を入れ替える
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

/// FIFO Queue (for any type)
pub struct GenericQueue<T> {
    older: Vec<T>,   // 古い要素
    younger: Vec<T>, // 新しい要素
}

impl<T> GenericQueue<T> {
    pub fn new() -> GenericQueue<T> {
        GenericQueue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    /// キューの末尾の要素を push する
    pub fn push(&mut self, c: T) {
        self.younger.push(c)
    }

    /// キューの先端から要素を pop する
    pub fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // younger の要素を older に移し、順番を入れ替える
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

// 定数パラメータを持つジェネリック構造体
struct Polunomial<const N: usize> {
    coefficients: [f64; N], // 各項の係数
}

impl<const N: usize> Polunomial<N> {
    fn new(coefficients: [f64; N]) -> Polunomial<N> {
        Polunomial { coefficients }
    }

    // x を与えて多項式の値をホーナー方で計算する
    fn eval(&self, x: f64) -> f64 {
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum
        }
        sum
    }
}

#[test]
fn test_polunomial() {
    let p = Polunomial::new([1.0, -2.0, 1.0]);

    struct Case {
        x: f64,
        expect: f64,
    }

    let cases = [
        Case {
            x: 1.0,
            expect: 0.0,
        },
        Case {
            x: 0.0,
            expect: 1.0,
        },
        Case {
            x: 4.0,
            expect: 9.0,
        },
    ];

    for c in cases {
        let e = p.eval(c.x);
        assert_eq!(e, c.expect)
    }
}

// トレイトの実装
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

#[test]
fn test_point() {
    let p1 = Point { x: 2.0, y: 2.2 };
    let p2 = Point { x: 2.0, y: 2.2 };
    let p3 = Point { x: 2.0, y: 2.1 };
    assert_eq!(format!("{:?}", p1), "Point { x: 2.0, y: 2.2 }");
    assert_eq!(p1, p2);
    assert!(p2 != p3);
}
