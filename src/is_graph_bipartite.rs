use crate::Solution;

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut union: Vec<usize> = (0..graph.len()).collect();
        for i in 0..graph.len() {
            let boss_i = Self::find_boss(&union, i);
            for &j in graph[i].iter() {
                let j = j as usize;
                let boss_j = Self::find_boss(&union, j);
                if boss_i == boss_j {
                    return false;
                }
                union[boss_j] = Self::find_boss(&union, graph[i][0] as usize);
            }
        }
        true
    }

    fn find_boss(union: &Vec<usize>, mut target: usize) -> usize {
        loop {
            let boss = union[target];
            if boss == target {
                return boss;
            }
            target = boss;
        }
    }
}
