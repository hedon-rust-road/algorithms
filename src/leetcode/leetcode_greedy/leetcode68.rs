use super::Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;
        let mut tmp = vec![vec![]; 1];
        let mut cur_level_len = 0;
        let mut cur_level = 0;
        for word in words {
            if cur_level_len + word.len() > max_width {
                tmp.push(vec![]);
                cur_level += 1;
                cur_level_len = 0;
            }
            cur_level_len += word.len() + 1; // 加一个空格符
            tmp[cur_level].push(word);
        }

        let mut res = vec![];
        for (line_num, line) in tmp.iter().enumerate() {
            let mut str = String::new();

            // 计算当前长度
            let len: usize = line.iter().map(|s| s.len()).sum();
            // 计算需要填充的空格数
            let need_blank = max_width - len;

            // 就一个元素，直接在后面填充空格
            if line.len() == 1 {
                str.push_str(&line[0]);
                str.push_str(&" ".repeat(need_blank));
                res.push(str);
                continue;
            }

            // 如果是最后一行，则左对齐就可以
            if line_num == tmp.len() - 1 {
                str = line.join(" "); // 中间填空格
                str.push_str(&" ".repeat(max_width - str.len())); // 后面再填满空格
                res.push(str);
                continue;
            }

            // 计算可以填空的位置
            let can_fill_blank = line.len() - 1;
            // 平均一个位置填几个
            let avg = need_blank / can_fill_blank;
            // 平均后，前面几个位置还需要填
            let remain = need_blank % can_fill_blank;
            for i in 0..line.len() {
                let s = &line[i];
                str.push_str(&s);
                if i == line.len() - 1 {
                    break;
                }
                let mut fill_count = avg;
                if i < remain {
                    fill_count += 1;
                }
                str.push_str(&" ".repeat(fill_count));
            }
            res.push(str);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn full_justify_should_work() {
        let res = Solution::full_justify(
            vec![
                "This".to_string(),
                "is".to_string(),
                "an".to_string(),
                "example".to_string(),
                "of".to_string(),
                "text".to_string(),
                "justification.".to_string(),
            ],
            16,
        );
        println!("{:?}", res);
    }
}
