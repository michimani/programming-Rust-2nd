11章 トレイトとジェネリクス
===

## トレイト
- トレイト (trait) は Rust におけるインターフェース (もしくは抽象基底クラス)
- 何らかの機能、その方ができる機能を示す
- バイト列を書き出すトレイト (`std::io::Write`)

  ```rust
  trait Write {
    fn write(&mut self, buf:&[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    ...
  }
  ```

- これを実際に使うコード

  ```rust
  use std::fs::File;
  
  let mut local_file = File::create("hello.txt")?;
  say_hello(&mut local_file)?;
  
  let mut bytes = vec![];
  say_hello(&mut bytes)?;
  assert_eq!(bytes, b"hello world\n");
  ```

- トレイとメソッドを使うには、トレイトをスコープに入れる必要がある
  ```rust
  use std::io::Write; // これが必要
  
  let mut buf: Vec<u8> = vec![];
  buf.write_all(b"hello")?;
  ```

- Clone や Iterator は標準のプレリュードに含まれるため明示的に記述する必要がない
- `&mut dyn Write` のようなトレイト型への参照を、 **トレイトオブジェクト** と呼ぶ
  - 何らかの値を指す
  - 生存期間を持つ
  - 可変か、共有化のどちらか


## ジェネリクス
- ジェネリクス (generict) は、いわゆるジェネリクス

  ```rust
  fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
      value1
    } else {
      value2
    }
  }
  ```
  
  - `<T: Ord>` は `Ord` トレイトを実装した型 `T` を表わす
  - このような、型に対する要請を **制約 (bound)** と呼ぶ (境界 と訳す場合もある)
  
## ジェネリック関数と型パラメータ

```rust
fn say_hello(out: &mut dyn Write);    // 普通の関数

fn say_hello<W: Write>(out: &mut W);  // ジェネリック関数
```

- `W: Write` の部分が型パラメータ
- `W` は `Write` トレイトを実装したなにかしらの型
- 型パラメータは大文字一文字で書くのが慣習
- 引数を取るジェネリック関数では、実行時に型情報は省略できる
- 引数を取らないジェネリック関数は、実行時に型情報の記述が必要 (コンパイラが推論できない)
- 複数のトレイトを要求するには、 `+` を使う
  
  ```rust
  use std::hash::Hash;
  use std::fmt::Debug;
  
  fn top_ten<T: Debug + Hash + Eq>(values: &Vec<T>) { ... }
  ```
- ジェネリック関数は複数の型パラメータを持つことができる
- 複数になって複雑になった場合は `where` 節を使って記述できる (が、そうならないような実装を考えたほうがよい)

  ```rust
  fn run_query<M, R>(data: &Dataset, map: M, reduce: R) -> Results
    where M: Mapper + Serialize,
          R: Reducer + Serialize
  { ... }
  ```
- ジェネリック関数は生存期間パラメータと型パラメータの両方を持つことができる。生存期間パラメータを先に書く
- トレイトオブジェクトとジェネリック関数、どちらを使うか
  - 複数の型が入り混じっているコレクションを扱う場合には、トレイトオブジェクトを使うのが正しい
  - トレイトオブジェクトを使用すると、コンパイル後のコードサイズを小さくできる
  - ジェネリクスが優れている点
    - スピード (最大の利点)
      - `dyn` キーワードがないことで、動的ディスパッチが発生しない
    - すべてのトレイトがトレイとオブジェクトをサポートするとは限らない
    - ジェネリックの型パラメータに対して複数のトレイトを用いた型制約を同時に指定するのが容易
  - Rust ではジェネリック関数を採用することが多い
  
## トレイとの定義と実装

```rust
// 定義
trait Visible {
  fn draw(&self, canvas: &mut Canvas);
  
  fn hit_test(&self, x: i32, y: i32) -> bool;
}

// 実装 
// imple <トレイト> for <型>
impl Visible for Broom {
  fn draw(&self, canvas: Canvas) {
    for y in self.y - self.height - 1 .. self.y {
      canvas.write_at(self.x, y, '|');
    }
    canvas.write_at(self.s, self.y, 'M');
  }
  
  fn hit_test(&self, x: i32, y: i32) -> bool {
    self.x == x
    && self.y -self.height - 1 <= y,
    && y <= self.y>
  }
}
```

- トレイトのメソッドの実装しか書けない
- ヘルパ関数を実装したい場合、トレイトを使わない別の同名の型を定義して、そこに実装する

  ```rust
  impl Broom {
    fn broomstick_range(&self) -> Range<i32> {
      self.y - self.height - 1 .. self.y
    }
  }
  
  impl Visible for Broom {
    fn draw(&self, canvas: Canvas) {
      for y in self.broomstick_range() {
        ...
      }
      ...
    }
    ...
  }
  ```
- **拡張トレイト** : 既存の型にメソッドを追加するようなトレイト
  - トレイトか型のどちらかが、そのスコープ内およびクレート内で定義された場合に限る
  - `impl Write for u8` みたいなことはできない (`Write` も `u8` も標準ライブラリに含まれているため)
  - **孤児ルール (orphan rule)** という

## 型関連関数

- 引数に `&self` を取らない関数
- `::` 構文を使って実行する

## 完全修飾メソッド呼び出し

- 下記は同じ
  
  ```rust
  "hello".to_string();
  
  // 下記のことを 修飾メソッド呼び出し という
  str::to_string("hello");
  
  ToString::to_string("hello");
  
  // 特にこれを 完全修飾メソッド呼び出し という
  <str as ToString>::to_string("hello");
  ```