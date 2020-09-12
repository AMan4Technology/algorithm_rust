use crate::Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        if num_courses < 1 {
            return Vec::new();
        }

        let num_courses = num_courses as usize;
        let mut result = Vec::with_capacity(num_courses);
        let mut counts = vec![0; num_courses];
        let mut next = vec![Vec::new(); num_courses];

        for prev in prerequisites.iter() {
            next[prev[1] as usize].push(prev[0] as usize);
            counts[prev[0] as usize] += 1usize;
        }

        let mut queue = Vec::new();
        for i in 0..counts.len() {
            if counts[i] == 0 {
                queue.push(i);
            }
        }

        while queue.len() != 0 {
            let mut new_queue = Vec::new();
            for &i in queue.iter() {
                result.push(i as i32);
                for &j in next[i].iter() {
                    if counts[j] == 0 {
                        return Vec::new();
                    }
                    counts[j] -= 1;
                    if counts[j] == 0 {
                        new_queue.push(j);
                    }
                }
            }
            queue = new_queue;
        }

        if result.len() == num_courses {
            result
        } else {
            Vec::new()
        }
    }
}
