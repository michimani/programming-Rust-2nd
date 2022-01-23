# 6 章

- Rust は (ほぼ) すべてが式 (= 値を返す)

## 6.8

- 生存期間 `'a` でループにラベル付けできる

  ```rust
  fn main() {
      let mut num = 0;

      'top: loop {
          if num > 200 {
              break;
          }

          if num < 100 {
              loop {
                  if num > 100 {
                      break 'top;
                  }
                  num += 1;
              }
          }

          num += 10;
      }

      assert_eq!(num, 101);
      // assert_eq!(num, 201); // 'top への break がなければこっち
  }
  ```

## 6.9

- 関数に明示的な `return` は不要
  - 関数のボディ部がブロック式と同じように機能するから
  

## 6.10

- `!` は **発散する関数 (divergent function)** を意味し、値を返さないことを意味する

  ```rust
  // std::proccess::exit() の実装
  fn exit(code: i32) -> !
  ```

## 6.11

- 型をコンパイラが推論できる場合には、型を省略したほうが良いと考えられている

## 6.12

- フィールドと要素へのアクセス

  ```rust
  user.name    // 構造体のフィールド
  categories.1 // タプルの要素
  members[i]   // 配列の要素
  ```
  - これらは各フィールド及び要素が mut 変数として定義されている場合には代入文の左辺に現れることができる (左辺値 (lvalue))
  
- 配列、ベクタ、スライスからスライスを取り出す

  ```rust
  let s = &v[0 .. 10];
  ```
  
  - `..` : RangeFull
  - `a ..` : RangeFrom{ start: a } 
  - `.. b` : RangeTo { end: b } (半開区間 = b は含まない)
  - `a .. b` : Range { start: a, end: b } (半開区間 = b は含まない)
  - `..= b` : RangeToInclusive { end: b } (閉区間 = b を含む)
  - `a ..= b` : RangeInclusive::new(a, b) (閉区間 = b を含む)
