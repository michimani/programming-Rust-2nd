#![allow(unused)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;

/// 入出力のサンプル
/// cargo run COMMAND
///     COMMAND: unicode, grep
fn main() {
    static UNI: &str = "unicode";
    static GREP: &str = "grep";

    let mut args = std::env::args().skip(1);
    match args.next() {
        // command
        Some(c) => {
            if c == UNI {
                // 標準入力から受け取った文字列の
                // 各文字の unicode 表現とともに出力する
                let res = unicode();
                if let Err(err) = res {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            } else if c == GREP {
                // grep のサンプル
                let result = grep_main();
                if let Err(err) = result {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            } else {
                eprintln!("invalid command");
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("command is required");
            std::process::exit(1);
        }
    }
}

fn unicode() -> io::Result<()> {
    let si = io::stdin();
    for l in si.lock().lines() {
        let line = l?;
        for c in line.chars() {
            println!("{}: \\u{:x}", &c, c as u32);
        }
    }

    Ok(())
}

fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
    R: BufRead,
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn grep_main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(2);
    let target = match args.next() {
        Some(s) => s,
        None => Err("usage: grep PATTERN FILE ...")?,
    };
    let files: Vec<PathBuf> = args.map(PathBuf::from).collect();

    if files.is_empty() {
        // 第2引数以降が省略されていた場合、標準入力を受け付ける
        let stdin = io::stdin();
        grep(&target, stdin.lock())?;
    } else {
        for file in files {
            println!("in {:?} ...", file.file_name());
            let f = File::open(file)?;
            grep(&target, BufReader::new(f))?;
        }
    }

    Ok(())
}

#[test]
fn test_process_command() {
    use std::process::{Command, Stdio};

    let mut my_word = vec!["hello", "apple", "children"];

    let mut child = Command::new("grep")
        .arg("-e")
        .arg("o")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    let mut to_child = child.stdin.take().unwrap();
    for w in my_word {
        writeln!(to_child, "{}", w).unwrap();
    }
    drop(to_child);
    child.wait().unwrap();
}

#[test]
fn test_repeat() {
    let mut buf = [0u8; 10];
    let mut rep_reader = io::repeat(9);

    rep_reader.read_exact(&mut buf);
    assert_eq!(buf, [9, 9, 9, 9, 9, 9, 9, 9, 9, 9])
}

#[test]
fn test_serialize() {
    use std::collections::HashMap;

    type RoomId = String;
    type RoomExists = Vec<(char, RoomId)>;
    type RoomMap = HashMap<RoomId, RoomExists>;

    let mut map = RoomMap::new();
    map.insert("Room 1".to_string(), vec![('A', "one".to_string())]);
    map.insert(
        "Room 2".to_string(),
        vec![('A', "two".to_string()), ('B', "two".to_string())],
    );

    match serde_json::to_writer(&mut std::io::stdout(), &map) {
        Err(e) => {
            eprintln!("{}", e);
        }
        _ => {}
    }
}

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
