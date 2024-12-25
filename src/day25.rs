use aoc_2024::util;

fn fits(lock: &[usize], key: &[usize]) -> bool {
    lock.iter().zip(key.iter()).all(|(a, b)| a + b <= 5)
}

fn count_fitting_pairs(locks: &[Vec<usize>], keys: &[Vec<usize>]) -> usize {
    let mut count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if fits(lock, key) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut input = Vec::new();
    let mut cur = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        if line.is_empty() {
            input.push(cur);
            cur = Vec::new();
            continue;
        }
        cur.push(line.chars().collect::<Vec<_>>());
    }
    input.push(cur);

    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for v in input.iter() {
        if v[0][0] == '#' {
            let mut lock = Vec::new();
            for j in 0..v[0].len() {
                for (i, row) in v.iter().enumerate() {
                    if row[j] != '#' {
                        lock.push(i - 1);
                        break;
                    }
                }
            }
            locks.push(lock);
        } else {
            let mut key = Vec::new();
            for j in 0..v[0].len() {
                for i in (0..v.len()).rev() {
                    if v[i][j] != '#' {
                        key.push(v.len() - i - 2);
                        break;
                    }
                }
            }
            keys.push(key);
        }
    }
    println!("{}", count_fitting_pairs(&locks, &keys));
}
