use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

use super::ds::{RSets, Rlist};

#[derive(Clone)]
pub struct Database {
    db: Arc<RwLock<HashMap<String, String>>>,
    expiry: Arc<RwLock<HashMap<String, Instant>>>,
    list: Arc<RwLock<HashMap<String, Rlist>>>,
    set: Arc<RwLock<HashMap<String, RSets>>>,
}
impl Database {
    pub fn new() -> Self {
        Database {
            db: Arc::new(RwLock::new(HashMap::new())),
            expiry: Arc::new(RwLock::new(HashMap::new())),
            list: Arc::new(RwLock::new(HashMap::new())),
            set: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, key: String, value: String, time_to_live: Option<u64>) {
        let mut db_map = self.db.write().await;
        db_map.insert(key.clone(), value);
        if let Some(ttl_sec) = time_to_live {
            let mut exp_map = self.expiry.write().await;
            let exp_time = Instant::now() + Duration::from_secs(ttl_sec);
            exp_map.insert(key, exp_time);
        }
    }
    pub async fn get(&self, key: &str) -> Option<String> {
        if self.is_expired(key).await {
            self.delete(key).await;
            return None;
        }
        let db_map = self.db.read().await;
        db_map.get(key).cloned()
    }
    pub async fn delete(&self, key: &str) -> bool {
        let mut del_map = self.db.write().await;
        let mut exp_map = self.expiry.write().await;
        exp_map.remove(key);
        match del_map.remove(key) {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn is_expired(&self, key: &str) -> bool {
        let expiry_map = self.expiry.read().await;
        if let Some(&expiry_time) = expiry_map.get(key) {
            return Instant::now() > expiry_time;
        }
        false
    }

    // for Rlist
    pub async fn lpush(&self, key: String, value: String) {
        let mut list_map = self.list.write().await;
        let list = list_map.entry(key).or_insert_with(Rlist::new);
        list.lpush(value);
    }
    pub async fn rpush(&self, key: String, value: String) {
        let mut list_map = self.list.write().await;
        let list = list_map.entry(key).or_insert_with(Rlist::new);
        list.rpush(value);
    }
    pub async fn rpop(&self, key: &str) -> Option<String> {
        let mut list_map = self.list.write().await;
        if let Some(list) = list_map.get_mut(key) {
            list.rpop()
        } else {
            None
        }
    }
    pub async fn lpop(&self, key: &str) -> Option<String> {
        let mut list_map = self.list.write().await;
        if let Some(list) = list_map.get_mut(key) {
            list.lpop()
        } else {
            None
        }
    }
    pub async fn lrange(&self, start: usize, end: usize, key: &str) -> Option<Vec<String>> {
        let mut list_map = self.list.write().await;
        if let Some(list) = list_map.get_mut(key) {
            Some(list.lrange(start, end))
        } else {
            None
        }
    }

    // For RSet
    pub async fn sadd(&self, key: String, value: String) -> bool {
        let mut set_map = self.set.write().await;
        let mut set = set_map.entry(key).or_insert_with(RSets::new);
        set.sadd(value)
    }
    pub async fn srem(&self, key: &str, value: &str) -> bool {
        let mut set_map = self.set.write().await;
        if let Some(set) = set_map.get_mut(key) {
            set.srem(value)
        } else {
            false
        }
    }
    pub async fn smembers(&self, key: &str) -> Option<Vec<String>> {
        let mut set_map = self.set.read().await;
        if let Some(set) = set_map.get(key) {
            Some(set.smembers())
        } else {
            None
        }
    }
    pub async fn ismember(&self, key: &str, val: &str) -> bool {
        let mut set_map = self.set.read().await;
        if let Some(set) = set_map.get(key) {
            set.ismember(val)
        } else {
            false
        }
    }
}
