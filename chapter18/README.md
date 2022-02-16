18 章 入出力
===

- 主要な 3 つの I/O トレイト

  ```
  Read
  ├── Stdin ... プロセスの標準入力ストリームからデータを取得
  ├── File ... std::fs::File::open(filename) でオープン
  ├── TcpStream ... ネットワークからデータを取得
  └── BufRead
      ├── BufReader<R>
      ├── Cursor<&[u8]>, Cursor<Vec<u8>> ... メモリ上にあるバイト配列、もしくはベクタから読み込む
      └── StdinLock

  Write
  ├── Stdout ... 端末に書き出し
  ├── Stderr
  ├── File ... std::fs::File::create(filename) でオープン
  ├── TcpStream ... ネットワーク経由でデータを送信
  ├── Vec<u8> ... write メソッドでベクタへ書き込み
  └── BufWriter<W>
  ```
  
- `std::io` モジュールのインポート
  - `use std::io::{self, Read, Write, ErrKind};`
  - `self` を入れることで `io::Result` とか `io::Error` と短縮した形で書ける

### Reader

- データを読み出すメソッドがいくつかあるが、それらは reader 自身を mut 参照として使う
- `reader.read(&mut buf)`
  - `buf.len()` を最大値として、バイト列を読み出して `buf` に格納する
  - 返り値は `io::Result<u64, io::Error>` で、 `u64` は読み込んだバイト数を表わす
  - `io::Error` には `kind()` メソッドがあり、エラーの種類が取得できる
    - PermissionDenied , ConnectionReset など
    - Interrupted に対しては、特に理由がなければ read をやり直す
  - このメソッドは OS と近い低レイヤにあるため、Rust は高レイヤのメソッドをいくつか用意している
- Read トレイトの主要なメソッド
  - `reader.read_to_end(&mut byte_vec)`
    - すべてのデータを読み出し、 `Vec<u8>` の `byte_vec` に追加する
    - 読み出すデータ量に制限がないため、注意が必要
    - `take()` メソッドで制限できる
  - `reader.read_to_string(&mut string)`
    - 読み込んだデータを引数の String の後ろにつなげる
    - 読み込んだデータが UTF-8 で解釈できない場合、 `ErrorKind::InvalidData` エラーとなる
  - `reader.read_exact(&mut buf)`
    - 与えられたバッファをちょうど埋めるだけのデータを読み出す
    - `buf.len() > 読み出せるデータ量` の場合 `ErrorKind::UnexpectedEof` エラーとなる
- Read トレイトの主張なアダプタメソッド
  - `reader.bytes()`
    - 入力ストリーム上のバイト列に対するイテレータを返す
    - 値を生成するたびにバイトのエラーチェックが必要
    - 値を生成するために `read()` メソッドを呼ぶため、 reader がバッファリングされていないとシステムコールを何度も実行することになり非常に非効率
  - `reader.chain(reader2)`
    - reader からの入力に続いて reader2 の入力を取り出す、新たな reader を返す
  - `reader.take(n)`
    - `n` バイトしか読み出さない新たな reader を返す
- **reader をクローズするようなメソッドはない**

### バッファ付き reader

- `reader.read_line(&mut line)`
  - 1行読み、 String 型の `line` の末尾に付け加える (行末の `\n` および `\r\n` も含まれる)
  - 返り値は `Result<u64, io::Error>` で、 `Ok(0)` となれば終了
- `reader.lines()`
  - 入力を改行ごとに区切った値を生成するイテレータを返す
  - 各値には改行文字は含まれない
  - テキストを読み出すほとんどの場合でこのメソッドを使う
- `reader.read_until(stop_byte, &mut byte_vec)`, `reader.split(stop_byte)`
  - `read_line`, `lines` と動作は同じだが、文字列単位ではなくバイト列単位
  - 区切りは改行ではなく `stop_byte` で指定したバイト列

### Writer

- 入力にはメソッドを使っていたが、出力にはマクロが存在する
  - print 系と write 系の違いは下記
    - 引数に writer を取るかどうか (print 系はとらない)
    - 引数を返すかどうか (print 系は返さない。write 系は `Result` を返す)
- 主なメソッド
  - `writer.write(&buf)`
    - buf 内のバイト列の一部を writer がもつストリームに出力する
    - *一部* となっているのはストリームの都合によるから
    - 返り値の `Result<usize>` で実際に出力されたバイト数がわかる
  - `writer.write_all(&buf)`
    - buf 内のバイト列をすべて出力する
    - 返り値は単に `Result<()>`
  - `writer.flush()`
    - バッファされたデータをストリームに書き出す
    - `println!` と `eprintln!` は自動的にフラッシュする
    - `print!` と `eprint!` は自動的にフラッシュしない。やりたければ明示的に `.flush()` を呼ぶ
- reader と同様に **writer をクローズするようなメソッドはない**
- バッファ付きの writer は `BufWriter::new(writer)` および `BufWriter::with_capacity(size, writer)` で生成できる

### ファイル

- `File::open(fname)`, `File::create(fname)` でオープンする
- `io` ではなく `fs` モジュール
- オープンしたあとは reader/writer と同様の動きをする
- `OpenOption::new()` でオープン方法を柔軟に設定する
  - [OpenOptions in std::fs - Rust](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html)
  - メソッドを連鎖させる書き方を ビルダ と呼ぶ
- `seek` メソッドによってファイル内を移動できるが、遅い。時間がかかる。

### その他の reader, writer

