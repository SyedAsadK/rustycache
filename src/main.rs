use server::create_server;

mod command;
mod database;
mod server;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:7777";
    eprintln!("Server will start at {:?}", addr);
    // here the server starts
    create_server(addr).await;
}
