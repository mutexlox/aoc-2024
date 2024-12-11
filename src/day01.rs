use aoc_2024::util;
use std::collections::HashMap;

fn sort_and_count(lists: &mut [Vec<i64>]) -> u64 {
    for list in lists.iter_mut() {
        list.sort();
    }
    lists[0]
        .iter()
        .zip(lists[1].iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

fn similarity_score(lists: &mut [Vec<i64>]) -> i64 {
    let mut map = HashMap::new();
    for n in lists[1].iter() {
        map.insert(n, map.get(n).unwrap_or(&0) + 1);
    }
    lists[0].iter().map(|n| map.get(n).unwrap_or(&0) * n).sum()
}

fn main() {
    let mut lists = [vec![], vec![]];
    for line in util::get_lines().map_while(Result::ok) {
        for (i, part) in line.split_ascii_whitespace().enumerate() {
            lists[i].push(
                part.parse::<i64>()
                    .unwrap_or_else(|_| panic!("bad int {}", part)),
            );
        }
    }
    println!("sum: {}", sort_and_count(&mut lists));
    println!("similarity: {}", similarity_score(&mut lists));
}
