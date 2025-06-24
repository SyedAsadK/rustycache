use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    spawn,
};

use crate::{command::cmd_parser, database::db::Database};

pub async fn create_server(addr: &str) {
    let listener = TcpListener::bind(addr)
        .await
        .expect("Error creating listener");
    let db = Database::new();
    loop {
        let (socket, _) = listener.accept().await.expect("Error creating socket");
        let db = db.clone();
        spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut buffer = String::new();
            while reader
                .read_line(&mut buffer)
                .await
                .expect("Error reading buffer")
                > 0
            {
                let response = cmd_parser(&db, buffer.trim())
                    .await
                    .unwrap_or_else(|e| format!("-ERR{}\r\n", e));

                writer.write_all(response.as_bytes()).await.unwrap();
                buffer.clear();
            }
        });
    }
}
