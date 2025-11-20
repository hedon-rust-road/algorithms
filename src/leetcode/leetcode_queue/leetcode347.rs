use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use super::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut m: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *m.entry(num).or_default() += 1;
        }
        let mut min_heap = BinaryHeap::with_capacity(k as usize);
        for (num, freq) in m {
            if min_heap.len() < k as usize {
                min_heap.push(Reverse((freq, num)));
            } else {
                let Reverse(peek) = min_heap.peek().unwrap();
                if peek.0 < freq {
                    min_heap.pop();
                    min_heap.push(Reverse((freq, num)));
                }
            }
        }
        min_heap.into_iter().map(|Reverse((_, num))| num).collect()
    }
}
