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
            println!("{}: 0x{:x}", &c, c as u32);
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
