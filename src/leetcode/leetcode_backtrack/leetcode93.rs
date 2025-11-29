use super::Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        if s.len() < 4 || s.len() > 12 {
            return vec![];
        }
        let mut res = vec![];
        let mut current = String::new();
        restore_ip_addresses_helper(s.as_bytes(), 0, &mut current, &mut res);
        res
    }
}

// remain: s 的剩余部分
// target_index: 当前要确定第几个部分的ip，从0开始
// tmp: 当前已经组装好的合法 ip 前缀
// res: 最终的结果集
fn restore_ip_addresses_helper(
    remain: &[u8],
    target_index: i8,
    tmp: &mut String,
    res: &mut Vec<String>,
) {
    // 递归出口
    if target_index == 4 {
        if remain.is_empty() {
            res.push(tmp.clone());
        }
        return;
    }

    // 尝试 remain 的前 3 位
    for i in 0..=2 {
        let i = i as usize;

        // 长度不足，剪枝
        if remain.len() < i + 1 {
            break;
        }

        // 前导 0，剪枝
        if i > 0 && remain[0] == b'0' {
            break;
        }

        // 大小不满足，剪枝
        let current = &remain[..i + 1];
        let current_str = std::str::from_utf8(current).unwrap_or("-1");
        let current_num: i32 = current_str.parse().unwrap_or(-1);
        if current_num < 0 || current_num > 255 {
            break;
        }

        // 拼接
        let len_before = tmp.len();
        if !tmp.is_empty() {
            tmp.push('.');
        }
        tmp.push_str(current_str);

        // 递归
        restore_ip_addresses_helper(&remain[i + 1..], target_index + 1, tmp, res);

        // 回溯
        tmp.truncate(len_before);
    }
}
