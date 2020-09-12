use crate::Solution;

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let (mut max, mut min) = (0, std::i32::MAX);
        let mut sum = arr.iter().fold(0, |sum, &v| {
            max = max.max(v);
            min = min.min(v);
            sum + v
        });
        if sum <= target {
            return max;
        }
        if min * arr.len() as i32 >= target {
            let closer = target / arr.len() as i32;
            let (abs_1, abs_2) = (
                (closer * arr.len() as i32 - target).abs(),
                ((closer + 1) * arr.len() as i32 - target).abs(),
            );
            return if abs_1 <= abs_2 { closer } else { closer + 1 };
        }

        let mut arr = arr;
        arr.sort();

        let mut curr_sum = arr[0];
        for i in 1..arr.len() {
            let sum = curr_sum + (arr.len() - i) as i32 * arr[i];
            if sum == target {
                return arr[i];
            }
            if sum > target {
                let closer = (target - curr_sum) / (arr.len() - i) as i32;
                let (abs_1, abs_2) = (
                    (curr_sum + closer * (arr.len() - i) as i32 - target).abs(),
                    (curr_sum + (closer + 1) * (arr.len() - i) as i32 - target).abs(),
                );
                return if abs_1 <= abs_2 { closer } else { closer + 1 };
            }
            curr_sum += arr[i];
        }
        0
    }
}
