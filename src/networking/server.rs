use tokio::net::TcpListener;
use tokio::prelude::*;

pub async fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("服務器運行於 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return, // 客戶端斷開連接
                    Ok(n) => {
                        println!("接收到數據：{:?}", &buf[..n]);
                        // 可以在這裡實現遊戲邏輯同步，向其他客戶端廣播
                    }
                    Err(e) => {
                        eprintln!("讀取錯誤：{}", e);
                        return;
                    }
                }
            }
        });
    }
}
