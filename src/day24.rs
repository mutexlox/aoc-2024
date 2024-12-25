use aoc_2024::util;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::LazyLock;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Op {
    Or,
    And,
    Xor,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Gate {
    op: Op,
    lhs: String,
    rhs: String,
    out: String,
}

#[derive(Clone, Debug)]
struct Graph {
    graph: HashMap<String, HashSet<String>>,
    inputs: HashMap<String, bool>,
    reverse_graph: HashMap<String, HashSet<String>>,
}

/// topologically sort the given graph with given reverse edges and start nodes
/// return None if there's a cycle, otherwise a correct order
fn topo_sort(graph: &Graph) -> Option<Vec<String>> {
    let mut actual_graph = graph.graph.clone();
    let mut reverse_graph = graph.reverse_graph.clone();

    let mut sorted = Vec::new();
    let mut no_incoming = VecDeque::from_iter(graph.inputs.keys().cloned());
    while let Some(n) = no_incoming.pop_front() {
        sorted.push(n.clone());
        for m in actual_graph
            .get_mut(&n)
            .map(|s| s.drain())
            .into_iter()
            .flatten()
        {
            let s = reverse_graph.get_mut(&m).unwrap();
            s.remove(&n);
            if s.is_empty() {
                no_incoming.push_back(m.to_string());
            }
        }
    }

    for neighs in actual_graph.values() {
        if !neighs.is_empty() {
            return None;
        }
    }

    Some(sorted)
}

/// given a valid ordering of nodes, input values, and a mapping from output wires to gates,
/// return the z value from running the gates
fn evaluate(
    sorted: &[String],
    inputs: &HashMap<String, bool>,
    gates: &HashMap<String, Gate>,
) -> u64 {
    let mut values = inputs.clone();

    let mut res = 0;

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
                if val {
                    res |= 1 << rest.parse::<u32>().unwrap();
                }
            }
        }
    }
    res
}

/// return a list of possible swaps, assuming that wiring swaps will be "local" -- that is,
/// if we find that a wire for z23 is suspicious and one for z24 is suspicious, we should swap
/// those rather than ones further apart.
///
/// output format is a list of all potential swap 'sets'.
fn get_possible_swaps(gates: &HashMap<String, Gate>) -> Vec<Vec<(String, String)>> {
    /*
     * for output zXX, 2 <= XX < 45
     * zXX = XOR(A, B)
     * where B = XOR(xXX, yXX)
     *       A = OR(C, D)
     *         where C = AND(x(XX-1), y(XX - 1A)
     *               D = AND(E, F)
     */
    let mut swappable = Vec::new();
    for i in 2..45 {
        let mut local_swaps = HashSet::new();

        let z = format!("z{:02}", i);
        let z_gate = &gates[&z];
        if z_gate.op != Op::Xor {
            local_swaps.insert(z.clone());
        }
        // One of these should be XOR and one should be OR; doesn't matter which
        let mut z_input_lhs = z_gate.lhs.clone();
        let mut z_input_rhs = z_gate.rhs.clone();
        if !gates.contains_key(&z_input_lhs) || !gates.contains_key(&z_input_rhs) {
            local_swaps.insert(z.clone());
            swappable.push(local_swaps);
            continue;
        }
        // Swap so that lhs sholuld be Or and Rhs should be Xor
        if gates[&z_input_lhs].op > gates[&z_input_rhs].op {
            (z_input_lhs, z_input_rhs) = (z_input_rhs, z_input_lhs);
        }
        let lhs_gate = &gates[&z_input_lhs];
        let rhs_gate = &gates[&z_input_rhs];
        if lhs_gate.op != Op::Or {
            local_swaps.insert(z_input_lhs.clone());
        }
        if rhs_gate.op != Op::Xor {
            local_swaps.insert(z_input_rhs.clone());
        }
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        if (rhs_gate.lhs != x || rhs_gate.rhs != y) && (rhs_gate.lhs != y || rhs_gate.rhs != x) {
            local_swaps.insert(z_input_rhs.clone());
        }

        if let Some(g) = gates.get(&lhs_gate.lhs) {
            if g.op != Op::And {
                local_swaps.insert(lhs_gate.lhs.clone());
            }
        } else {
            local_swaps.insert(lhs_gate.out.clone());
        }
        if let Some(g) = gates.get(&lhs_gate.rhs) {
            if g.op != Op::And {
                local_swaps.insert(lhs_gate.rhs.clone());
            }
        } else {
            local_swaps.insert(lhs_gate.out.clone());
        }
        swappable.push(local_swaps);
    }
    let mut possible_swaps = Vec::new();
    for (i, swaps) in swappable.iter().enumerate().skip(1) {
        if swaps.is_empty() || swappable[i - 1].is_empty() {
            continue;
        }
        possible_swaps.push(
            swaps
                .iter()
                .cloned()
                .cartesian_product(swappable[i - 1].iter().cloned())
                .collect::<Vec<_>>(),
        );
    }
    let swap_sets = possible_swaps
        .iter()
        .map(|v| v.iter().cloned())
        .multi_cartesian_product();
    swap_sets.collect::<Vec<_>>()
}

