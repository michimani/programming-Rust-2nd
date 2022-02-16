17 章 文字列とテキスト
===

## Unicode

- `0` から `0x7f` を **ISO/IEC 8859-1** 文字セットに割り当てており、これを **Latin-1 コードブロック** と呼んでいる (ので、以後そう呼ぶ)
- Latin-1 は ASCII のスーパーセット、 Unicode は Latin-1 のスーパーセット
  - ASCII ⊂ Latin-1 ⊂ Unicode
- Latin-1 → Unicode の変換は普通にできる (変換テーブル不要)
  - 逆は、Unicode のコードポイントが Latin-1 の範囲に入っていれば普通にできる
- UTF-8
  - １つの文字を 1~4 バイトの列にエンコードする
  - 下記の範囲はエンコードしてはいけない (文字以外を対象として予約されている、またはそもそも Unicode の範囲ではない)
    - `0xd800` ~ `0xdfff`
    - `0x10ffff` ~
  - `0` ~ `0x7f` までのコードポイントは、そのままバイトとなる
    - つまり、ASCII テキストのバイト列はそのまま有効な UTF-8 の文字列となる (逆も然り)
    
## 文字 (char)

- Unicode のコードポイントを保持する 32 ビットの値
- `0` から `0xd7ff` および `0xe000` ~ `0x10ffff` の範囲
- `slice.chars()` で char を生成するイテレータを返す
- `is_numeric`, `is_alphabetic`, `is_digit`, `is_uppercase` ...
- `to_uppercase()`, `to_lowercase()` は それぞれ値を char を生成するイテレータを返す
  - Unicode では大文字 - 小文字 の文字数が 1:1 とは限らない

## String と str

- サイズ可変なバッファが必要か、もしくは既にあるテキストをそのまま使えるか によって `str` と `String` のそれぞれにメソッドが定義されている
- `String` は `Vec<u8>`
- `String` は `&str` に参照解決できる
- `String` へのテキスト追加には `write!` および `write_ln!` マクロが使える
  - これらは `Result<String>` を返す
  - ただし String への書き出しは失敗することはないので `.unwrap()` で握りつぶしてよい
- `string.insert`, `string.insert_str`
  - 指定したバイトオフセット `i` に文字列を挿入する
  - `i` 以降をずらしてスペースを確保する必要があるため、実行には文字の長さの 二乗の時間がかかる
- `clear`, `truncate(n)`, `pop()`, `remove(n)`, `drain(range)`, `replace_range(range, rep)`
  - `remove(n)` は `n` 以降の文字をずらす必要があるため、`n` 以降の文字数に比例した時間がかかる
  - `replace_range` は、置き換えるスライスが指定した範囲と同じ長さである必要はない
    - ただし、差異がある場合には指定した範囲以降の文字の移動が発生する (少なくても多くても)
- `split` 系のメソッドもあるが、ほぼスライスのときと同様。違いは引数に pattern を渡すくらい
- 文字列からのパース
  - `std::str::FromStr` を実装している型なら、文字列からパースする方法が用意されている
  - `from_str` は `Result<T>` を返す
- 文字列へのパース
  - `std::fmt::Display` を実装している型について、 `format` マクロを使う
  - `std::fmt::Display` を実装している型は `ToString` を実装しているので `to_string()` メソッドを使う
    - `ToString` は `Display` よりも柔軟性に欠けるので、独自の型に実装する場合は `ToString` よりも `Display` を実装したほうがよい
  - `std::fmt::Debug` を実装している型について `format!({:?}, T)` を使う
- バイト表現としての取得
  - `as_bytes()`: 参照のみ
  - `into_bytes()`: 所有権も取得
- バイト表現から文字列へ
  - `str::from_utf8`: UTF-8 として解釈できない部分がある場合は `Err(e)` を返す
  - `String::from_utf8`: UTF-8 として解釈できない部分がある場合は `Err(e)` を返す
  - `String::from_utf8_lossy`: 必ず成功する。UTF-8 として解釈できない部分は 「�」で置き換えれる
  
## フォーマット

- `format!`: `String` を生成
- `print!`, `println!`: 標準出力
- `write!`, `writeln!`: 出力ストリーム
- `panic!`: 要因とともにパニックを起こす
- `{:p}` でポインタの値を確認できる
- ユーザ定義型のフォーマット出力
  - `fmt::Display` を実装する
  - `fmt` メソッドを定義する
  
## 正規表現

- 標準 (`std`) ライブラリには含まれていないが、 `std` を管理しているチームがクレート `regex` を公開している (実質公式ライブラリ)
- [regex - crates.io: Rust Package Registry](https://crates.io/crates/regex)
- `Regex::new()` の実行にはコストがかかる
  - 毎回 Regex を構築するのではなく、予め構築したものを再利用するべき
  - `lazy_static` を使う
  - [lazy_static - crates.io: Rust Package Registry](https://crates.io/crates/lazy_static)