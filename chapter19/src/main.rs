#![allow(unused)]
use rayon::prelude::*;
use std::io;
use std::sync::Arc;
use std::{thread, time};

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_greeeeet() {
    let greets = vec![
        "Good morning".to_string(),
        "Hello".to_string(),
        "Hey".to_string(),
        "Goodby".to_string(),
        "おはよう".to_string(),
        "こんにちは".to_string(),
        "さようなら".to_string(),
        "おやすみ".to_string(),
    ];

    // 複数のスレッドで共有したい不変の値
    let to: Arc<String> = Arc::new("Taro".to_string());

    // spawn で並列処理
    // by_spawn(&to, greets);

    // rayon で並列処理
    by_rayon(&to, greets).unwrap();
}
fn by_spawn(to: &Arc<String>, greets: Vec<String>) {
    let mut thread_handles = vec![];
    for g in greets {
        // clone して使う
        // 完全な値のコピーではなく、スマートポインタ Arc のコピー
        let to_c = to.clone();
        // 後で join するために spawn の返り値 `JoinHandle` をベクタに詰めている
        thread_handles.push(thread::spawn(move || sleep_and_say(&to_c, &g)));
    }

    for handle in thread_handles {
        handle.join().unwrap().unwrap()
    }
}

fn by_rayon(to: &Arc<String>, greets: Vec<String>) -> io::Result<()> {
    greets
        .par_iter()
        .map(|greet| sleep_and_say(to, greet))
        .reduce_with(|r1, r2| if r1.is_err() { r1 } else { r2 })
        .unwrap()
}

fn sleep_and_say(to: &Arc<String>, greet: &str) -> io::Result<()> {
    let one_sec = time::Duration::from_secs(1);
    thread::sleep(one_sec);
    println!("{}, {}", greet, to);
    Ok(())
}

use std::sync::mpsc;

#[test]
fn test_channel() {
    let r = start_sender();
    message_printer(r)
}

/// 10 件のメッセージを 1 秒ごとに送信する
fn start_sender() -> mpsc::Receiver<String> {
    let (sender, receiver) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let one_sec = time::Duration::from_secs(1);
        for i in 0..10 {
            thread::sleep(one_sec);
            let msg = format!("message {}", i);
            println!("send '{}'", msg);
            if sender.send(msg).is_err() {
                break;
            };
        }
    });

    receiver
}

/// 3 秒ごとにメッセージを読み込んで出力する
fn message_printer(r: mpsc::Receiver<String>) {
    let five_sec = time::Duration::from_secs(3);
    for m in r {
        thread::sleep(five_sec);
        println!("{}", m)
    }
}
