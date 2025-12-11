use super::Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_tank = 0;
        let mut current_tank = 0;
        let mut start = 0;

        for i in 0..gas.len() {
            let net_gain = gas[i] - cost[i];

            total_tank += net_gain;
            current_tank += net_gain;

            if current_tank < 0 {
                start = i + 1;
                current_tank = 0;
            }
        }

        if total_tank < 0 { -1 } else { start as i32 }
    }
}
