use std::collections::{HashSet, VecDeque};

pub struct Rlist {
    pub list: VecDeque<String>,
}
impl Rlist {
    pub fn new() -> Self {
        Rlist {
            list: VecDeque::new(),
        }
    }
    pub fn lpush(&mut self, val: String) {
        self.list.push_front(val);
    }
    pub fn rpush(&mut self, val: String) {
        self.list.push_back(val);
    }
    pub fn lpop(&mut self) -> Option<String> {
        self.list.pop_front()
    }
    pub fn rpop(&mut self) -> Option<String> {
        self.list.pop_back()
    }
    pub fn lrange(&mut self, start: usize, end: usize) -> Vec<String> {
        self.list
            .iter()
            .skip(start)
            .take(end - start + 1)
            .cloned()
            .collect()
    }
}
pub struct RSets {
    pub set: HashSet<String>,
}
impl RSets {
    pub fn new() -> Self {
        RSets {
            set: HashSet::new(),
        }
    }
    pub fn sadd(&mut self, val: String) -> bool {
        self.set.insert(val)
    }
    pub fn srem(&mut self, val: &str) -> bool {
        self.set.remove(val)
    }
    pub fn smembers(&self) -> Vec<String> {
        self.set.iter().cloned().collect()
    }
    pub fn ismember(&self, key: &str) -> bool {
        self.set.contains(key)
    }
}
