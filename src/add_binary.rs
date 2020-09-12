use crate::Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (short, long) = if a.len() < b.len() { (a, b) } else { (b, a) };
        if short.len() == 0 { return long; }

        let (short, mut long) = (short.as_bytes(), Vec::from(long));
        long.insert(0, b'0');
        let (len, len_l) = (long.len() - short.len(), long.len());
        for i in (len..len_l).rev() {
            let more = long[i] + short[i - len] - 2 * b'0';
            long[i] = b'0' + more % 2;
            long[i - 1] += more / 2;
        }

        for i in (1..len).rev() {
            if long[i] < b'2' { break; }
            long[i] = b'0';
            long[i - 1] += 1;
        }

        if long[0] == b'0' {
            long.remove(0);
        }
        String::from_utf8(long).unwrap()
    }
}