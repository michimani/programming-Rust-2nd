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

- Rust では守るべき対象となるデータを `Mutex` の内部に持つ
- `Mutex::new(item)`
- `Arc::new()` と合わせて `Arc::new(Mutex::new(item))` のように使うことが多い
  - Arv はスレッド間で何かを共有する際に有用
  - Mutex は複数のスレッドから共有アクセスされる可変データを保持するのに有用
- とはいえ Mutex に頼りすぎるのも良くない
  - デッドロックは防げない
    - クリティカルセクションを小さく持ち、入ったらすぐに出るようにする
  - 毒された排他ロック
    - Mutex を保持したスレッドがパニックを起こすと、それ以降 Mutex への lock はエラーを返す
    - パニックを起こしたスレッドはデータの更新を途中までしか行っていない等、データ不整合が起こっている可能性があるため、Rust は安全のためにこのような仕様になっている

### 排他ロックを用いた、複数消費者を持つチャネル

- Rust の `Receiver` は 1 つしか作れない
- ただし、 `Receiver` を `Mutex` でくるむことで実現できる
  - `Arc<Mutex<Receiver<T>>>` みたいな感じ

### RwLock\<T\>

- `RwLock::write`
  - `Mutex.lock()` に近い
  - 守られている値に対する排他的な mut アクセスを取得できるまで待つ
- `RwLock::read()`
  - 非 mut アクセスを提供する
  - 読み取りに関しては複数スレッドから同時に行っても安全、という観点から、こちらのほうが短いという利点がある

### 条件変数 (CondVar)

- `CondVar::wait(guard)` で条件が満たされるまで待つ
- `CondVar::notify_all()` または `CondVar::notify_one()` で、待っているスレッドを起こす

### アトミック変数

- `AtomicIsize`, `AtomicUsize`, `AtomicI32`, `AtomicBool` などの型
- アトミックな値については複数のスレッドが同時に読み書きしてもデータ競合が発生しない

  ```rust
  #[test]
  fn test_atomic() {
      let mut count = Arc::new(AtomicUsize::new(0));

      let mut thread_handles = vec![];
      for i in 0..10 {
          let c = count.clone();
          thread_handles.push(thread::spawn(move || {
              // fetch_add は `lock incq` 命令にコンパイルされる
              // 通常の c += 1  は ただの `incq` 命令
              // Ordering::SeqCst はメモリ順序を表わす
              // 迷ったらとりあえず SeqCst でいい
              c.fetch_add(1, Ordering::SeqCst);
          }))
      }

      for handle in thread_handles {
          handle.join().unwrap()
      }

      println!("{:?}", count);
  }
  ```

### グローバル変数

- アトミック変数を用いてグローバル変数を定義することが可能

  ```rust
  use std::sync::atomic::AtomicUsize;

  static STATIC_VALUE: AtomicUsize = AtomicUsize::new(0);
  ```