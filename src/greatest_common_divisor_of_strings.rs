use crate::Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let (str1, str2) = (str1.as_bytes(), str2.as_bytes());
        let mut results = vec![];

        let length = str1.len().min(str2.len());
        for i in 0..length {
            if str1[i] != str2[i] { return String::new(); }
            results = results.iter().filter_map(|&t| if str1[i % t] == str1[i] { Some(t) } else { None }).collect();
            if str1.len() % (i + 1) == 0 && str2.len() % (i + 1) == 0 { results.push(i + 1); }
        }

        for i in length..str1.len() {
            results = results.iter().filter_map(|&t| if str1[i % t] == str1[i] { Some(t) } else { None }).collect();
        }

        for i in length..str2.len() {
            results = results.iter().filter_map(|&t| if str2[i % t] == str2[i] { Some(t) } else { None }).collect();
        }

        if results.is_empty() { String::new() } else {
            String::from_utf8_lossy(&str2[..results[results.len() - 1]]).to_string()
        }
    }
}