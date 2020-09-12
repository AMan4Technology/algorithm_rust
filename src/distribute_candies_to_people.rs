use crate::Solution;

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        fn all_num(start: i32, m: i32, n: i32) -> i32 {
            return (2 * start + (n - 1) * m) * n / 2;
        }

        let mut candies = candies;
        let first = all_num(1, 1, num_people);
        let mut n = 0;
        while candies >= all_num(first, num_people * num_people, n + 1) {
            n += 1;
        }
        candies -= all_num(first, num_people * num_people, n);

        let mut result = vec![0; num_people as usize];
        for i in 0..num_people {
            result[i as usize] = all_num(i + 1, num_people, n);
        }

        let first = 1 + n * num_people;
        for i in 0..num_people {
            if candies >= first + i {
                candies -= first + i;
                result[i as usize] += first + i;
            } else {
                result[i as usize] += candies;
                candies = 0;
                break;
            }
        }

        result
    }
}