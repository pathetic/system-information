// src/main.rs
mod collectors;
mod display;

use collectors::*;

#[tokio::main]
async fn main() {
    let client_info = ClientInfo::gather().await;
    client_info.display();
}