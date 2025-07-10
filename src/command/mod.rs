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
                let mut response_var = format!("*{} - ITEM(s)\r\n", list.len());
                for items in list {
                    response_var.push_str(&format!("${}\r\n{}\r\n", items.len(), items));
                }
                Ok(response_var)
            } else {
                Ok(format!("+ \"{}\" - LIST NOT FOUND\r\n", key))
            }
        }

        //SET operations here
        ["SADD", key, val] => {
            let add = db.sadd(key.to_string(), val.to_string()).await;
            Ok(format!("${}\r\n", if add { 1 } else { 0 }))
        }
        ["SREM", key, val] => {
            let rem = db.srem(key, val).await;
            Ok(format!("${}\r\n", if rem { 1 } else { 0 }))
        }
        ["SMEMBERS", key] => {
            // Ok(format!("${}\r\n", if rem { 1 } else { 0 }))
            if let Some(list) = db.smembers(key).await {
                let mut response_var = format!("*{} - ITEM(s)\r\n", list.len());
                for items in list {
                    response_var.push_str(&format!("${}\r\n{}\r\n", items.len(), items));
                }
                Ok(response_var)
            } else {
                Ok(format!("+ \"{}\" - SET NOT FOUND\r\n", key))
            }
        }
        ["ISMEMBER", key, member] => {
            let is_mem = db.ismember(key, member).await;
            Ok(format!("${}\r\n", if is_mem { 1 } else { 0 }))
        }

        _ => Ok("-ERR\r\nCommand is unknown, Please try again\n".to_string()),
    }
}
