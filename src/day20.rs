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
fn bfs(grid: &[Vec<Square>], start: Point) -> Option<Vec<Vec<usize>>> {
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
        let mut out = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
        for (i, row) in out.iter_mut().enumerate() {
            for (j, val_out) in row.iter_mut().enumerate() {
                if let Some(val_in) = seen.get(&(i, j)) {
                    *val_out = steps - *val_in;
                }
            }
        }
        Some(out)
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
    if let Some(costs_from) = bfs(grid, start) {
        let mut out = 0;
        for (i, row) in costs_from.iter().enumerate() {
            for (j, &orig_cost) in row.iter().enumerate() {
                if orig_cost <= min_savings || orig_cost == usize::MAX {
                    continue;
                }
                let skips_i64 = skips_allowed as i64;
                for delta_i in -skips_i64..=skips_i64 {
                    for delta_j in (delta_i.abs() - skips_i64)..=(skips_i64 - delta_i.abs()) {
                        if let (Ok(new_i), Ok(new_j)) = (
                            TryInto::<usize>::try_into(i as i64 + delta_i),
                            TryInto::<usize>::try_into(j as i64 + delta_j),
                        ) {
                            let cost = new_i.abs_diff(i) + new_j.abs_diff(j);
                            if cost <= skips_allowed
                                && new_i < grid.len()
                                && new_j < grid[0].len()
                                && grid[new_i][new_j] != Square::Wall
                                && orig_cost >= costs_from[new_i][new_j] + cost + min_savings
                            {
                                out += 1;
                            }
                        }
                    }
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
