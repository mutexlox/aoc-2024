use aoc_2024::util;
use std::collections::{HashMap, HashSet};

fn find_largest_maximal_clique(graph: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    // Based on "BronKerbosch2" on https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
    fn bron_kerbosch(
        graph: &HashMap<String, HashSet<String>>,
        r: &mut HashSet<String>,
        p: &mut HashSet<String>,
        x: &mut HashSet<String>,
        out: &mut Option<HashSet<String>>,
    ) {
        if p.is_empty() && x.is_empty() {
            if out.as_ref().is_none_or(|o| o.len() < r.len()) {
                *out = Some(r.clone());
            }
            return;
        }
        let (_, pivot) = p
            .union(x)
            .map(|v| (graph[v].len(), v.to_string()))
            .max()
            .unwrap();
        while let Some(v) = p.difference(&graph[&pivot]).next().cloned() {
            r.insert(v.to_string());
            bron_kerbosch(
                graph,
                r,
                &mut (&*p & &graph[&v]),
                &mut (&*x & &graph[&v]),
                out,
            );
            r.remove(&v);
            p.remove(&v);
            x.insert(v.to_string());
        }
    }
    let mut out = None;
    let mut p = HashSet::from_iter(graph.keys().cloned());
    bron_kerbosch(
        graph,
        &mut HashSet::new(),
        &mut p,
        &mut HashSet::new(),
        &mut out,
    );
    out.unwrap()
}

// Count all fully-connected subgraphs of size 3 where at least one member starts with
// `prefix`.
fn size_3_cliques(graph: &HashMap<String, HashSet<String>>, prefix: char) -> HashSet<[String; 3]> {
    let mut components = HashSet::new();
    let mut nodes_in_components = HashMap::new();
    for (u, neighbors) in graph.iter() {
        if !u.starts_with(prefix) {
            continue;
        }
        for v in neighbors.iter() {
            let inter = graph[v].intersection(neighbors);
            for w in inter {
                let mut s = [u.clone(), v.clone(), w.clone()];
                nodes_in_components
                    .entry(u.clone())
                    .or_insert_with(HashSet::new)
                    .insert(v.clone());
                nodes_in_components
                    .entry(u.clone())
                    .or_insert_with(HashSet::new)
                    .insert(w.clone());

                nodes_in_components
                    .entry(v.clone())
                    .or_insert_with(HashSet::new)
                    .insert(u.clone());
                nodes_in_components
                    .entry(v.clone())
                    .or_insert_with(HashSet::new)
                    .insert(w.clone());

                nodes_in_components
                    .entry(w.clone())
                    .or_insert_with(HashSet::new)
                    .insert(u.clone());
                nodes_in_components
                    .entry(w.clone())
                    .or_insert_with(HashSet::new)
                    .insert(v.clone());
                s.sort();
                components.insert(s);
            }
        }
    }

    components
}

fn main() {
    let edges = util::get_lines().map_while(Result::ok).map(|s| {
        let mut it = s.split("-").map(|e| e.to_string());
        (it.next().unwrap(), it.next().unwrap())
    });
    let mut graph = HashMap::new();
    for (u, v) in edges {
        graph
            .entry(u.clone())
            .or_insert_with(HashSet::new)
            .insert(v.clone());
        graph.entry(v).or_insert_with(HashSet::new).insert(u);
    }
    let cliques = size_3_cliques(&graph, 't');
    println!("{}", cliques.len());
    let largest = find_largest_maximal_clique(&graph);
    let mut v = largest.iter().cloned().collect::<Vec<_>>();
    v.sort();
    println!("{}", v.join(","));
}
