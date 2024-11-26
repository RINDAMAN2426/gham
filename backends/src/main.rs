use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:2426").await.unwrap();
    println!("Running on ws://127.0.0.1:2426");

    // TODO: 소켓 통신 구현
}

