use std::collections::HashMap;

pub fn frequency_sort(s: String) -> String {
    let mut char_freq = HashMap::new();
    s.chars().for_each(|x| {
        *char_freq.entry(x).or_insert(0) += 1;
    });

    let n = s.len();
    let mut buckets: Vec<Vec<char>> = vec![vec![]; n + 1];
    for (c, freq) in char_freq {
        buckets[freq].push(c);
    }

    let mut result = String::new();
    for freq in (1..=n).rev() {
        for &c in &buckets[freq] {
            result.extend(std::iter::repeat(c).take(freq));
        }
    }
    result
}
