use crate::Solution;

impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, a.len() - 1);
        let mut score = a[left] + a[right] - (right - left) as i32;
        let (mut next_left, mut next_right) = (left, right);
        while left < right {
            if left == next_left {
                for i in left + 1..right {
                    if a[i] - a[left] + (i - left) as i32 > 0 {
                        next_left = i;
                        break;
                    }
                }
                if next_left == left {
                    next_left += 1;
                }
            }
            if right == next_right {
                for i in (left + 1..right).rev() {
                    if a[i] - a[right] + (right - i) as i32 > 0 {
                        next_right = i;
                        break;
                    }
                }
                if next_right == right {
                    next_right -= 1;
                }
            }

            println!("{}", next_left);

            let (left_change, right_change) = (
                a[next_left] - a[left] + (next_left - left) as i32,
                a[next_right] - a[right] + (right - next_right) as i32,
            );
            if left_change <= 0 && right_change <= 0 {
                break;
            }

            if left_change <= right_change {
                if next_right <= left {
                    next_right = right;
                    continue;
                }
                right = next_right;
                score += right_change;
            } else {
                if next_left >= right {
                    next_left = left;
                    continue;
                }
                left = next_left;
                score += left_change;
            }
        }
        score
    }
}
