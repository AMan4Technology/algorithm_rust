use crate::Solution;
use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut contain = false;
        for word in word_list.iter() {
            if word == &end_word {
                contain = true;
                break;
            }
        }
        if !contain {
            return Vec::new();
        }

        let mut prev: HashMap<&String, Vec<&String>> = HashMap::with_capacity(word_list.len());
        let mut visited = HashMap::with_capacity(word_list.len());
        let mut lens = HashMap::with_capacity(word_list.len());
        lens.insert(&begin_word, 0);

        let mut queue = VecDeque::with_capacity(1);
        queue.push_back(&begin_word);
        while let Some(curr) = queue.pop_front() {
            println!("{:?}", curr);
            if !visited.contains_key(curr) {
                let mut next = Vec::new();
                'word: for word in word_list.iter() {
                    let mut diff = 0;
                    for i in 0..curr.len() {
                        if curr[i..i + 1] != word[i..i + 1] {
                            if diff == 1 {
                                continue 'word;
                            }
                            diff = 1;
                        }
                    }
                    if diff == 1 {
                        next.push(word);
                    }
                }
                visited.insert(curr, next);
            }

            let &len = lens.get(curr).unwrap();
            for &n in visited.get(curr).unwrap().iter() {
                if let Some(len_n) = lens.get(n) {
                    if *len_n < len + 1 {
                        continue;
                    } else if *len_n == len + 1 {
                        prev.get_mut(n).unwrap().push(curr);
                        continue;
                    }
                }
                queue.push_back(n);
                lens.insert(n, len + 1);
                if let Some(p) = prev.get_mut(n) {
                    p.clear();
                    p.push(curr)
                } else {
                    prev.insert(n, vec![curr]);
                }
            }
        }

        println!("{:?}", prev);

        let mut result = Vec::new();
        Self::visit_2(&mut result, &prev, &end_word, &begin_word, end_word.clone());
        result
    }

    fn visit_2(
        result: &mut Vec<Vec<String>>,
        prev: &HashMap<&String, Vec<&String>>,
        curr: &String,
        begin: &String,
        path: String,
    ) {
        if curr == begin {
            result.push(path.split(',').map(|node| node.to_string()).collect());
            return;
        }
        if let Some(p) = prev.get(curr) {
            for &p in p.iter() {
                let mut path = path.clone();
                path.insert(0, ',');
                path.insert_str(0, p);
                Self::visit_2(result, prev, p, begin, path);
            }
        }
    }
}
