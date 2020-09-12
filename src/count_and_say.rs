use crate::Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut result = vec![b'1'];
        for _ in 2..=n {
            let (mut count, mut value) = (0, b' ');
            let mut temp = vec![];
            result.push(b' ');
            for v in result.iter() {
                if *v != value {
                    if count > 0 {
                        for x in count.to_string().as_bytes() {
                            temp.push(*x);
                        }
                        temp.push(value);
                    }
                    value = *v;
                    count = 1;
                } else { count += 1; }
            }
            result = temp;
        }
        String::from_utf8(result).unwrap()
    }
}