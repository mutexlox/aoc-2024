use aoc_2024::util;
use std::collections::{HashMap, HashSet};

fn score(trails: &[Vec<u32>]) -> (usize, usize) {
    let mut nines_reachable_from = vec![vec![HashSet::new(); trails[0].len()]; trails.len()];

    let mut distinct_trails = vec![vec![0; trails[0].len()]; trails.len()];

    let mut position_map = HashMap::new();
    for (i, row) in trails.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            position_map
                .entry(val)
                .or_insert_with(Vec::new)
                .push((i, j));
        }
    }
    for &(i, j) in &position_map[&9] {
        nines_reachable_from[i][j].insert((i, j));
        distinct_trails[i][j] = 1;
    }
    for height in (0..9).rev() {
        for &(i, j) in &position_map[&height] {
            let mut reachable = HashSet::new();
            let mut sum = 0;
            if i > 0 && trails[i - 1][j] == height + 1 {
                reachable.extend(&nines_reachable_from[i - 1][j]);
                sum += distinct_trails[i - 1][j];
            }
            if j > 0 && trails[i][j - 1] == height + 1 {
                reachable.extend(&nines_reachable_from[i][j - 1]);
                sum += distinct_trails[i][j - 1];
            }
            if i < trails.len() - 1 && trails[i + 1][j] == height + 1 {
                reachable.extend(&nines_reachable_from[i + 1][j]);
                sum += distinct_trails[i + 1][j];
            }
            if j < trails[0].len() - 1 && trails[i][j + 1] == height + 1 {
                reachable.extend(&nines_reachable_from[i][j + 1]);
                sum += distinct_trails[i][j + 1];
            }
            nines_reachable_from[i][j] = reachable;
            distinct_trails[i][j] = sum;
        }
    }
    (
        position_map[&0]
            .iter()
            .map(|&(i, j)| nines_reachable_from[i][j].len())
            .sum(),
        position_map[&0]
            .iter()
            .map(|&(i, j)| distinct_trails[i][j])
            .sum(),
    )
}

fn main() {
    let mut trails = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let row = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        trails.push(row);
    }
    println!("{:?}", score(&trails));
}
