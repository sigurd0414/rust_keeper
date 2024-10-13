use tokio::net::TcpStream;
use tokio::prelude::*;

pub async fn run_client() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("成功連接到服務器");

    let message = b"Hello Server!";
    stream.write_all(message).await.unwrap();
    println!("已發送消息：{}", String::from_utf8_lossy(message));
}
