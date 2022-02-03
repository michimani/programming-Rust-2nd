14 章 クロージャ
===

- 使い所
  - Iterator のメソッド、 map や filter などでデータ列を処理する
  - thread::spawn などのスレッド API で新しいスレッドを開始する

## move

```rust
use std::thread;
fn start_sorting_thread(mut users: Vec<User>, stat: Statistic) -> thread::JoinHandle<Vec<User>> {
    // let key_fn = |user: &User| -> i64 { -user.get_statistic(stat) };
    // ↑これは、クロージャが start_sorting_thread よりも長く存在する可能性があるのに
    // その変数である stat を借用しているのでコンパイルできない
    let key_fn = move |user: &User| -> i64 { -user.get_statistic(stat) };

    // thread::spawn(|| {
    // これも同じ理由でコンパイルできない
    thread::spawn(move || {
        users.sort_by_key(key_fn);
        users
    })
}
```

- `move` をつけると、クロージャが所有権を参照するのではなく盗む
- 上の場合、１つ目のクロージャが `stat` と所有権を盗んで、２つ目のクロージャが `key_fn` と `users` の所有権を盗んでいる

## 関数型とクロージャ型

- 関数もクロージャも型がある
- 関数とクロージャの型は同じではない
  - クロージャは呼び出し可能だが `fn` ではない
  - クロージャはそれぞれ独自の型を持つ (値を借用したり盗んだりしたり...)
  - `Fn` トレイトは実装されているので、関数もしくはクロージャを受け取る関数をジェネリック関数として、 `where` で制約を課せばよい
  
    ```rust
    /// 特定の条件に該当するユーザの数を返す関数
    /// これは test_fn に関数しか受け取れない
    fn count_specific_user(users: &Vec<User>, test_fn: fn(u: &User) -> bool) -> usize {
        let mut count = 0;
        for user in users {
            if test_fn(&user) {
                count += 1;
            }
        }

        count
    }

    /// こうすれば test_fn に関数とクロージャの両方を受け取れる
    fn count_specific_user<F>(users: &Vec<User>, test_fn: F) -> usize 
      where F: Fn(u: &User) -> bool
    {
        let mut count = 0;
        for user in users {
            if test_fn(&user) {
                count += 1;
            }
        }

        count
    }
    ```
  
## クロージャの性能

- Rust のクロージャは高速
- Box や Vec に入れない限りはヒープ上に確保されることはない
- クロージャの型をコンパイラが知ることができればインライン化できる (実際多くがそうなっている)
- 値を借用するクロージャの場合、クロージャは値へのポインタのみを持つ
- `move` を付与したクロージャの場合、クロージャは値自体を持つ
- 外部の値を使用しないクロージャは、メモリを消費しない

## クロージャの安全性

- 特定の値をドロップするようなクロージャは複数回呼び出せない
  - コンパイルエラーになる
- 値をドロップするようなクロージャを関数に渡し、その関数内で複数回実行しようとしてもコンパイルエラーになる
  - 値をドロップするようなクロージャは `Fn` トレイトが実装されていないから
    - クロージャはそれぞれ型をもっている
  - このようなクロージャは、一度きりの実行を許された `FnOnce` トレイトを実装している
  - この辺をコンパイラが理解してくれるのすごい
- 何らかの値に mut でアクセスするが、ドロップはしないクロージャは `FnMut` を実装している
- `Fn` ∈ `FnMut` ∈ `FnOnce` (イメージ)
- クロージャがキャプチャしたものがすべて Copy ならクロージャも Copy  (Clone についても同じ)
