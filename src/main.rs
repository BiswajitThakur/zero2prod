//! main.rs

use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    println!("Server running: 127.0.0.1:{port}");
    zero2prod::run(listener)
        .expect("Failed to bind address")
        .await
}
