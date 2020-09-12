use crate::Solution;

impl Solution {
    pub fn is_palindrome_num(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }
        let (mut rev_num, mut x) = (0, x);
        while x > rev_num {
            rev_num = rev_num * 10 + x % 10;
            x /= 10;
        }
        rev_num == x || rev_num / 10 == x
    }
}
