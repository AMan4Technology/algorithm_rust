use crate::Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        if equations.is_empty() {
            return true;
        }

        fn find(parent: &Vec<usize>, i: usize) -> usize {
            let mut result = i;
            while parent[result] != result {
                result = parent[result];
            }
            result
        }

        let mut parent = vec![0; 26];
        for i in 1..26 {
            parent[i] = i;
        }

        for equation in equations.iter() {
            let equation = equation.as_bytes();
            match equation[1] {
                b'=' => {
                    let a = find(&parent, (equation[0] - b'a') as usize);
                    let b = find(&parent, (equation[3] - b'a') as usize);
                    parent[a] = b;
                }
                b'!' => {
                    if find(&parent, (equation[0] - b'a') as usize)
                        == find(&parent, (equation[3] - b'a') as usize)
                    {
                        return false;
                    }
                }
                _ => {
                    continue;
                }
            }
        }

        for equation in equations.iter() {
            let equation = equation.as_bytes();
            match equation[1] {
                b'!' => {
                    if find(&parent, (equation[0] - b'a') as usize)
                        == find(&parent, (equation[3] - b'a') as usize)
                    {
                        return false;
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        true
    }
}
