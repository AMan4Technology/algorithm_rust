use crate::Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let (is_left, is_right) = (x_center < x1, x_center > x2);
        let (is_up, is_down) = (y_center > y2, y_center < y1);

        if !(is_left || is_right || is_up || is_down) {
            return true;
        }

        match (is_left, is_up, is_right, is_down) {
            (true, false, false, false) => x1 - x_center <= radius,
            (true, true, false, false) => {
                (x1 - x_center).pow(2) + (y_center - y2).pow(2) <= radius.pow(2)
            }
            (false, true, false, false) => y_center - y2 <= radius,
            (false, true, true, false) => {
                (x_center - x2).pow(2) + (y_center - y2).pow(2) <= radius.pow(2)
            }
            (false, false, true, false) => x_center - x2 <= radius,
            (false, false, true, true) => {
                (x_center - x2).pow(2) + (y1 - y_center).pow(2) <= radius.pow(2)
            }
            (false, false, false, true) => y1 - y_center <= radius,
            (true, false, false, true) => {
                (x1 - x_center).pow(2) + (y1 - y_center).pow(2) <= radius.pow(2)
            }
            _ => false,
        }
    }
}
