use rand::Rng;
use std::collections::HashMap;

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */

struct RandomizedSet {
    hash: HashMap<i32, usize>, // key: val, value: index
    slice: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            hash: HashMap::new(),
            slice: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.hash.contains_key(&val) {
            return false;
        }
        self.slice.push(val);
        self.hash.insert(val, self.slice.len() - 1);
        return true;
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.hash.remove(&val) {
            None => false,
            Some(index) => {
                let len = self.slice.len();
                // 交换
                if index != len - 1 {
                    // 更新索引
                    self.slice.swap(len - 1, index);
                    self.hash.insert(self.slice[index], index);
                }
                // 取出最后一个
                self.slice.pop();
                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        let index = rand::rng().random_range(0..self.slice.len());
        self.slice[index]
    }
}
