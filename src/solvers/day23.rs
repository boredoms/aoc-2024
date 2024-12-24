use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    mem::swap,
};

pub fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();

        let s = graph.entry(a.to_string()).or_default();
        s.push(b.to_string());

        let s = graph.entry(b.to_string()).or_default();
        s.push(a.to_string());
    });

    graph
}

pub fn solve_part_one(input: &str) -> usize {
    let graph = parse(input);

    let mut wedges: HashMap<String, Vec<String>> = HashMap::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();

        if graph[a].len() < graph[b].len() || (graph[a].len() == graph[b].len() && a < b) {
            let s = wedges.entry(a.to_string()).or_default();
            s.push(b.to_string());
        } else {
            let s = wedges.entry(b.to_string()).or_default();
            s.push(a.to_string());
        }
    });

    let mut res = 0;

    for (node, edges) in wedges.iter() {
        let d = edges.len();

        for (i, u) in edges.iter().enumerate() {
            for v in edges.iter().skip(i) {
                // test if wedge is closed
                let es = graph.get(v).unwrap();

                if es.iter().position(|w| w == u).is_some() {
                    if node.chars().next().unwrap() == 't'
                        || u.chars().next().unwrap() == 't'
                        || v.chars().next().unwrap() == 't'
                    {
                        res += 1;
                    }
                }
            }
        }
    }

    res
}

fn bron_kerbosch(
    graph: &HashMap<String, Vec<String>>,
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    res: &mut Vec<String>,
) {
    if p.is_empty() && x.is_empty() {
        let mut clique_vec: Vec<String> = Vec::new();

        r.iter().for_each(|v| clique_vec.push(v.clone()));

        clique_vec.sort();

        res.push(clique_vec.join(","));
    }

    let pps = p.clone();

    for v in pps.iter() {
        let mut r = r.clone();
        r.insert(v.to_string());
        let s: HashSet<String> = HashSet::from_iter(graph.get(v).unwrap().iter().cloned());

        let xx = &x & &s;
        let pp = &p & &s;

        bron_kerbosch(graph, r, pp, xx, res);

        p.remove(v);
        x.insert(v.clone());
    }
}

pub fn solve_part_two(input: &str) -> usize {
    let graph = parse(input);
    let mut res = Vec::new();
    let p = HashSet::from_iter(graph.keys().cloned());

    bron_kerbosch(&graph, HashSet::new(), p, HashSet::new(), &mut res);

    println!("{}", res.iter().max_by_key(|clique| clique.len()).unwrap());

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(&std::fs::read_to_string("data/day23/input.txt").unwrap());
        assert_eq!(7, result);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(&std::fs::read_to_string("data/day23/input.txt").unwrap());
        assert_eq!(0, result);
    }
}
