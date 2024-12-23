use aoc_2024::util;
use std::collections::{HashMap, VecDeque};

const MODULO: i64 = 16_777_216;

fn prng_step(x: i64) -> i64 {
    let x = ((x << 6) ^ x) & (MODULO - 1);
    let x = ((x >> 5) ^ x) & (MODULO - 1);
    ((x << 11) ^ x) & (MODULO - 1)
}

// Assumes each element x is -9 <= x <= 9
fn hash_key(arr: [i64; 4]) -> i64 {
    // range 0 - 18; 5 bits
    (arr[0] + 9) | (arr[1] + 9) << 5 | (arr[2] + 9) << 10 | (arr[3] + 9) << 15
}

// map is a map of last 4 changes -> sum of what you can get
fn run_steps(map: &mut HashMap<i64, i64>, mut x: i64, steps: usize) -> i64 {
    let mut last_n = VecDeque::new();
    let mut map_local = HashMap::new();
    for _ in 0..steps {
        let next = prng_step(x);
        if last_n.len() == 4 {
            last_n.pop_front();
        }
        last_n.push_back(next % 10 - x % 10);
        x = next;
        if last_n.len() == 4 {
            let to_add = [last_n[0], last_n[1], last_n[2], last_n[3]];
            map_local.entry(hash_key(to_add)).or_insert(next % 10);
        }
    }
    for (&k, &v) in map_local.iter() {
        *map.entry(k).or_default() += v;
    }
    x
}

fn run_steps_and_sum(seeds: &[i64], steps: usize) -> (i64, i64) {
    let mut map = HashMap::new();
    let sum = seeds.iter().map(|&x| run_steps(&mut map, x, steps)).sum();
    (sum, *map.values().max().unwrap())
}

fn main() {
    let seeds = util::get_lines()
        .map_while(Result::ok)
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("{:?}", run_steps_and_sum(&seeds, 2000));
}
