13 章 ユーティリティトレイト
===

- 言語拡張トレイト
  - ユーザ定義の型を言語と統合するための、拡張点の役割を果たすトレイト
  - `Drop`, `Deref`, `DerefMut`, `From`, `Into`
- マーカトレイト
  - ジェネリックな型変数を制約するトレイト
  - `Sized`, `Copy`
- 共有語彙としてのトレイト
  - クレートやモジュールの公開インターフェイスで役に立つ
  - `Default`, `AsRef`, `AsMut`, `Borrow`, `BorrowMut`, `TryFrom`, `TryInto`, `ToOwned`

## Drop

- 値がドロップされた (所有者がいなくなった) ときの動作を独自実装できる
- C++ のデストラクタや、他の言語のファイナライザに似ている
- Drop トレイトを実装しているなら Copy トレイトを実装することはできない

## Sized

- Rust では殆どの型が siezed 型
- `u64` は 8バイト、 `(f32, f32, f32)` 型のタプルは 12 バイト
- `Vec<T>` は可変サイズのバッファをヒープ上に持つが、 `Vec<T>` 自体はバッファへのポインタと容量と長さのみなので sized 型
- `&str` は↑と同じ理由で sized
- `str` は unsized
- unsized の値を変数に格納したり引数として使うことはできない
- 殆どの方変数は sized にするべき
- コンパイラは `struct S<T> {...}` を `struct S<T: Sized> {...}` として暗黙的に解釈する

## Clone

- clone メソッドによって self の独立したコピーを返す
- clone メソッドの返り値は `Self` 型になっている
- Clone トレイトは Sized トレイトを拡張している
  - つまり、Slef の方を Sized に制約している
- Clone は保有しているものすべてをコピーするので、時間とメモリの両方で高価である (可能性がある)
  - `Vec<String>` の Clone はベクタだけでなく String 型の要素もコピーされる
- `s = t.clone()`
  - `t` をクローン、 `s` の古い値をドロップ、クローンされた値を `s` に移動
  - 可能なら `clone_from()` を使ったほうがよい
    - もともとの `s` が `t` を収めるのに十分なバッファを持っていれば、とりあえず移動させて `s` の長さを調節すればよい


## Copy

- Copy 型では、代入によって値が移動し移動元が未初期化状態になることはない
- Copy を実装できるのは、浅いバイト単位のコピーだけでコピーが可能な型に限られる
  - ヒープ上のバッファのコピーが必要な型は Copy を実装できない
- `#[derive(Copy)]` で簡単に実装できる
- Copy を実装した型は扱いやすいが、それを使った実装は強く制約され、暗黙のコピーが効果になる場合もあるので慎重に考えたほうが良い

## Deref と DerefMut

- `*` や `.` などの参照解決演算子の動作を指定できる
- DerefMut は Deref を拡張したもの (それはそう)
- 参照解決型変換
  - `Rc<String>` の値 `r` について、 `(*r).find('?')` ではなくて `r.find('?')` と書くだけで良い

## Default

- Go で言うゼロ値
- ベクタなら空ベクタ、文字列なら空文字、数値なら 0、Option なら None など
- 

## AsRef と AsMut

- ある型が `AsRef<T>` を実装している場合、 `&T` を効率的に借用できる

  ```rust
  trait AsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
  }

  trait AsMut<T: ?Sized> {
    fn as_mut(&mut self) -> &mut T;
  }
  ```

- `std::fs::File::open` の例

  ```rust
  fn open<P: AsRef<Path>>(path: P) -> Result<File>
  ```

  - `open()` が本当に必要なのは `&Path` だが、 `AsRef<Path>` を実装した型、つまり `String` や `str` などを受け取ることができる。つまり `open()` に文字列リテラルを直接渡すことができる
  
    ```rust
    let f = std::fs::File::open("/path/to/file")?;
    ```
  
## Borrow と BorrowMut

- AsRef トレイトに似ている
- ある型が `Borrow<T>` を実装している場合、 `&T` を効率的に借用できる
  - ハッシュ値や比較が、もとの値と同じ用に行える場合にだけ Borrow を実装するべき
  - String は `AsRef<str>`, `AsRef<[u8]>`, `AsRef<Path>` を実装しているが、 str のみハッシュ値が一致することが保証されている(?)ため `Borrow<str>` のみ実装している

## From と Into

- ある型の値を消費して別の方を返す変換を表わす

  ```rust
  trait Into<T>: Sized {
    fn into(self) -> T;
  }
  
  trait From<T>: Sized {
    fn from(other: T) -> Self;
  }
  ```
  
- すべての T 型は `From<T>` と `Into<T>` を実装している

## TryFrom と TryInto

- `i32` に対する `From<i64>` の実装は、情報が欠落する可能性があるので実装されていない
- 代わりに `TryFrom<i64>` が実装されており、 `from` の返り値は `Result<i32>`
- 失敗する可能性のある型変換を必要とする場合に実装する