- reader 型
  - `stdin()`
  - `empty()`
    - 何もしない reader
    - `read()` メソッドは `Ok(0)` を返すだけ
  - `repeat(byte)`
    - 指定した byte を無限に繰り返す reader
- writer 型
  - `stdout()`, `stderr()`
  - `Vec<u8>`
    - 文字列を生成するには `String::from_u8(vec)` とする
  - `io::sink()`
    - 何もしない writer
    - `write()` メソッドは `Ok(())` を返すだけ
- 両方
  - `Cursor::new(buf)`
    - buf から読み出す、バッファ付き reader
    - String から読み出す reader を作る場合にはこれ
    - buf の型が `&mut [u8]` または `Vec<u8>` であれば Write も実装する
  - `std::net::TcpStream`
  - `std::process::Command`
    - 子プロセスを起動

### バイナリデータ、圧縮、シリアライズ

- `std::io` に対して機能を追加する OSS クレートがいくつか存在する
  - `byteorder`
    - reader と writer にバイナリデータの入出力機能を追加
    - [byteorder - crates.io: Rust Package Registry](https://crates.io/crates/byteorder)
  - `flate2`
    - gzip 圧縮されたデータの読み書きを行うアダプタメソッドを提供する
    - [flate2 - crates.io: Rust Package Registry](https://crates.io/crates/flate2)
  - `serde`, `serde_json`
    - Rust の構造体とバイト列との変換をサポートする
    - [serde - crates.io: Rust Package Registry](https://crates.io/crates/serde)
    - [serde_json - crates.io: Rust Package Registry](https://crates.io/crates/serde_json)

## ファイルとディレクトリ

- OS はファイル名が有効な Unicode であることを強制しない
- null 文字とスラッシュ以外を有効なバイト列として扱う
- Rust でもそのようなバイト列を扱うために `std::ffi::OsStr` と `OsString` がある
- `OsStr` は UTF-8 のスーパーセット
- つまり Rust にはふたつの文字列型が存在する
  - Unicode 文字列を表わす `str` 型
  - OS が扱うバイト列にも対応した `OsStr` 型
- `std::path::Path` は `OsStr` とほぼ同じだが、パスを扱う上での便利メソッドが実装されている

### Path と PathBuf

- `Path::new(str) -> &Path`
  - `&str`, `&OsStr` を `&Path` に変換する
- `path.parent() -> Option<&Path>`
  - path の親ディレクトリがあればそれを返す
- `path.file_name() -> Option<&OsStr>`
  - path の最後の構成要素があればそれを返す
- `path.is_absolute()`, `path.is_relative()`
  - 絶対パスか、相対パスかを返す

  ```rust
  #[test]
  fn test_osstr_path() {
      use std::ffi::OsStr;
      use std::path::Path;

      // 相対パスか、絶対パスか
      assert!(Path::new("hoge1/hoge2").is_relative());
      assert!(Path::new("/hoge0/hoge1/hoge2").is_absolute());

      let p = Path::new("/Users/hoge/dir/filename");
      // 親ディレクトリ
      assert_eq!(p.parent(), Some(Path::new("/Users/hoge/dir")));

      // 親ディレクトリの親ディレクトリ
      assert_eq!(p.parent().unwrap().parent(), Some(Path::new("/Users/hoge")));

      // ファイル名
      assert_eq!(p.file_name(), Some(OsStr::new("filename")));

      // ルートまで遡るイテレータ
      assert_eq!(
          p.ancestors().collect::<Vec<_>>(),
          vec![
              Path::new("/Users/hoge/dir/filename"),
              Path::new("/Users/hoge/dir"),
              Path::new("/Users/hoge"),
              Path::new("/Users"),
              Path::new("/"),
          ]
      );

      // パスの結合
      assert_eq!(
          Path::new("hoge/dir").join(Path::new("file")),
          Path::new("hoge/dir/file")
      );
      // path2 が絶対パスの場合、返り値は path2 のコピーとなる
      assert_eq!(
          Path::new("hoge/dir").join(Path::new("/Users")),
          Path::new("/Users")
      )
  }
  ```

### ファイルシステム、ディレクトリの読み出し

```rust
#[test]
fn test_file_system() {
    use std::ffi::OsStr;
    use std::fs;
    use std::path::Path;

    let tmpdir = Path::new("test/tmp_for_test");

    match fs::create_dir_all(tmpdir) {
        Err(e) => panic!("{}", e),
        Ok(_) => println!("create test directory"),
        _ => panic!("undefined error"),
    }

    let metadata = match fs::metadata(tmpdir) {
        Err(e) => panic!("{:?}", e),
        Ok(m) => m,
        _ => panic!("undefined error"),
    };

    assert!(metadata.is_dir());

    match fs::remove_dir(tmpdir) {
        Err(e) => panic!("{}", e),
        Ok(_) => println!("remove test directory"),
        _ => panic!("undefined error"),
    }

    let testdir = Path::new("test");

    let mut diriter = match testdir.read_dir() {
        Err(e) => panic!("{}", e),
        Ok(i) => i,
        _ => panic!("undefined error"),
    };

    let mut c = 0;
    for d in diriter {
        let dir = match d {
            Err(e) => panic!("{}", e),
            Ok(e) => e,
            _ => panic!("undefined error"),
        };

        assert_eq!(
            dir.file_name().to_string_lossy()[0..10],
            "test_file_".to_string()
        );

        c += 1;
    }
    assert_eq!(c, 5);
}
```