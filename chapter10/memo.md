10 章
===

## C スタイルの列挙型

```rust
#[derive(PartialEq, Debug)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}
```

- メモリ上では整数
- 値を明示的に指定しなければ、コンパイラが 0 から順番に値を決める
- 列挙型 -> 整数型 のキャストは可能。逆は不可。
  - 自分でチェックしながらのキャストが必要
  
    ```rust
    fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
        match n {
            200 => Some(HttpStatus::Ok),
            304 => Some(HttpStatus::NotModified),
            404 => Some(HttpStatus::NotFound),
            _ => None,
        }
    }
    ```

## データ型を保持する列挙型

- タプルヴァリアント
- 構造体型バリアント
- 列挙型に値もしくはフィールドを持てる
- 一つの列挙型の中にこれらが混ざっていても良い

## 列挙型を用いたデータ構造

- 列挙型はツリー状のデータ構造を簡単に実装するのにも向いている
- JSON データを扱う例

  ```rust
  use std::collections::HashMap;

  enum Json {
      Null,
      Boolean(bool),
      Number(f64),
      String(String),
      Array(Vec<Json>),
      Object(Box<HashMap<String, Json>>),
  }
  ```
  
  - JSON ドキュメントに現れるデータ型を規定している
  - C++ で同じことを書こうとすると 30 行以上になる

## ジェネリック列挙型

- 標準ライブラリでは `Result`, `Option` がそれ

  ```rust
  enum Result<T> {
    Ok(T),
    Err(E),
  }

  enum Option<T> {
    None,
    Some(T),
  }
  ```

## パターンマッチのパターン型いろいろ

- p218〜
- マッチする値を使用しない場合 `_` によるワイルドカードパターンを使う
- パターンの最後にワイルドカードパターンを置く必要がある場合も多い
- そのパターンに到達しないと確信している場合でも、ワイルドカードパターンによる分岐を最後において panic させるとよい
- 反駁不能パターン
  - マッチすることが保証されている
  - `let` のうしろ、関数引数 `for` の後ろ、クロージャの引数 で使える
- 反駁可能パターン
  - マッチしないかもしれない
  - 例:
    - `Ok(x)` はエラー結果にはマッチしない
    - `'0'..='9'` は文字 `'Q'` にはマッチしない
  - `match` 式で使える
  - `if let` 式や `while let` 式でも使える
