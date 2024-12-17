use aoc_2024::util;
// Use KeyedPriorityQueue because, unlike std::collections::BinaryHeap, it supports priority
// updates on arbitrary keys.
use keyed_priority_queue::KeyedPriorityQueue;
use std::cmp::{Ord, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};

/// Raw representation of the board as given.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum RawSquare {
    Wall,
    Empty,
    Start,
    End,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn neighbor(self, point: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (point.0 - 1, point.1),
            Direction::Down => (point.0 + 1, point.1),
            Direction::Left => (point.0, point.1 - 1),
            Direction::Right => (point.0, point.1 + 1),
        }
    }
    fn immediate_neighbors(self) -> [Self; 2] {
        match self {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        }
    }
    fn directions() -> [Self; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
    }
}

const TURN_COST: u64 = 1000;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct ProcessedSquareKey {
    dir: Direction,
    i: usize,
    j: usize,
}

/// A graph node, which is a ProcessedSquareKey, a list of neigbhbors, and the
/// costs to get to each neighbor.
/// Note that if a square in the raw input has two neighbors, up and down, that square will have
/// **two** Nodes in the graph, and each will neighbor the other with a cost of
/// TURN_COST (except for the End square, which will have costs of 0 to each of the other End
/// squares; one of which will arbitarily be the destination of dijkstra)
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Node {
    square_type: RawSquare,
    neighbors: Vec<(u64, ProcessedSquareKey)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProcessedGraph {
    root: ProcessedSquareKey,
    target: ProcessedSquareKey,
    grid: HashMap<ProcessedSquareKey, Node>,
}

// Given the raw board representation as provided in input, produce "virtual" nodes for each
// direction one might face and return the root node (i.e. Start + facing right)
fn produce_graph(raw_board: &[Vec<RawSquare>]) -> ProcessedGraph {
    let root = ProcessedSquareKey {
        dir: Direction::Right,
        i: raw_board.len() - 2,
        j: 1,
    };
    assert_eq!(raw_board[root.i][root.j], RawSquare::Start);
    let target = ProcessedSquareKey {
        dir: Direction::Right, // arbitrary
        i: 1,
        j: raw_board[0].len() - 2,
    };
    assert_eq!(raw_board[target.i][target.j], RawSquare::End);

    let mut grid = HashMap::new();
    for (i, row) in raw_board.iter().enumerate() {
        for (j, &square) in row.iter().enumerate() {
            match square {
                RawSquare::Wall => {}
                RawSquare::Empty | RawSquare::Start => {
                    for &d in Direction::directions().iter() {
                        let mut neighbors = Vec::new();
                        for &n in d.immediate_neighbors().iter() {
                            neighbors.push((TURN_COST, ProcessedSquareKey { dir: n, i, j }));
                        }
                        let (next_i, next_j) = d.neighbor((i, j));
                        if raw_board[next_i][next_j] != RawSquare::Wall {
                            neighbors.push((
                                1,
                                ProcessedSquareKey {
                                    dir: d,
                                    i: next_i,
                                    j: next_j,
                                },
                            ));
                        }
                        grid.insert(
                            ProcessedSquareKey { dir: d, i, j },
                            Node {
                                square_type: square,
                                neighbors,
                            },
                        );
                    }
                }
                RawSquare::End => {
                    for &d in Direction::directions().iter() {
                        let mut neighbors = Vec::new();
                        for &n in d.immediate_neighbors().iter() {
                            neighbors.push((0, ProcessedSquareKey { dir: n, i, j }));
                        }
                        grid.insert(
                            ProcessedSquareKey { dir: d, i, j },
                            Node {
                                square_type: square,
                                neighbors,
                            },
                        );
                    }
                }
            }
        }
    }
    ProcessedGraph { root, target, grid }
}

#[derive(Copy, Clone, Debug)]
struct MaxHeapEntry {
    cost: Option<u64>,
}

impl Ord for MaxHeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // This looks backwards, and that is deliberate -- KeyedPriorityQueue is a
        // **max** heap, so since we always want the smallest priority, reverse the order.
        match (self.cost, other.cost) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(sc), Some(oc)) => oc.cmp(&sc),
        }
    }
}

impl PartialOrd for MaxHeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MaxHeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for MaxHeapEntry {}

fn find_min_cost(graph: &ProcessedGraph) -> (u64, usize) {
    let mut heap = KeyedPriorityQueue::new();
    let mut best_dist = HashMap::new();
    let mut prev = HashMap::new();

    heap.push(graph.root, MaxHeapEntry { cost: Some(0) });
    best_dist.insert(graph.root, 0);

    for &v in graph.grid.keys() {
        if v != graph.root {
            heap.push(v, MaxHeapEntry { cost: None });
        }
    }

    while let Some((node, dist)) = heap.pop() {
        if dist.cost.is_none() {
            // Unreachable
            break;
        }
        let dist = dist.cost.unwrap();
        for &(cost, neigh) in graph.grid[&node].neighbors.iter() {
            let alt = dist + cost;
            if best_dist.get(&neigh).is_none_or(|&x| x > alt) {
                heap.set_priority(&neigh, MaxHeapEntry { cost: Some(alt) })
                    .unwrap();
                best_dist.insert(neigh, alt);
                prev.insert(neigh, vec![node]);
            } else if best_dist.get(&neigh).is_some_and(|&x| x == alt) {
                prev.get_mut(&neigh).unwrap().push(node);
            }
        }
    }
    let mut seen = HashSet::new();
    let mut possible_path_nodes = HashSet::new();
    possible_path_nodes.insert((graph.target.i, graph.target.j));
    let mut nodes = VecDeque::new();
    nodes.push_back(graph.target);
    while let Some(n) = nodes.pop_front() {
        if seen.contains(&n) {
            continue;
        }
        seen.insert(n);
        if let Some(prior) = prev.get(&n) {
            for &p in prior.iter() {
                nodes.push_back(p);
                possible_path_nodes.insert((p.i, p.j));
            }
        }
    }

    (best_dist[&graph.target], possible_path_nodes.len())
}

fn main() {
    let mut raw_board = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c {
                '#' => RawSquare::Wall,
                '.' => RawSquare::Empty,
                'E' => RawSquare::End,
                'S' => RawSquare::Start,
                _ => panic!("bad character {}", c),
            });
        }
        raw_board.push(row);
    }
    let graph = produce_graph(&raw_board);
    println!("{:?}", find_min_cost(&graph));
}
