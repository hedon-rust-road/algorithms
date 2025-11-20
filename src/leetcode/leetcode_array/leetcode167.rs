pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left: usize = 0;
    let mut right: usize = numbers.len() - 1;

    while left < right {
        if numbers[left] + numbers[right] == target {
            return vec![(left + 1) as i32, (right + 1) as i32];
        } else if numbers[left] + numbers[right] < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    panic!("unreachable!")
}
