#![allow(unused)]
use async_std::io::prelude::*;
use async_std::net;
use std::io::prelude::*;
use surf;

/// 20.1.11
fn main() {
    let requests = &[
        "https://michimani.net".to_string(),
        "https://michimani.net/about".to_string(),
        "https://michimani.net/projects".to_string(),
    ];

    let results = async_std::task::block_on(many_requests(requests));
    for r in results {
        match r {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{}", e),
        }
    }
}

pub async fn many_requests(urls: &[String]) -> Vec<Result<String, surf::Error>> {
    let client = surf::Client::new();

    let mut handles = vec![];
    for url in urls {
        let request = client.get(&url).recv_string();
        handles.push(async_std::task::spawn(request));
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await)
    }

    results
}

// fn main() -> std::io::Result<()> {
//     use async_std::task;

//     let requests = vec![
//         ("michimani.net".to_string(), 80, "/".to_string()),
//         ("michimani.net".to_string(), 80, "/about".to_string()),
//         ("michimani.net".to_string(), 80, "/projects".to_string()),
//     ];

//     let results = async_std::task::block_on(many_requests_tmp(requests));

//     for r in results {
//         match r {
//             Ok(res) => println!("{}", res),
//             Err(e) => eprintln!("{}", e),
//         }
//     }

//     Ok(())
// }

async fn cheapo_request(host: &str, port: u16, path: &str) -> std::io::Result<String> {
    // 初回のポーリング時にはここで止まる
    // ここの Future が Ready(output) を返すまで、先には進めない
    let mut socket = net::TcpStream::connect((host, port)).await?;

    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
    // ポーリングされるたびに await をたどっていく
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write);

    let mut response = String::new();
    socket.read_to_string(&mut response).await?;

    Ok(response)
}

async fn cheapo_owning_request(host: String, port: u16, path: String) -> std::io::Result<String> {
    cheapo_request(&host, port, &path).await
}

pub async fn many_requests_tmp(
    requests: Vec<(String, u16, String)>,
) -> Vec<std::io::Result<String>> {
    use async_std::task;

    let mut handlers = vec![];
    for (host, port, path) in requests {
        handlers.push(task::spawn_local(cheapo_owning_request(host, port, path)));
    }

    let mut results = vec![];
    for h in handlers {
        results.push(h.await);
    }

    results
}

async fn verify_password(p: &str, h: &str, k: &str) -> Result<bool, argonautica::Error> {
    let password = p.to_string();
    let hash = h.to_string();
    let key = k.to_string();

    async_std::task::spawn_blocking(move || {
        argonautica::Verifier::default()
            .with_hash(hash)
            .with_password(password)
            .with_secret_key(key)
            .verify()
    })
    .await
}
