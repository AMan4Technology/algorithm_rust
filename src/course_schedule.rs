use crate::Solution;
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut courses = vec![HashSet::new(); num_courses as usize];
        let mut nums = vec![0; num_courses as usize];

        for prev in prerequisites.iter() {
            courses[prev[1] as usize].insert(prev[0] as usize);
            nums[prev[0] as usize] += 1;
        }

        let mut queue = VecDeque::with_capacity(num_courses as usize);
        for i in 0..num_courses as usize {
            if nums[i] == 0 {
                queue.push_back(i);
            }
        }

        while queue.len() != 0 {
            if let Some(i) = queue.pop_front() {
                for &x in courses[i].iter() {
                    nums[x] -= 1;
                    if nums[x] == 0 {
                        queue.push_back(x);
                    }
                }
            }
        }

        for i in 0..num_courses as usize {
            if nums[i] > 0 {
                return false;
            }
        }
        true
    }
}
