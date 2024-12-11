use aoc_2024::util;

#[derive(Debug, Copy, Clone)]
struct Transform {
    delta_i: i64,
    delta_j: i64,
}

impl Transform {
    // Shorthand
    const fn new(delta_i: i64, delta_j: i64) -> Self {
        Self { delta_i, delta_j }
    }
}

fn count_matches_at(haystack: &[Vec<char>], needle: &[char], i: usize, j: usize) -> usize {
    static TRANSFORMS: [Transform; 8] = [
        Transform::new(-1, -1),
        Transform::new(-1, 0),
        Transform::new(-1, 1),
        Transform::new(0, -1),
        Transform::new(0, 1),
        Transform::new(1, -1),
        Transform::new(1, 0),
        Transform::new(1, 1),
    ];
    if needle[0] != haystack[i][j] {
        return 0;
    }
    let mut matches = 0;
    for transform in TRANSFORMS.iter() {
        let extent_i: Result<usize, _> =
            (i as i64 + (needle.len() - 1) as i64 * transform.delta_i).try_into();
        match extent_i {
            Ok(extent) => {
                if extent >= haystack.len() {
                    continue;
                }
            }
            Err(_) => continue,
        }
        let extent_j: Result<usize, _> =
            (j as i64 + (needle.len() - 1) as i64 * transform.delta_j).try_into();
        match extent_j {
            Ok(extent) => {
                if extent >= haystack[0].len() {
                    continue;
                }
            }
            Err(_) => continue,
        }
        if needle.iter().enumerate().all(|(k, &c)| {
            // safe because of earlier check
            let new_i = (i as i64 + k as i64 * transform.delta_i) as usize;
            let new_j = (j as i64 + k as i64 * transform.delta_j) as usize;
            haystack[new_i][new_j] == c
        }) {
            matches += 1;
        }
    }
    matches
}

fn count_matches(haystack: &[Vec<char>], needle: &[char]) -> usize {
    let mut matches = 0;

    for (i, row) in haystack.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == needle[0] {
                let new_matches = count_matches_at(haystack, needle, i, j);
                matches += new_matches;
            }
        }
    }

    matches
}

fn is_x_match_at(haystack: &[Vec<char>], needle: &[char], i: usize, j: usize) -> bool {
    static TRANSFORM_SETS: [[Transform; 2]; 2] = [
        [Transform::new(-1, -1), Transform::new(1, 1)],
        [Transform::new(-1, 1), Transform::new(1, -1)],
    ];
    if needle[1] != haystack[i][j] {
        return false;
    }
    for transforms in TRANSFORM_SETS.iter() {
        let actual = [
            haystack[(transforms[0].delta_i + i as i64) as usize]
                [(transforms[0].delta_j + j as i64) as usize],
            haystack[(transforms[1].delta_i + i as i64) as usize]
                [(transforms[1].delta_j + j as i64) as usize],
        ];
        let expected = [needle[0], needle[2]];
        let expected_rev = [needle[2], needle[0]];

        if actual != expected && actual != expected_rev {
            return false;
        }
    }
    true
}

fn count_x_matches(haystack: &[Vec<char>], needle: &[char]) -> usize {
    let mut matches = 0;
    assert!(needle.len() == 3);

    // Skip first and last row; an X won't be centered there.
    for (i, row) in haystack.iter().enumerate().skip(1).take(haystack.len() - 2) {
        for (j, ch) in row.iter().enumerate().skip(1).take(row.len() - 2) {
            if *ch == needle[1] && is_x_match_at(haystack, needle, i, j) {
                matches += 1;
            }
        }
    }

    matches
}

fn main() {
    let mut haystack = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        haystack.push(line.chars().collect::<Vec<_>>());
    }
    println!(
        "matches: {}",
        count_matches(&haystack, &['X', 'M', 'A', 'S'])
    );
    println!(
        "X-shape matches: {}",
        count_x_matches(&haystack, &['M', 'A', 'S'])
    );
}
