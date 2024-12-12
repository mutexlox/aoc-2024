use aoc_2024::util;
use std::collections::HashMap;

fn count_rocks_after(rocks: &[u64], steps: usize) -> usize {
    let mut cache = HashMap::new();
    fn helper(cache: &mut HashMap<(u64, usize), usize>, n: u64, steps: usize) -> usize {
        if let Some(&res) = cache.get(&(n, steps)) {
            return res;
        }
        if steps == 0 {
            return 1;
        }
        let res = if n == 0 {
            helper(cache, 1, steps - 1)
        } else {
            let digits = n.ilog10() + 1;
            if digits % 2 == 0 {
                let divisor = 10_u64.pow(digits / 2);
                helper(cache, n / divisor, steps - 1) + helper(cache, n % divisor, steps - 1)
            } else {
                helper(cache, n * 2024, steps - 1)
            }
        };
        cache.insert((n, steps), res);
        res
    }

    rocks.iter().map(|&n| helper(&mut cache, n, steps)).sum()
}

fn main() {
    let input = util::get_all_input();
    let rocks = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    println!("{}", count_rocks_after(&rocks, 25));
    println!("{}", count_rocks_after(&rocks, 75));
}