/// Given a list of possible swaps, evaluate them all and find the first one that provides
/// the sum of the given inputs.
fn isolate_swaps(
    swap_sets: &[Vec<(String, String)>],
    graph: &Graph,
    gates: &HashMap<String, Gate>,
    in_x: u64,
    in_y: u64,
    depth: usize,
) -> Option<Vec<(String, String)>> {
    let mut working = Vec::new();
    for swap_set in swap_sets {
        let mut modified_graph = graph.clone();
        let mut modified_gates = gates.clone();

        for swap in swap_set.iter() {
            modified_gates.insert(
                swap.0.clone(),
                Gate {
                    op: gates[&swap.1].op,
                    lhs: gates[&swap.1].lhs.clone(),
                    rhs: gates[&swap.1].rhs.clone(),
                    out: swap.0.clone(),
                },
            );
            modified_gates.insert(
                swap.1.clone(),
                Gate {
                    op: gates[&swap.0].op,
                    lhs: gates[&swap.0].lhs.clone(),
                    rhs: gates[&swap.0].rhs.clone(),
                    out: swap.1.clone(),
                },
            );

            modified_graph
                .reverse_graph
                .insert(swap.0.clone(), graph.reverse_graph[&swap.1].clone());
            modified_graph
                .reverse_graph
                .insert(swap.1.clone(), graph.reverse_graph[&swap.0].clone());

            let g = &gates[&swap.0];
            if let Some(s) = modified_graph.graph.get_mut(&g.lhs) {
                if !s.contains(&swap.1) {
                    s.remove(&swap.0);
                    s.insert(swap.1.clone());
                }
            }
            if let Some(s) = modified_graph.graph.get_mut(&g.rhs) {
                if !s.contains(&swap.1) {
                    s.remove(&swap.0);
                    s.insert(swap.1.clone());
                }
            }

            let g = &gates[&swap.1];
            if let Some(s) = modified_graph.graph.get_mut(&g.lhs) {
                if !s.contains(&swap.0) {
                    s.remove(&swap.1);
                    s.insert(swap.0.clone());
                }
            }
            if let Some(s) = modified_graph.graph.get_mut(&g.rhs) {
                if !s.contains(&swap.0) {
                    s.remove(&swap.1);
                    s.insert(swap.0.clone());
                }
            }
        }

        if let Some(sorted) = topo_sort(&modified_graph) {
            if evaluate(&sorted, &graph.inputs, &modified_gates) == in_x + in_y {
                working.push(swap_set.clone());
            }
        }
    }
    if !working.is_empty() {
        if working.len() > 1 {
            // Try different inputs
            let mut new_inputs = graph.inputs.clone();
            let (new_x, new_y) = if depth == 0 {
                for (k, v) in new_inputs.iter_mut() {
                    if k.starts_with('x') {
                        *v = !*v;
                    }
                }
                let new_x = !in_x & ((1 << 45) - 1);
                (new_x, in_y)
            } else if depth == 1 {
                for v in new_inputs.values_mut() {
                    *v = !*v;
                }
                let new_x = !in_x & ((1 << 45) - 1);
                let new_y = !in_y & ((1 << 45) - 1);
                (new_x, new_y)
            } else {
                panic!("too deep");
            };
            isolate_swaps(
                &working,
                &Graph {
                    graph: graph.graph.clone(),
                    inputs: new_inputs,
                    reverse_graph: graph.reverse_graph.clone(),
                },
                gates,
                new_x,
                new_y,
                depth + 1,
            )
        } else {
            Some(working[0].clone())
        }
    } else {
        None
    }
}

fn main() {
    let mut inputs = HashMap::new();
    let mut gates = Vec::new();
    let mut seen_empty = false;

    let mut x_val: u64 = 0;
    let mut y_val: u64 = 0;

    static GATE_RE: LazyLock<Regex> = LazyLock::new(|| {
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
            if val {
                if let Some(rest) = name.strip_prefix('x') {
                    x_val |= 1 << rest.parse::<u32>().unwrap();
                } else if let Some(rest) = name.strip_prefix('y') {
                    y_val |= 1 << rest.parse::<u32>().unwrap();
                }
            }
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

    let g = Graph {
        graph,
        inputs: inputs.clone(),
        reverse_graph,
    };
    let sorted = topo_sort(&g).unwrap();
    let actual = evaluate(&sorted, &inputs, &gates_by_out);
    println!("{}", actual);

    let swaps = get_possible_swaps(&gates_by_out);
    let pairs = isolate_swaps(&swaps, &g, &gates_by_out, x_val, y_val, 0).unwrap();
    let mut wire_list = pairs
        .iter()
        .flat_map(|p| vec![p.0.clone(), p.1.clone()])
        .collect::<Vec<_>>();
    wire_list.sort();
    println!("{}", wire_list.join(","));
}
