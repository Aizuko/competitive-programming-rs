pub mod bellman_ford {
    pub fn shortest_path(graph: &Vec<Vec<(usize, i64)>>, start: usize, inf: i64) -> Option<Vec<i64>> {
        let n = graph.len();
        let mut dist = vec![inf; n];
        dist[start] = 0;
        for _ in 0..n {
            for v in 0..n {
                for e in &graph[v] {
                    let (to, cost) = *e;
                    if dist[v] == inf || dist[to] <= dist[v] + cost {
                        continue;
                    }
                    dist[to] = dist[v] + cost;
                }
            }
        }

        let mut negative = vec![false; n];
        for _ in 0..n {
            for v in 0..n {
                for e in &graph[v] {
                    let (to, cost) = *e;
                    if dist[v] == inf {
                        continue;
                    }
                    if dist[to] > dist[v] + cost {
                        dist[to] = dist[v] + cost;
                        negative[to] = true;
                    }
                    if negative[v] {
                        negative[to] = true;
                    }
                }
            }
        }

        for i in 0..n {
            if negative[i] {
                return None;
            }
        }
        return Some(dist);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std;
    use std::fs;
    use test_helper::TestCaseProducer;

    #[test]
    fn solve_grl_1_b() {
        let mut input = TestCaseProducer::new_from_directory("./assets/GRL_1_B/in/");
        let mut output = TestCaseProducer::new_from_directory("./assets/GRL_1_B/out/");

        while !input.is_empty() {
            let v: usize = input.next();
            let e: usize = input.next();
            let r: usize = input.next();

            let mut graph = vec![vec![]; v];

            for _ in 0..e {
                let s: usize = input.next();
                let t: usize = input.next();
                let d: i64 = input.next();
                graph[s].push((t, d));
            }

            let inf = std::i64::MAX;

            match bellman_ford::shortest_path(&graph, r, inf) {
                Some(dist) => {
                    for i in 0..v {
                        if dist[i] == inf {
                            let out: String = output.next();
                            assert_eq!(out, "INF");
                        } else {
                            let out: i64 = output.next();
                            assert_eq!(dist[i], out);
                        }
                    }
                }
                None => {
                    let out1: String = output.next();
                    let out2: String = output.next();
                    assert_eq!(out1, "NEGATIVE");
                    assert_eq!(out2, "CYCLE");
                }
            }
        }
    }
}