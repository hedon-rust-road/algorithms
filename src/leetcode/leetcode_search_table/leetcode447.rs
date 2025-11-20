use std::collections::HashMap;

use super::Solution;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        points
            .iter()
            .map(|center| {
                let distance_counts = points.iter().filter(|&p| p != center).fold(
                    HashMap::new(),
                    |mut map, point| {
                        *map.entry(distance(center, point)).or_insert(0) += 1;
                        map
                    },
                );

                distance_counts
                    .values()
                    .map(|&count| count * (count - 1))
                    .sum::<i32>()
            })
            .sum()
    }
}

fn distance(point1: &Vec<i32>, point2: &Vec<i32>) -> i32 {
    (point1[0] - point2[0]) * (point1[0] - point2[0])
        + (point1[1] - point2[1]) * (point1[1] - point2[1])
}
