use crate::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() < 2 { return true; }
        if nums[0] == 0 { return false; }

        let mut i = nums.len() - 2;
        while i > 0 {
            if nums[i] != 0 {
                i -= 1;
                continue;
            }
            let mut can = false;
            for j in (0..i).rev() {
                if nums[j] as usize > i - j {
                    i = j;
                    can = true;
                    break;
                }
            }
            if !can { return false; }
        }
        return true;
    }
}