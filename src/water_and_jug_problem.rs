use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn can_measure_water(x: i32, y: i32, z: i32) -> bool {
        if z == 0 || z == x || z == y || z == x + y {
            return true;
        }
        if x == 0 || y == 0 || z > x + y || x == y {
            return false;
        }

        z % Self::greatest_common_divisor(x, y) == 0
    }
    pub fn greatest_common_divisor(x: i32, y: i32) -> i32 {
        let (mut a, mut b) = if x < y { (x, y) } else { (y, x) };
        while b != 0 {
            let temp = a % b;
            a = b;
            b = temp;
        }
        a
    }
}
