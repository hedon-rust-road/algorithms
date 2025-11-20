use std::collections::{HashSet, VecDeque};

use super::Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_map: HashSet<String> = word_list.into_iter().collect();
        if !word_map.contains(&end_word) {
            return 0;
        }

        let mut queue = VecDeque::new();
        queue.push_back(begin_word);

        let mut visited = HashSet::new();

        let end_bytes = end_word.into_bytes();
        let mut step = 1;

        while queue.len() > 0 {
            let current_level_size = queue.len();
            for _ in 0..current_level_size {
                let mut curr_bytes = queue.pop_front().unwrap().into_bytes();
                // 每个位置都尝试从 'a' 替换到 'z'
                for j in 0..curr_bytes.len() {
                    let origin_byte = curr_bytes[j];

                    for k in 0..26 {
                        let byte_to_replace = b'a' + k as u8;
                        if byte_to_replace == origin_byte {
                            continue;
                        }
                        curr_bytes[j] = byte_to_replace;

                        // 找到结尾了
                        if curr_bytes == end_bytes {
                            return step + 1;
                        }

                        // 没有找到，且没有访问过，则入队
                        let curr_str = String::from_utf8(curr_bytes.clone()).unwrap();
                        if word_map.contains(&curr_str) && !visited.contains(&curr_str) {
                            queue.push_back(curr_str.clone());
                            visited.insert(curr_str.clone());
                        }
                    }

                    // 还原
                    curr_bytes[j] = origin_byte;
                }
            }
            step += 1;
        }
        0
    }
}
