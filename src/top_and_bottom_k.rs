use crate::Solution;

impl Solution {
    pub fn get_least_numbers(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        if arr.len() < k {
            return Vec::new();
        }

        let mut arr = arr;
        let arr = arr.as_mut_slice();
        let (mut start, mut end) = (0, arr.len() - 1);

        while start <= end {
            let i = Self::sort(arr, start, end);
            if k == i || k == i + 1 {
                return arr[..k].to_vec();
            }
            if k < i {
                end = i - 1;
            } else {
                start = i + 1;
            }
        }
        Vec::new()
    }

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        if nums.len() < k {
            return -1;
        }

        let mut arr = nums;
        let arr = arr.as_mut_slice();
        let (mut start, mut end) = (0, arr.len() - 1);

        while start <= end {
            let i = Self::sort(arr, start, end);
            println!("{:?}:{}", arr, i);
            if k == arr.len() - i {
                return arr[i];
            }
            if k < arr.len() - i {
                start = i + 1;
            } else {
                end = i - 1;
            }
        }
        -1
    }

    fn sort(arr: &mut [i32], start: usize, end: usize) -> usize {
        let pivot = arr[end];
        let (mut i, mut j) = (start, start);
        while j < end && i <= end {
            if arr[i] <= pivot {
                i += 1;
                j = i;
                continue;
            }
            while j <= end && arr[j] > pivot {
                j += 1;
            }
            arr.swap(i, j);
        }
        i
    }
}
