use std::fmt::format;

use crate::database::db::Database;

pub async fn cmd_parser(db: &Database, cmd: &str) -> Result<String, String> {
    let cmd_splitted: Vec<&str> = cmd.split_whitespace().collect();
    match cmd_splitted.as_slice() {
        ["SET", key, val, "EXP", ttl] => {
            let ttl = ttl.parse::<u64>().expect("Invalid time");
            db.set(key.to_string(), val.to_string(), Some(ttl)).await;
            Ok("+OK\r\n".to_string())
        }

        ["SET", key, val] => {
            db.set(key.to_string(), val.to_string(), None).await;
            Ok("+OK\r\n".to_string())
        }
        ["GET", key] => {
            if let Some(val) = db.get(key).await {
                Ok(format!("${}\r\n{},\r\n", val.len(), val))
            } else {
                Ok("$-1\r\n".to_string())
            }
        }
        ["DEL", key] => {
            if db.delete(key).await {
                Ok("$+1\r\n".to_string())
            } else {
                Ok("$-0\r\n".to_string())
            }
        }
        // For list
        ["LPUSH", key, val] => {
            db.lpush(key.to_string(), val.to_string()).await;
            Ok("+OK\r\n".to_string())
        }

        ["LPOP", key] => {
            if let Some(popped) = db.lpop(key).await {
                Ok(format!("${}\r\n{},\r\n", popped.len(), popped))
            } else {
                Ok("+OK\r\n".to_string())
            }
        }

        ["RPUSH", key, val] => {
            db.rpush(key.to_string(), val.to_string()).await;
            Ok("+OK\r\n".to_string())
        }

        ["RPOP", key] => {
            if let Some(popped) = db.rpop(key).await {
                Ok(format!("${}\r\n{},\r\n", popped.len(), popped))
            } else {
                Ok("+OK\r\n".to_string())
            }
        }
        ["LRANGE", start, end, key] => {
            let start = start
                .parse::<usize>()
                .map_err(|_| "Start index is invalid")?;
            let end = end.parse::<usize>().map_err(|_| "End index is invalid")?;
            if let Some(list) = db.lrange(start, end, key).await {
                let mut response_var = format!("*{}\r\n", list.len());
                for items in list {
                    response_var.push_str(&format!("${}\r\n{}\r\n", items.len(), items));
                }
                Ok(response_var)
            } else {
                Ok("+OK\r\n".to_string())
            }
        }

        _ => Ok("-ERR\r\nCommand is unknown, Please try again\n".to_string()),
    }
}
