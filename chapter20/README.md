20 章 非同期プログラミング
===

- 非同期タスクはスレッドに似ている
  - 生成に時間がかからない
  - メモリのオーバーヘッドは桁違いに小さい
- 最小限の言語機能
  - フューチャ
  - 非同期関数
  - await 式
  - タスク
  - `block_on`, `spawn_local`

### フューチャ

- `std::future::Future` トレイト
  - [The Future Trait - Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/02_execution/02_future.html)

    ```rust
    trait SimpleFuture {
        type Output;
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    }

    enum Poll<T> {
        Ready(T),
        Pending,
    }
    ```

- `poll` メソッドは、処理が終わっていれば `Ready(T)`, 終わっていなければ `Pending` を返す
- 同期版と非同期版の関数のシグネチャはほとんど同じ
- 返り値が `Future` でラップされたものになっている
- 呼び出し元では、返り値に対して `Ready(output)` が得られるまで `poll` メソッドを呼ぶだけ
- `Future` が `Ready(output)` を返したあとは、 `poll` は呼ばれないものと仮定しても良い
  - `Ready(output)` を返したあとに `Pending` を返し続けるものもあればパニックやハングを起こすものもある
  - `fuse` メソッドによって `Pending` を返し続けるようにすることもできる
  - ただし、基本的には `fuse` を使わずじ `Future` の実装を尊重する

### async 関数と await 式

