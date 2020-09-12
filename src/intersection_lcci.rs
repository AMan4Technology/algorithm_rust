use crate::Solution;

impl Solution {
    pub fn intersection(
        start1: Vec<i32>,
        end1: Vec<i32>,
        start2: Vec<i32>,
        end2: Vec<i32>,
    ) -> Vec<f64> {
        let (mut start1, mut end1) = if start1[0] <= end1[0] {
            (
                (start1[0] as f64, start1[1] as f64),
                (end1[0] as f64, end1[1] as f64),
            )
        } else {
            (
                (end1[0] as f64, end1[1] as f64),
                (start1[0] as f64, start1[1] as f64),
            )
        };
        let (mut start2, mut end2) = if start2[0] <= end2[0] {
            (
                (start2[0] as f64, start2[1] as f64),
                (end2[0] as f64, end2[1] as f64),
            )
        } else {
            (
                (end2[0] as f64, end2[1] as f64),
                (start2[0] as f64, start2[1] as f64),
            )
        };
        let mut result = Vec::with_capacity(2);
        if start1.0 == end1.0 && start2.0 == end2.0 {
            if start1.0 != start2.0
                || start1.1.max(end1.1) < start2.1.min(end2.1)
                || start2.1.max(end2.1) < start1.1.min(end1.1)
            {
                return result;
            }
            result.push(start1.0);
            result.push(start1.1.min(end1.1).max(start2.1.min(end2.1)));
            return result;
        }
        if start2.0 == end2.0 {
            let mut tmp = start2;
            start2 = start1;
            start1 = tmp;
            tmp = end2;
            end2 = end1;
            end1 = tmp;
        }
        if start1.0 == end1.0 {
            if start1.0 < start2.0 || end2.0 < start1.0 {
                return result;
            }
            let k = (end2.1 - start2.1) / (end2.0 - start2.0);
            let b = k * (start1.0 - start2.0) + start2.1;
            if b > start1.1.max(end1.1) || b < start1.1.min(end1.1) {
                return result;
            }
            result.push(start1.0);
            result.push(b);
            return result;
        }
        let (k1, k2) = (
            (end1.1 - start1.1) / (end1.0 - start1.0),
            (end2.1 - start2.1) / (end2.0 - start2.0),
        );
        let (b1, b2) = (start1.1 - k1 * start1.0, start2.1 - k2 * start2.0);
        if k1 == k2 {
            if b1 != b2
                || start1.1.max(end1.1) < start2.1.min(end2.1)
                || start2.1.max(end2.1) < start1.1.min(end1.1)
            {
                return result;
            }
            let x = start1.0.min(end1.0).max(start2.0.min(end2.0));
            result.push(x);
            result.push(k1 * x + b1);
            return result;
        }
        let x = (b2 - b1) / (k1 - k2);
        if x < start1.0.min(end1.0)
            || x > start1.0.max(end1.0)
            || x < start2.0.min(end2.0)
            || x > start2.0.max(end2.0)
        {
            return result;
        }
        result.push(x);
        result.push(k1 * x + b1);
        result
    }
}
