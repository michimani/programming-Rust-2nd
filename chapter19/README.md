19 章 並列性
===

- Rust での並列処理におけるスレッドの使い方
  - フォーク・ジョイン並列
  - チャネル
  - 可変状態の共有



## フォーク・ジョイン並列

- 非常に単純。実装もシンプル
- ボトルネックがない。待ちが発生するのはフォークされたスレッドが最後にジョインされる部分のみ
- 独立したユニットに分割できる場合のみ使える
- `std::thread::spawn()`: 新しいスレッドを作成する
- main 関数は、中で走っているスレッドがあっても (何もしなければ) それらの終了を待たずに終了する
  - main を終了するまでにフォークしたすべてのスレッドを join する
- `handle.join()` は Result を返すので、適切にエラーハンドリングをすれば各スレッド内でのパニックはそのスレッド内にのみ影響する
  - `unwrap()` で暗黙的に処理したりすると親スレッドにパニックを伝搬することになる
- **不変の** 共通の値を複数のスレッドで共有するためには `Arc<T>` 型としてスレッドに渡す
- `spawn` もいいが、もうちょっといい感じにスレッド管理してくれる **Rayon** というクレートがある
  - [rayon - crates.io: Rust Package Registry](https://crates.io/crates/rayon)

## チャネル

- あるスレッドから別のスレッドへ値を送信する一方通行のパイプ
- スレッド安全なキュー
- `sender.send(item)` で送信
- `receiver.recv()` で値を1つ取り除く
  - チャネルがからの場合、値が送信されるまで `recv()` はブロックする
- チャネルは `std::sync::mpsc` モジュールの一部
  - mpsc => multi-producer, single-consumer
  - 送信者を複数持つことが可能
  - `Sender<T>` は Clone を実装しているので、必要なだけクローンして使うことができる
  - 一方、 `Receiver<T>` は Clone できないので、複数のスレッドで共有して使いたい場合は `Mutex` を使う必要がある
- `mpsc::channel()` で `(sender, receiver)` を生成
- `mpsc::sync_channel(n)` でチャネルが保持できる値の数を指定できる (同期チャネル)
  - チャネルがいっぱいのときは `send()` がブロックする
- sender, receiver それぞれの型は `mpsc::channel::<String>()` という形で指定できる
- `send()` も `recv()` も値自体のコピーは行わず、移動のみ
- それぞれ `Result` を返すが、失敗するのは相手がドロップされている場合のみ
- 接続をクローズするためにチャネルの一端をドロップするというのはよくやる
- receiver が値を待つ処理は、下記の二通りの書き方ができる (どちらも挙動は同じ)

  ```rust
  while let Ok(item) = receiver.recv() {
    do_something_with(item)
  }
  ```

  ```rust
  for item in receiver {
    do_something_with(item)
  }
  ```

- `sender1.send(sender2)` として、 receiver 側で `sender2.send()` すれば双方向に値をやりとりできる


## 可変状態の共有




