use crate::Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        if s.is_empty() {
            return s;
        }

        let s_array = s.as_bytes();
        let mut stack = Vec::new();
        let mut i = 0;
        'outer: while i < s_array.len() {
            match s_array[i] {
                b'0'..=b'9' => {
                    for j in i + 1..s_array.len() {
                        match s_array[j] {
                            b'0'..=b'9' => {
                                continue;
                            }
                            _ => {
                                stack.push(s[i..j].to_string());
                                i = j;
                                continue 'outer;
                            }
                        }
                    }
                }
                b'[' => {
                    stack.push("[".to_string());
                }
                x if x.is_ascii_alphabetic() => {
                    for j in i + 1..s_array.len() {
                        match s_array[j] {
                            x if x.is_ascii_alphabetic() => {
                                continue;
                            }
                            _ => {
                                stack.push(s[i..j].to_string());
                                i = j;
                                continue 'outer;
                            }
                        }
                    }
                    stack.push(s[i..s_array.len()].to_string());
                    break;
                }
                b']' => {
                    let mut value = String::new();
                    while let Some(v) = stack.pop() {
                        match v.as_str() {
                            "[" => {
                                break;
                            }
                            _ => value.insert_str(0, v.as_ref()),
                        }
                    }

                    let times = stack.pop().unwrap_or("0".to_string()).parse().unwrap_or(0);
                    let mut result = String::with_capacity(value.len() * times);
                    for _ in 0..times {
                        result.push_str(&value);
                    }
                    stack.push(result);
                }
                _ => {}
            }
            i += 1;
        }

        let mut result = String::new();
        for x in stack.iter() {
            result.push_str(x);
        }
        result
    }
}
