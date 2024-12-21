use aoc_2024::util;
use aoc_2024::util::Direction;
use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Square {
    Wall,
    Empty,
    Start,
    End,
}

type Point = (usize, usize);

// Finds a path from start to end, returning its length and, for each (i, j) on the path, the
// length of the path from that point.
fn bfs(grid: &[Vec<Square>], start: Point) -> Option<HashMap<Point, usize>> {
    let mut seen = HashMap::new();
    let mut queue = VecDeque::new();
    assert_eq!(grid[start.0][start.1], Square::Start);

    queue.push_back((start, 0));

    let mut total_steps = None;
    while let Some((point, steps)) = queue.pop_front() {
        if seen.contains_key(&point) {
            continue;
        }
        seen.insert(point, steps);
        if grid[point.0][point.1] == Square::End {
            total_steps = Some(steps);
            break;
        }
        for n in Direction::directions()
            .iter()
            .filter_map(|d| d.neighbor(point))
            .filter(|&(i, j)| {
                i < grid.len()
                    && j < grid[0].len()
                    && matches!(grid[i][j], Square::Empty | Square::End)
            })
        {
            queue.push_back((n, steps + 1));
        }
    }

    if let Some(steps) = total_steps {
        for (_, v) in seen.iter_mut() {
            *v = steps - *v;
        }
        Some(seen)
    } else {
        None
    }
}

fn count_cheats_at_least(
    min_savings: usize,
    skips_allowed: usize,
    grid: &[Vec<Square>],
    start: Point,
) -> Option<usize> {
    // Given a start point and steps remaining, return all points reachable with noclip in that
    // number of steps along with the minimum number of steps.
    fn helper(
        grid: &[Vec<Square>],
        cache: &mut HashMap<(Point, usize), HashMap<Point, usize>>,
        start: Point,
        remaining: usize,
    ) -> HashMap<Point, usize> {
        if let Some(s) = cache.get(&(start, remaining)) {
            return s.clone();
        }
        let mut s = HashMap::new();
        if remaining == 0 {
            if grid[start.0][start.1] != Square::Wall {
                s.insert(start, 0);
            }
            return s;
        }
        for d in Direction::directions() {
            if let Some(n) = d.neighbor(start) {
                if n.0 < grid.len() && n.1 < grid[0].len() {
                    for (&dest, &cost) in helper(grid, cache, n, remaining - 1).iter() {
                        let v = s.entry(dest).or_insert(cost + 1);
                        if *v > cost + 1 {
                            *v = cost + 1;
                        }
                    }
                    if grid[n.0][n.1] != Square::Wall {
                        s.insert(n, 1);
                    }
                }
            }
        }
        cache.insert((start, remaining), s.clone());
        s
    }

    if let Some(costs_from) = bfs(grid, start) {
        let mut out = 0;
        let mut cache = HashMap::new();
        for (&square, &orig_cost) in costs_from.iter() {
            if costs_from[&(square.0, square.1)] <= min_savings {
                continue;
            }
            for (&dest, &cost) in helper(grid, &mut cache, square, skips_allowed).iter() {
                if orig_cost >= costs_from[&(dest.0, dest.1)] + cost + min_savings {
                    out += 1;
                }
            }
        }
        Some(out)
    } else {
        None
    }
}

fn main() {
    let mut grid = Vec::new();
    let mut start = None;
    for (i, line) in util::get_lines().map_while(Result::ok).enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => Square::Wall,
                '.' => Square::Empty,
                'E' => Square::End,
                'S' => {
                    start = Some((i, j));
                    Square::Start
                }
                _ => panic!("bad character {}", c),
            });
        }
        grid.push(row);
    }
    println!("{:?}", count_cheats_at_least(100, 2, &grid, start.unwrap()));
    println!(
        "{:?}",
        count_cheats_at_least(100, 20, &grid, start.unwrap())
    );
}
