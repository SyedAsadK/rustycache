use std::collections::VecDeque;

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
