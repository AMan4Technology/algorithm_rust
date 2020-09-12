use crate::Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (m, n) = (m as usize, n as usize);
        if n == 0 { return; }
        if m == 0 {
            nums1[..n].copy_from_slice(&nums2[..n]);
            return;
        }

        let (mut i, mut j) = (0, 0);
        let mut result = vec![];
        while i < m && j < n {
            if nums1[i] <= nums2[j] {
                result.push(nums1[i]);
                i += 1;
            } else {
                result.push(nums2[j]);
                j += 1;
            }
        }
        if i < m {
            nums1.copy_within(i..m, result.len());
        } else {
            nums1[result.len()..m + n].copy_from_slice(&nums2[j..n]);
        }
        nums1[..result.len()].copy_from_slice(&result[..result.len()])
    }
}