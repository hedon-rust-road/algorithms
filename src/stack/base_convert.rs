fn divide_by_two(mut dec_num: u32) -> String {
    let mut rem_stack = vec![];

    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }

    let mut bin_str = "".to_string();
    while rem_stack.len() > 0 {
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }

    bin_str
}

fn base_converter(mut dec_num: u32, base: u32) -> String {
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    assert!(base > 0 && base <= 16);

    let mut rem_stack = vec![];

    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    let mut base_str = "".to_string();
    while let Some(rem) = rem_stack.pop() {
        let rem = rem as usize;
        base_str += &digits[rem].to_string();
    }

    base_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_by_two_basic() {
        assert_eq!(divide_by_two(0), "");
        assert_eq!(divide_by_two(1), "1");
        assert_eq!(divide_by_two(2), "10");
        assert_eq!(divide_by_two(3), "11");
        assert_eq!(divide_by_two(4), "100");
        assert_eq!(divide_by_two(5), "101");
        assert_eq!(divide_by_two(10), "1010");
    }

    #[test]
    fn test_divide_by_two_large_number() {
        // 255 in binary is 11111111, so reversed: "11111111"
        assert_eq!(divide_by_two(255), "11111111");
        // 256 in binary is 100000000, so reversed: "000000001"
        assert_eq!(divide_by_two(256), "100000000");
    }

    #[test]
    fn test_divide_by_two_edge_cases() {
        // Test a number that is a power of two
        assert_eq!(divide_by_two(8), "1000");
        // Test a number that is not a power of two
        assert_eq!(divide_by_two(9), "1001");
    }

    #[test]
    fn test_base_converter_basic() {
        assert_eq!(base_converter(0, 2), "");
        assert_eq!(base_converter(1, 2), "1");
        assert_eq!(base_converter(10, 2), "1010");
        assert_eq!(base_converter(10, 8), "12");
        assert_eq!(base_converter(10, 10), "10");
        assert_eq!(base_converter(10, 16), "A");
        assert_eq!(base_converter(255, 16), "FF");
        assert_eq!(base_converter(255, 2), "11111111");
        assert_eq!(base_converter(255, 8), "377");
    }

    #[test]
    fn test_base_converter_edge_cases() {
        // base 0 is invalid, should panic
        let result = std::panic::catch_unwind(|| base_converter(10, 0));
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| base_converter(10, 17));
        assert!(result.is_err());

        // 0 in any base should return empty string
        assert_eq!(base_converter(0, 2), "");
        assert_eq!(base_converter(0, 8), "");
        assert_eq!(base_converter(0, 16), "");
    }

    #[test]
    fn test_base_converter_hex_and_letters() {
        assert_eq!(base_converter(26, 16), "1A");
        assert_eq!(base_converter(31, 16), "1F");
        assert_eq!(base_converter(15, 16), "F");
        assert_eq!(base_converter(16, 16), "10");
        assert_eq!(base_converter(255, 16), "FF");
        assert_eq!(base_converter(4095, 16), "FFF");
    }

    #[test]
    fn test_base_converter_large_numbers() {
        assert_eq!(base_converter(1024, 2), "10000000000");
        assert_eq!(base_converter(4096, 16), "1000");
        assert_eq!(base_converter(65535, 16), "FFFF");
        assert_eq!(base_converter(123456789, 8), "726746425");
    }
}
