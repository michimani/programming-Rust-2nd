12 章 演算子オーバーロード
===

- オーバーロードに用いるのは `std::ops` および `std::cmp` トレイト
- `a + b` は `std::ops::Add` トレイトに含まれる `add` メソッドを使った `a.add(b)` の省略形
- 単項演算子
  - `std::ops::Not`, `std::ops::Neg`
- 二項演算子
  - `std::ops::Add`, `std::ops::Sub`, `std::ops::Mul`, `std::ops::Div` ...
- 複合代入演算子
  - `std::ops::AddAssign`, `std::ops::SubAssign` ...
- 等価性テスト
  - `==` : `std::cmp::PartialEq` の `eq` メソッド
  - `!=` : `std::cmp::PartialEq` の `ne` メソッド
  - PartialEq は対象を参照で受け取る
  - 通常は型パラメータは `sized` であることを要求するが
  
    ```rust
    where 
      Rhs: ?Sized,
    ```
    
    とすることで緩和できる
    
  - 数学的な Equal では `x == x は常に真である` ことを求めるが、 Rust では f32 や f64 の値において満たせない。よって **Partial**Eq という名前を使っている
- 順序比較
  - `std::cmp::PartialOrd`
  - PartialEq と同様の理由で **Partial** がついている
  - PartialOrd を実装している型のほとんどは Ord も実装している。 f32 と f64 だけが例外
- インデック操作
  - `a[i]` のような操作
  - `std::ops::Index` と `std::ops::IndexMut`