use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    if s.len() < t.len() {
        return "".to_string();
    }

    let s_bs = s.as_bytes();
    let t_bs = t.as_bytes();

    // 统计 t 字符频率
    let mut t_map = HashMap::new();
    for &ch in t_bs {
        *t_map.entry(ch).or_insert(0) += 1;
    }

    let mut window = HashMap::new();
    let mut left = 0;
    let mut matched = 0;
    let required = t_map.len();
    let mut result = (0, usize::MAX); // (start, length)

    for right in 0..s_bs.len() {
        let right_ch = s_bs[right];
        *window.entry(right_ch).or_insert(0) += 1;

        // 找右边界
        if let Some(&count) = t_map.get(&right_ch) {
            if window[&right_ch] == count {
                matched += 1;
            }
        }

        while matched == required {
            // 先保存结果
            if right - left + 1 < result.1 {
                result = (left, right - left + 1);
            }

            // 收缩左边界
            let left_ch = s_bs[left];
            if let Some(&count) = t_map.get(&left_ch) {
                if window[&left_ch] == count {
                    matched -= 1;
                }
            }

            *window.entry(left_ch).or_insert(0) -= 1;
            left += 1;
        }
    }

    if result.1 == usize::MAX {
        "".to_string()
    } else {
        String::from_utf8_lossy(&s_bs[result.0..result.0 + result.1]).to_string()
    }
}
