use aoc_2024::util;
use std::collections::{HashMap, HashSet, VecDeque};

fn sum_costs(garden: &[Vec<char>]) -> (u64, u64) {
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let mut cost = 0;
    let mut bulk_cost = 0;

    for (i, row) in garden.iter().enumerate() {
        for (j, &start) in row.iter().enumerate() {
            if visited[i][j] {
                continue;
            }
            // bfs from here for extent
            let mut queue = VecDeque::new();
            queue.push_back((i, j));
            let mut perim = 0;
            let mut area = 0;

            let mut union_find = HashMap::new();

            while let Some((next_i, next_j)) = queue.pop_front() {
                if visited[next_i][next_j] {
                    continue;
                }
                visited[next_i][next_j] = true;

                area += 1;

                if next_i > 0 && garden[next_i - 1][next_j] == start {
                    queue.push_back((next_i - 1, next_j));
                } else {
                    perim += 1;

                    let key = (next_i, next_j, Direction::Up);
                    add_and_maybe_union_neighbors(
                        &mut union_find,
                        key,
                        next_j.checked_sub(1).map(|j| (next_i, j)),
                        (next_i, next_j + 1),
                    );
                }

                if next_i < garden.len() - 1 && garden[next_i + 1][next_j] == start {
                    queue.push_back((next_i + 1, next_j));
                } else {
                    perim += 1;

                    let key = (next_i, next_j, Direction::Down);
                    add_and_maybe_union_neighbors(
                        &mut union_find,
                        key,
                        next_j.checked_sub(1).map(|j| (next_i, j)),
                        (next_i, next_j + 1),
                    );
                }

                if next_j > 0 && garden[next_i][next_j - 1] == start {
                    queue.push_back((next_i, next_j - 1));
                } else {
                    perim += 1;

                    let key = (next_i, next_j, Direction::Left);
                    add_and_maybe_union_neighbors(
                        &mut union_find,
                        key,
                        next_i.checked_sub(1).map(|i| (i, next_j)),
                        (next_i + 1, next_j),
                    );
                }

                if next_j < garden[0].len() - 1 && garden[next_i][next_j + 1] == start {
                    queue.push_back((next_i, next_j + 1));
                } else {
                    perim += 1;

                    let key = (next_i, next_j, Direction::Right);
                    add_and_maybe_union_neighbors(
                        &mut union_find,
                        key,
                        next_i.checked_sub(1).map(|i| (i, next_j)),
                        (next_i + 1, next_j),
                    );
                }
            }

            cost += area * perim;

            let mut roots = HashSet::new();
            let keys = union_find.keys().cloned().collect::<Vec<_>>();
            for &key in keys.iter() {
                roots.insert(find(&mut union_find, key));
            }

            bulk_cost += area * (roots.len() as u64);
        }
    }
    (cost, bulk_cost)
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type HashKey = (usize, usize, Direction);

fn find(union_find: &mut HashMap<HashKey, (HashKey, u64)>, mut a: HashKey) -> HashKey {
    let mut root = a;
    while union_find[&root].0 != root {
        root = union_find[&root].0;
    }

    while union_find[&a].0 != root {
        let parent = union_find[&a].0;
        union_find.insert(a, (root, union_find[&a].1));
        a = parent;
    }
    root
}

fn union(union_find: &mut HashMap<HashKey, (HashKey, u64)>, a: HashKey, b: HashKey) {
    let a = find(union_find, a);
    let b = find(union_find, b);
    if a == b {
        return;
    }

    let a_size = union_find[&a].1;
    let b_size = union_find[&b].1;
    if a_size < b_size {
        union_find.insert(b, (a, b_size));
        union_find.insert(a, (union_find[&a].0, a_size + b_size));
    } else {
        union_find.insert(a, (b, a_size));
        union_find.insert(b, (union_find[&b].0, a_size + b_size));
    }
}

fn add_and_maybe_union_neighbors(
    union_find: &mut HashMap<HashKey, (HashKey, u64)>,
    key: HashKey,
    small_neighbor: Option<(usize, usize)>,
    large_neighbor: (usize, usize),
) {
    union_find.insert(key, (key, 1));

    let direction = key.2;

    if let Some((i, j)) = small_neighbor {
        if union_find.contains_key(&(i, j, direction)) {
            union(union_find, key, (i, j, direction));
        }
    }

    if union_find.contains_key(&(large_neighbor.0, large_neighbor.1, direction)) {
        union(
            union_find,
            key,
            (large_neighbor.0, large_neighbor.1, direction),
        );
    }
}

fn main() {
    let mut garden = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        garden.push(line.chars().collect());
    }
    println!("{:?}", sum_costs(&garden));
}
