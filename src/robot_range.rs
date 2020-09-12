use crate::Solution;

impl Solution {
    pub fn moving_count(m: i32, n: i32, k: i32) -> i32 {
        let (m, n, k) = (m as usize, n as usize, k as usize);
        let mut count_m = vec![0; m];
        let mut count_n = vec![0; n];

        let mut points = vec![vec![false; n]; m];
        for i in 0..m {
            count_m[i] = Self::get_count(i);
        }
        for i in 0..n {
            count_n[i] = Self::get_count(i);
        }
        let mut count = 0;
        Self::go(0, 0, k, &mut points, &count_m, &count_n, m, n, &mut count);
        count
    }

    fn get_count(x: usize) -> usize {
        let mut count = 0;
        let mut x = x;
        while x != 0 {
            count += x % 10;
            x /= 10;
        }
        count
    }

    fn go(
        x: usize,
        y: usize,
        k: usize,
        points: &mut Vec<Vec<bool>>,
        count_m: &Vec<usize>,
        count_n: &Vec<usize>,
        m: usize,
        n: usize,
        count: &mut i32,
    ) {
        if count_m[x] + count_n[y] > k || points[x][y] {
            return;
        }
        *count += 1;
        points[x][y] = true;
        if x > 0 {
            Self::go(x - 1, y, k, points, count_m, count_n, m, n, count)
        }
        if x < m - 1 {
            Self::go(x + 1, y, k, points, count_m, count_n, m, n, count)
        }
        if y > 0 {
            Self::go(x, y - 1, k, points, count_m, count_n, m, n, count)
        }
        if y < n - 1 {
            Self::go(x, y + 1, k, points, count_m, count_n, m, n, count)
        }
    }
}
