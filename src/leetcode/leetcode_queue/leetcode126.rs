use std::collections::{HashMap, HashSet, VecDeque};

use super::Solution;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let word_set: HashSet<String> = word_list.into_iter().collect();
        if !word_set.contains(&end_word) {
            return vec![];
        }

        // parents: 记录每个单词的父节点列表 (child -> [parent1, parent2...])
        let mut parents: HashMap<String, Vec<String>> = HashMap::new();
        // steps: 记录每个单词第一次被访问时的步数
        let mut steps: HashMap<String, i32> = HashMap::new();
        // found: 找到一次就不需要继续往下一层找了
        let mut found = false;

        // 初始化队列
        let mut queue = VecDeque::new();
        queue.push_back(begin_word.clone());
        steps.insert(begin_word.clone(), 0);
        let mut step_count = 0;

        while !queue.is_empty() && !found {
            let level_size = queue.len();
            step_count += 1;

            // 尝试当前层
            for _ in 0..level_size {
                let curr_str = queue.pop_front().unwrap();
                let mut curr = curr_str.clone().into_bytes();
                // 尝试替换每一个位置
                for j in 0..curr.len() {
                    let origin_byte = curr[j];

                    for k in 0..26 {
                        let byte_to_replace = b'a' + k;

                        // 相同的直接返回
                        if origin_byte == byte_to_replace {
                            continue;
                        }

                        // 必须是合法单词
                        curr[j] = byte_to_replace;
                        let next = String::from_utf8(curr.clone()).unwrap();
                        if !word_set.contains(&next) {
                            continue;
                        }

                        // 标记找到
                        if &next == &end_word {
                            found = true
                        }

                        if let Some(step) = steps.get(&next) {
                            if step == &step_count {
                                // 如果之前看到过，且 step == step_count，说明有别的路径可以到达 next
                                // step < step_count 说明之前是更短的路径，则不需要考虑当前的路径了
                                parents.entry(next).or_insert(vec![]).push(curr_str.clone());
                            }
                        } else {
                            // 之前没看到过，则入队
                            parents.insert(next.clone(), vec![curr_str.clone()]);
                            steps.insert(next.clone(), step_count);
                            queue.push_back(next);
                        }
                    }

                    // 还原
                    curr[j] = origin_byte
                }
            }
        }

        // BFS 回溯路径
        let mut res = vec![];
        if found {
            let mut path = vec![end_word.clone()];
            backtrack(&end_word, &begin_word, &parents, &mut path, &mut res);
        }
        res
    }
}

fn backtrack(
    curr: &str,
    target: &str,
    parents: &HashMap<String, Vec<String>>,
    path: &mut Vec<String>,
    res: &mut Vec<Vec<String>>,
) {
    if curr == target {
        // 如果找到了，则将 path 倒序存入 res
        let mut tmp = path.clone();
        tmp.reverse();
        res.push(tmp);
        return;
    }

    // 没找到则继续回溯
    if let Some(ps) = parents.get(curr) {
        for parent in ps {
            path.push(parent.clone());
            backtrack(&parent, target, parents, path, res);
            path.pop();
        }
    }
}
