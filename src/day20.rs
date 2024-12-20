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

// Finds a path from start to end, returning its length and, for each (i, j) on the path, the
// length of the path from that point.
fn bfs(grid: &[Vec<Square>], start: (usize, usize)) -> Option<HashMap<(usize, usize), usize>> {
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
    grid: &[Vec<Square>],
    start: (usize, usize),
) -> Option<usize> {
    if let Some(costs_from) = bfs(grid, start) {
        let mut out: usize = 0;
        for (&square, &cost) in costs_from.iter() {
            for d in Direction::directions() {
                out += d
                    .neighbor(square)
                    .and_then(|s| {
                        if s.0 < grid.len() && s.1 < grid[0].len() && grid[s.0][s.1] == Square::Wall
                        {
                            d.neighbor(s)
                        } else {
                            None
                        }
                    })
                    .and_then(|point| costs_from.get(&point))
                    .map(|c| c + min_savings + 2 <= cost)
                    .is_some_and(|b| b) as usize;
                out += d
                    .neighbor(square)
                    .and_then(|s| {
                        if s.0 < grid.len() && s.1 < grid[0].len() && grid[s.0][s.1] == Square::Wall
                        {
                            d.neighbor(s)
                        } else {
                            None
                        }
                    })
                    .and_then(|s| {
                        if s.0 < grid.len() && s.1 < grid[0].len() && grid[s.0][s.1] == Square::Wall
                        {
                            d.neighbor(s)
                        } else {
                            None
                        }
                    })
                    .and_then(|point| costs_from.get(&point))
                    .map(|c| c + min_savings + 3 <= cost)
                    .is_some_and(|b| b) as usize;
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
    println!("{:?}", count_cheats_at_least(100, &grid, start.unwrap()));
}
