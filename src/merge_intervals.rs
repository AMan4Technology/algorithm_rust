use crate::Solution;

impl Solution {
    pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() < 2 {
            return intervals;
        }
        let mut results = Vec::new();

        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let (mut start, mut end) = (intervals[0][0], intervals[0][1]);
        for i in 1..intervals.len() {
            if intervals[i][0] <= end {
                end = end.max(intervals[i][1]);
                continue;
            }
            results.push(vec![start, end]);
            start = intervals[i][0];
            end = intervals[i][1];
        }
        results.push(vec![start, end]);
        results
    }
}
