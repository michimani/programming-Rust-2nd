# 7 章

## 7.1

- パニックが起きたときの動作は、「スタックを巻き戻す」か「プロセスをアボートする」かの 2 通りが考えられる
  - デフォルトは巻き戻す方
- `RUNTASK_BACKTRACE=1` を設定して実行するとパニック時のスタックのダンプが出力される

  ```rust
  fn main() {
      let ans = divide(10, 0);
      println!("{}", ans)
  }

  // 0 で除算すると panic が発生する
  fn divide(num: usize, divisor: usize) -> usize {
      num / divisor
  }
  ```

  ```bash
  $ RUST_BACKTRACE=1 cargo run

      Finished dev [unoptimized + debuginfo] target(s) in 0.00s
      Running `target/debug/chapter7`
  thread 'main' panicked at 'attempt to divide by zero', src/main.rs:7:5
  stack backtrace:
    0: rust_begin_unwind
              at /rustc/02072b482a8b5357f7fb5e5637444ae30e423c40/library/std/src/panicking.rs:498:5
    1: core::panicking::panic_fmt
              at /rustc/02072b482a8b5357f7fb5e5637444ae30e423c40/library/core/src/panicking.rs:107:14
    2: core::panicking::panic
              at /rustc/02072b482a8b5357f7fb5e5637444ae30e423c40/library/core/src/panicking.rs:48:5
    3: chapter7::divide
              at ./src/main.rs:7:5
    4: chapter7::main
              at ./src/main.rs:2:15
    5: core::ops::function::FnOnce::call_once
              at /rustc/02072b482a8b5357f7fb5e5637444ae30e423c40/library/core/src/ops/function.rs:227:5
  note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
  ```
  
- パニックが起きるのはプログラマの責任 (Don't panic)
- パニックは定義された動作である。ただ、起きるべきではない動作というだけ。
- パニックはスレッド単位で発生するので、他のスレッドの動作は引き続き行われる
- `std::panic::catch_unwind()` で、パニックによるスタックの巻き戻しをキャッチできる
- パニックによるスタックの巻き戻しが発生せず、プロセスをアボートするパターン
  - 1 つ目のパニック中に `.drop()` メソッドで 2 つ目のパニックが発生した場合
    - 致命的な状態として、スタックの巻き戻しを中止してプロセスを完全に停止させる
  - コンパイル時に `-C panic=abort` を指定した場合
    - 1 つ目のパニックが発生した時点でプロセスを完全に停止させる
    - プログラムはスタックの巻き戻し方法を知っておく必要がなくなる、コンパイルされたソースコードのサイズが小さくなる

## 7.2

- Rust に例外はない
- 何らかのエラーが発生する可能性がある関数は `Result` 型を返す

  ```rust
  use std::io;

  fn main() {
      let ans = match divide_safe(10, 0) {
          Ok(a) => a,
          Err(e) => {
              eprintln!("{:?}", e);
              std::process::exit(1)
          }
      };

      println!("{}", ans)
  }

  // 0 で除算しようとしても panic は発生しない
  fn divide_safe(num: usize, divisor: usize) -> Result<usize, io::Error> {
      if divisor == 0 {
          let err_msg = "cannot divide with 0 value";
          return Err(io::Error::new(io::ErrorKind::Other, err_msg));
      }

      Ok(num / divisor)
  }
  ```

- 複数種類のエラー型を返す可能性がある場合は
  - 独自のエラー型を定義する
  - 「すべてのエラー」を意味する `Box<dyn std::error:Error + Send + Sync + 'static>` に変換する
    - 長いので予め定義しておいて使うのがよさそう
      
      ```rust
      type GenericError = Box<dyn std::error:Error + Send + Sync + 'static>;
      type GenericResult = Result<T, GenericError>;
      ``` 
    
    - どの型のエラーなのかを表現できなくなる
    - 特定のエラー型にのみ対処したい場合 `error.downcast_ref::<ErrorType>()` を用いると良い
    
- エラーが起こり得ない場面でのエラー処理には `.unwrap()` を使うことが一番良さそう
  - 文字列から数値への変換処理で、文字列には数字のみしか渡らないと言える場合など
  - それでもエラーになる (= panic する) 場合はプログラマが間違っている

- main 関数でのエラーハンドリングには `.expect()` を使う方法が最も単純
  - main 関数の返り値は通常 `Result` ではないので `?` は使えない
  - ただし、 `expect()` は panic を発生させる
- main 関数が `Result` を返すように実装することも可能 (そうすれば `?` が使える)
  - ただし、エラーメッセージは規定のもので、原因がわかりづらくなる
  - 理想とは言えない
  - メッセージの詳細を出力したいのであれば、自分で適切なメッセージを考えて出力したほうがよい

- まとめ
  - 最も一般的なハンドリングはエラーを伝搬することで、 Rust ではそれを `?` だけでできるようになっている
  - `Result` 型を返すようにすることで、失敗する可能性があるかどうかの判断がしやすくなる