- [async-std - crates.io: Rust Package Registry](https://crates.io/crates/async-std)
- 非同期版のメソッド (`Future` を返す呼び出し) のあとに `await` をつける
  - `Future` が `Ready(output)` を返すまで待機する
  - これは構造体のフィールドを参照しているのではない
- 非同期関数 (`async fn`) の返り値は `T` のままであっても、コンパイル時に `T` の `Future` として扱ってくれる
- 非同期関数内では、ポーリングのたびに `await` をたどっていく
- 途中の `await` で `Ready(output)` が返されない限り、先には進めない

### 非同期関数を同期関数から呼び出す: block_on

- 非同期関数 (`async fn`) の **中** で `Future` の値を取得するには `await` するだけでよい
- 同期関数 (例えば `main`) の中で非同期関数を呼んだ場合、その関数から返された `Future` に対してポーリングを行うには `task::block_on` 関数を使う
- `block_on` は同期関数内でのみ使う。非同期関数の中では使ってはいけない
- ポーリングのループを手で書くのは難しくはないが、 `block_on` を使う上で嬉しいポイントは、ポーリングすべき状態になるまでスリープしてくれるところ
  - 無駄なポーリングをしてリソースを消費するようなことがない
  
### 非同期タスクの起動

- `task::block_on` で `Future` が `Ready` を返すまでブロックして勝手にポーリングしてくれるのはわかったが、それでは普通に同期実行しているのと変わらない
- 非同期タスクを待っている間、他のことをやりたい というのが本来の目的
- これを実現するためには `async_std::task::spawn_local` 関数を使う
  - [spawn_local in async_std::task - Rust](https://docs.rs/async-std/1.10.0/async_std/task/fn.spawn_local.html)
  - 引数として `Future` を受け取り、プールに保持する
  - `v1.10.0` 時点では `unstable` なので `Config.toml` で明示的に指定する必要がある
  - 使い方は `std::thread::spawn` に似ている
- `spawn_local` に詰め込んだ処理は非同期実行されるように見えるが、実際には詰め込んだ `Future` に対して同期的にポーリングしているだけ
- `Future` が `Poll::Pending` をすぐに返せば問題ないが、関数の中で時間がかかる処理を実行していた場合

### 非同期ブロック

- ブロックも非同期化できる
- 通常のブロックと異なり、最後の式の `Future` を返す
- 非同期ブロック内で `?` を使った場合、ブロックから return するのみで外側の関数から return するわけではない
- クロージャと同様に `async move` とすればキャプチャした値を参照ではなく所有権を保持することができる
- 非同期ブロックには返り値の型を指定できない
  - 将来的に指定可能になる可能性はある
  - 現状は `Ok(())` を返すことで回避できる
  
### 非同期タスクをスレッドプールで実行

- `async_std::task::spawn` を使ってスレッドプールで `Future` を実行できる
- `async_std::task::spawn_local` と同じように使える
  - 実際、 `async_std::task::spawn` のほうが使われている
    - 計算機資源をバランス良く使える
  - 
- 注意点
  - ある非同期タスクがあるスレッドで実行され await でブロックされたあと、別のスレッドで再開される可能性がある
  - スレッドローカルなストレージで実行している (`spawn`)の場合、 await する前に持っていたデータが await 後に別物になっている可能性がある (別スレッドからポーリングされた結果)
    - await されるたびに別のスレッドで実行される可能性があるということ
    - `Future` は別スレッドで実行されるので `Send` マーカトレイとを実装している必要がある (→ 19章)
  - これが困る場合、タスクローカルストレージを使うとよい
    - [task_local in async_std - Rust](https://docs.rs/async-std/1.10.0/async_std/macro.task_local.html)

### 長時間の計算: yield_now と spawn_blocking

- `Future` は可能な限り poll メソッドからリターンするべき
  - 他の非同期タスクを待たせることになる
- 回避策
  - 単純に、時々 await を呼び出す
    - `async_std::task::yield_now().await`
    - 初回は `Poll::Pending` を返すが、以降は次の await を辿る
    - 自分で実装する非同期関数なら容易だが、クレートなどを改変するのは大変
  - 専用のスレッドで実行する
    - `async_std::task::spawn_blocking()`
    - クロージャを引数として受け取る
    - パスワードハッシュ化のためのクレート
      - [argonautica - crates.io: Rust Package Registry](https://crates.io/crates/argonautica/0.2.0)
      - 1秒近くかかる
      
### 非同期機構の設計

- 他の言語 (JavaScript の Promise, C# のタスク) も同様に await 式を用いるが、それらは非同期呼び出しと同時に実行されう
- Rust は `block_on`, `spawn`, `spawn_local` などの **エグゼキュータ (executor)** と呼ばれる関数に渡すことで始めて実行される
- エグゼキュータには `tokio` と呼ばれるものがあったり、独自に実装することもできる

### 具体的なアプリケーションの実装 (チャットアプリ)

- [ProgrammingRust/async-chat: Example code from Chapter 19, Asynchronous Programming: an asynchronous chat client and server](https://github.com/ProgrammingRust/async-chat)
- 大きいクレートを使用する場合、依存を小さくするために `feature` で必要なコンポーネントだけを取り込む
  - `feature = ["hogehoge"]`
- アプリケーションで使う汎用的なエラーについては、 `anyhow` クレートを使うと良い
  - [anyhow - crates.io: Rust Package Registry](https://crates.io/crates/anyhow)
- 非同期関数内での `lines` は興味深い
  - イテレータを生成するのではなく、`Result<T>` のストリームを返す
  - ストリームは、イテレータとフューチャのハイブリッドのようなもの
  - ストリームのもつ `poll_next` を直接呼ぶのではなく、 `next` を呼んで返されたフューチャに対して `await` すればよい
  - ストリームを使う際には、 `async_std::prelude::*` を忘れないようにする
  - 終了したストリームへのポーリングの挙動は定義されていない
     - `fuse` メソッドで挙動が予測できるようになる (イテレータやフューチャと同様)

## フューチャとエグゼキュータ

- `block_on` のようなエグゼキュータはフューチャにポーリングする際に **ウェイカ** と呼ばれるコールバックを渡す
- フューチャの準備ができていなければ `Poll::Pending` が返され、再度ポーリングする意味がある状態になったらウェイカが実行される
- エグゼキュータとウェイカがが交互に poll と wake を実行するのが理想

## ピン留め

- ポインタに対する「承認の封印」 (とは)
- フューチャにはふたつのライフステージがあり、場合によっては Rust の借用チェッカをすり抜けるメモリ安全性の侵害が発生する可能性がある
  - 第1ステージ
    - フューチャが作成された直後
    - 関数のボディ部は未実行で、フューチャが保持している値が借用されることはない
    - この時点では他の部分と同様に安全に値の移動が可能
  - 第2ステージ
    - フューチャが始めてポーリングされて以降
    - 関数の一部が実行され、その際に保持した値の変数への参照をもったまま await している状態
    - この状態での値の移動は安全ではないと仮定するべき
- `Pin` は `&mut Self` ポインタ型へのラッパ
- ポインタの使われ方を制御し、ポインタが今後移動しないことを保証する
- `Pin` でラップしたポインタを作らない限りポーリングすることはできない
- 通常の非同期コードではピン留めの心配をする必要はない
  - await するかエグゼキュータに関数を渡すかで、内部的にピン留めされているため

## 非同期コードはどのような場面で使うべきか

- 非同期タスクはメモリ使用量が少ない
  - より多くの同時接続を処理できそう
- 非同期タスクは生成が高速
  - スレッドの生成と比較して、 1/15 程度の時間で生成できる
- コンテクストスイッチが高速
  - 多数の独立したタスクが相互に通信する際には恩恵を得られそう
- マルチスレッドプログラムに対しては解析ツールがあるが、現時点で Rust の非同期タスクには解析ツールがない
  - サーバのチューニングはしづらい
