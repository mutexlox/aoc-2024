use aoc_2024::util;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Gate {
    op: Op,
    lhs: String,
    rhs: String,
    out: String,
}

fn evaluate(
    graph: &HashMap<String, HashSet<String>>,
    inputs: &HashMap<String, bool>,
    gates: &HashMap<String, Gate>,
    reverse_graph: &HashMap<String, HashSet<String>>,
) -> u64 {
    let mut graph = graph.clone();
    let mut reverse_graph = reverse_graph.clone();
    // First, do a topological sort
    let mut sorted = Vec::new();
    let mut no_incoming = VecDeque::from_iter(inputs.keys().cloned());
    while let Some(n) = no_incoming.pop_front() {
        sorted.push(n.clone());
        for m in graph.get_mut(&n).map(|s| s.drain()).into_iter().flatten() {
            let s = reverse_graph.get_mut(&m).unwrap();
            s.remove(&n);
            if s.is_empty() {
                no_incoming.push_back(m.to_string());
            }
        }
    }
    let mut values = inputs.clone();
    let mut z_outs = HashMap::new();
    for out in sorted.iter() {
        if let Some(gate) = gates.get(out) {
            let lhs = values[&gate.lhs];
            let rhs = values[&gate.rhs];
            let val = match gate.op {
                Op::And => lhs && rhs,
                Op::Or => lhs || rhs,
                Op::Xor => lhs != rhs,
            };
            values.insert(gate.out.clone(), val);
            if let Some(rest) = gate.out.strip_prefix('z') {
                z_outs.insert(rest.parse::<u32>().unwrap(), val);
            }
        }
    }
    let mut res = 0;
    let mut i = 0;
    while let Some(&val) = z_outs.get(&i) {
        if val {
            res |= 1 << i;
        }
        i += 1;
    }
    res
}

fn main() {
    let mut inputs = HashMap::new();
    let mut gates = Vec::new();
    let mut seen_empty = false;
    static GATE_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?<lhs>...) (?<op>OR|XOR|AND) (?<rhs>...) -> (?<out>...)").unwrap()
    });
    for line in util::get_lines().map_while(Result::ok) {
        if line.is_empty() {
            seen_empty = true;
            continue;
        }
        if !seen_empty {
            let mut parts = line.split(':');
            let name = parts.next().unwrap().to_string();
            let val = parts.next().unwrap().trim() == "1";
            inputs.insert(name, val);
        } else if let Some(caps) = GATE_RE.captures(&line) {
            let lhs = caps["lhs"].to_string();
            let op = match &caps["op"] {
                "OR" => Op::Or,
                "AND" => Op::And,
                "XOR" => Op::Xor,
                _ => panic!("bad op {}", &caps["op"]),
            };
            let rhs = caps["rhs"].to_string();
            let out = caps["out"].to_string();
            gates.push(Gate { op, lhs, rhs, out });
        }
    }
    // Create a graph with edges from inputs to outputs so we can topo-sort.
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    // Map outputs to gates so that, once we have a topo sort, we can determine which gate to do
    // in which order. (Earliest thing in topo sort first.)
    let mut gates_by_out = HashMap::new();
    let mut reverse_graph = HashMap::new();
    for gate in gates.iter() {
        graph
            .entry(gate.lhs.clone())
            .or_default()
            .insert(gate.out.clone());
        graph
            .entry(gate.rhs.clone())
            .or_default()
            .insert(gate.out.clone());

        let s: &mut HashSet<String> = reverse_graph.entry(gate.out.clone()).or_default();
        s.insert(gate.lhs.clone());
        s.insert(gate.rhs.clone());

        gates_by_out.insert(gate.out.clone(), gate.clone());
    }
    println!(
        "{}",
        evaluate(&graph, &inputs, &gates_by_out, &reverse_graph)
    );
}
