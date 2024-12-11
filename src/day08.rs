use aoc_2024::util;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Square {
    Empty,
    Full(char),
}

fn find_and_count_antinodes(map: &[Vec<Square>], repeated: bool) -> usize {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let mut frequency_positions = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &square) in row.iter().enumerate() {
            if let Square::Full(c) = square {
                frequency_positions
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }
    }

    for positions in frequency_positions.values() {
        for (i, p1) in positions.iter().enumerate() {
            for p2 in positions.iter().skip(i + 1) {
                let x_diff = p2.0 as i64 - p1.0 as i64;
                let y_diff = p2.1 as i64 - p1.1 as i64;
                antinodes.insert(*p1);
                antinodes.insert(*p2);
                // check (p2.0 + x_diff, p2.1 + y_diff) and (p1.0 - x_diff, p1.1 - y_diff)
                let mut near_p2 = (p2.0 as i64 + x_diff, p2.1 as i64 + y_diff);
                while (0..map.len() as i64).contains(&near_p2.0)
                    && (0..map[0].len() as i64).contains(&near_p2.1)
                {
                    antinodes.insert((near_p2.0 as usize, near_p2.1 as usize));
                    if !repeated {
                        break;
                    }
                    near_p2 = (near_p2.0 + x_diff, near_p2.1 + y_diff)
                }
                let mut near_p1 = (p1.0 as i64 - x_diff, p1.1 as i64 - y_diff);
                while (0..map.len() as i64).contains(&near_p1.0)
                    && (0..map[0].len() as i64).contains(&near_p1.1)
                {
                    antinodes.insert((near_p1.0 as usize, near_p1.1 as usize));
                    if !repeated {
                        break;
                    }
                    near_p1 = (near_p1.0 - x_diff, near_p1.1 - y_diff);
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    let mut map = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(if c == '.' {
                Square::Empty
            } else {
                Square::Full(c)
            });
        }
        map.push(row);
    }
    println!("{}", find_and_count_antinodes(&map, false));
    println!("{}", find_and_count_antinodes(&map, true));
}
