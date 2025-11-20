pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = height.len() - 1;
    let mut max: i32 = min_height(&height, left, right) * (right - left) as i32;

    while left < right {
        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }

        let tmp = min_height(&height, left, right) * (right - left) as i32;
        if tmp > max {
            max = tmp;
        }
    }

    max
}

fn min_height(height: &Vec<i32>, left: usize, right: usize) -> i32 {
    if height[left] > height[right] {
        height[right]
    } else {
        height[left]
    }
}
