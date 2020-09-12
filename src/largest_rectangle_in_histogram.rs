use crate::Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        if heights.is_empty() {
            return 0;
        }

        let (mut max_idx, mut min_idx) = (0, 0);
        for i in 1..heights.len() {
            if max_idx == i - 1 && heights[i] >= heights[max_idx] {
                max_idx = i;
            }
            if min_idx == i - 1 && heights[i] <= heights[min_idx] {
                min_idx = i;
            }
        }
        let mut result = heights[0] as usize;
        if max_idx == heights.len() - 1 {
            for i in 0..heights.len() {
                result = result.max((max_idx - i + 1) * heights[i] as usize);
            }
            return result as i32;
        }
        if min_idx == heights.len() - 1 {
            for i in 0..heights.len() {
                result = result.max((i + 1) * heights[i] as usize);
            }
            return result as i32;
        }

        let mut max = vec![(0, 0); heights.len()];
        let mut curr = vec![Vec::new(); heights.len()];
        max[0] = (1, heights[0] as usize);
        curr[0] = vec![(0, 1)];

        for i in 1..heights.len() {
            if heights[i] == 0 {
                continue;
            }

            let mut top = i;
            for j in (0..i).rev() {
                if heights[j] < heights[i] {
                    break;
                }
                top = j;
            }
            max[i] = (i - top + 1, heights[i] as usize);
            if max[i].0 * max[i].1 < (max[top].0 + i - top) * max[top].1.min(heights[i] as usize) {
                max[i] = (max[top].0 + i - top, max[top].1.min(heights[i] as usize));
            }
            if top > 0 {
                for j in 0..curr[top - 1].len() {
                    let (j, length) = curr[top - 1][j];
                    if max[i].0 * max[i].1 < (i - j + length) * heights[j] as usize {
                        max[i] = (i - j + length, heights[j] as usize);
                    }
                    curr[i].push((j, length));
                }
            }
            curr[i].push((i, i - top + 1));

            println!("{}:{},{}", i, max[i].0, max[i].1);

            if max[i].0 * max[i].1 > result {
                result = max[i].0 * max[i].1;
            }
        }
        result as i32
    }
}
