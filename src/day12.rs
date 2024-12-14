use aoc_2024::util;
use std::collections::{HashMap, HashSet, VecDeque};

fn sum_costs(garden: &[Vec<char>]) -> u64 {
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let mut cost = 0;
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
                }
                if next_i < garden.len() - 1 && garden[next_i + 1][next_j] == start {
                    queue.push_back((next_i + 1, next_j));
                } else {
                    perim += 1;
                }
                if next_j > 0 && garden[next_i][next_j - 1] == start {
                    queue.push_back((next_i, next_j - 1));
                } else {
                    perim += 1;
                }
                if next_j < garden[0].len() - 1 && garden[next_i][next_j + 1] == start {
                    queue.push_back((next_i, next_j + 1));
                } else {
                    perim += 1;
                }
            }

            cost += area * perim;
        }
    }
    cost
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

fn sum_bulk_costs(garden: &[Vec<char>]) -> u64 {
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let mut cost = 0;
    for (i, row) in garden.iter().enumerate() {
        for (j, &start) in row.iter().enumerate() {
            if visited[i][j] {
                continue;
            }
            // bfs from here for extent
            let mut queue = VecDeque::new();
            queue.push_back((i, j));
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
                    let key = (next_i, next_j, Direction::Up);
                    union_find.insert(key, (key, 1));

                    if next_j > 0 && union_find.contains_key(&(next_i, next_j - 1, Direction::Up)) {
                        union(&mut union_find, key, (next_i, next_j - 1, Direction::Up));
                    }

                    if union_find.contains_key(&(next_i, next_j + 1, Direction::Up)) {
                        union(&mut union_find, key, (next_i, next_j + 1, Direction::Up));
                    }
                }

                if next_i < garden.len() - 1 && garden[next_i + 1][next_j] == start {
                    queue.push_back((next_i + 1, next_j));
                } else {
                    let key = (next_i, next_j, Direction::Down);

                    union_find.insert(key, (key, 1));

                    if next_j > 0 && union_find.contains_key(&(next_i, next_j - 1, Direction::Down))
                    {
                        union(&mut union_find, key, (next_i, next_j - 1, Direction::Down));
                    }

                    if union_find.contains_key(&(next_i, next_j + 1, Direction::Down)) {
                        union(&mut union_find, key, (next_i, next_j + 1, Direction::Down));
                    }
                }

                if next_j > 0 && garden[next_i][next_j - 1] == start {
                    queue.push_back((next_i, next_j - 1));
                } else {
                    let key = (next_i, next_j, Direction::Left);

                    union_find.insert(key, (key, 1));

                    if next_i > 0 && union_find.contains_key(&(next_i - 1, next_j, Direction::Left))
                    {
                        union(&mut union_find, key, (next_i - 1, next_j, Direction::Left));
                    }

                    if union_find.contains_key(&(next_i + 1, next_j, Direction::Left)) {
                        union(&mut union_find, key, (next_i + 1, next_j, Direction::Left));
                    }
                }

                if next_j < garden[0].len() - 1 && garden[next_i][next_j + 1] == start {
                    queue.push_back((next_i, next_j + 1));
                } else {
                    let key = (next_i, next_j, Direction::Right);

                    union_find.insert(key, (key, 1));

                    if next_i > 0
                        && union_find.contains_key(&(next_i - 1, next_j, Direction::Right))
                    {
                        union(&mut union_find, key, (next_i - 1, next_j, Direction::Right));
                    }

                    if union_find.contains_key(&(next_i + 1, next_j, Direction::Right)) {
                        union(&mut union_find, key, (next_i + 1, next_j, Direction::Right));
                    }
                }
            }

            let mut roots = HashSet::new();
            let keys = union_find.keys().cloned().collect::<Vec<_>>();
            for &key in keys.iter() {
                roots.insert(find(&mut union_find, key));
            }

            let sides = roots.len() as u64;

            cost += area * sides;
        }
    }
    cost
}

fn main() {
    let mut garden = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        garden.push(line.chars().collect());
    }
    println!("{}", sum_costs(&garden));
    println!("{}", sum_bulk_costs(&garden));
}
