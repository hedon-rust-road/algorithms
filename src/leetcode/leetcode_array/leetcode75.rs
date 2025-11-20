pub fn sort_colors(nums: &mut Vec<i32>) {
    if nums.len() == 0 {
        return;
    }

    // [0, p] 全为 0
    // [q, len-1] 全为 2
    let mut p: i32 = -1;
    let mut q = nums.len();
    let mut i = 0;
    while i < q {
        match nums[i] {
            0 => {
                p += 1;
                if p as usize != i {
                    nums.swap(p as usize, i);
                }
                i += 1;
            }
            2 => {
                q -= 1;
                nums.swap(q, i);
            }
            _ => {
                i += 1;
            }
        }
    }
}
