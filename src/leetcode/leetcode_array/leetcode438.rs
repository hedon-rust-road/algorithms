pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if p.len() > s.len() {
        return vec![];
    }

    let mut result = vec![];
    let s_bs = s.as_bytes();
    let p_bs = p.as_bytes();

    // 只有小写字母
    let mut p_count = [0; 26];
    let mut window_count = [0; 26];

    // 初始化 p 窗口
    for &ch in p_bs {
        p_count[(ch - b'a') as usize] += 1;
    }

    // 初始化第一个窗口
    for i in 0..p_bs.len() {
        window_count[(s_bs[i] - b'a') as usize] += 1;
    }

    if window_count == p_count {
        result.push(0);
    }

    // 滑动窗口
    for (i, &ch) in s_bs[p_bs.len()..].iter().enumerate() {
        // 移除左边字符
        window_count[(s_bs[i - p_bs.len()] - b'a') as usize] -= 1;
        // 添加右边字符
        window_count[(ch - b'a') as usize] += 1;

        if window_count == p_count {
            result.push((i - p_bs.len() + 1) as i32);
        }
    }

    result
}
