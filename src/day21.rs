use aoc_2024::util;
use aoc_2024::util::Direction;
use itertools::Itertools;
use std::collections::HashMap;

fn get_arrow_target(c: char, a_pos: (usize, usize)) -> (usize, usize) {
    /* +---+---+---+
     * | 7 | 8 | 9 |
     * +---+---+---+
     * | 4 | 5 | 6 |
     * +---+---+---+
     * | 1 | 2 | 3 |
     * +---+---+---+
     *     | 0 | A |
     *     +---+---+
     *
     *     +---+---+
     *     | ^ | A |
     * +---+---+---+
     * | < | v | > |
     * +---+---+---+
     */
    match c {
        '^' => (0, 1),
        'A' => a_pos,
        '<' => (1, 0),
        'v' => (1, 1),
        '>' => (1, 2),
        '9' => (0, 2),
        '8' => (0, 1),
        '7' => (0, 0),
        '6' => (1, 2),
        '5' => (1, 1),
        '4' => (1, 0),
        '3' => (2, 2),
        '2' => (2, 1),
        '1' => (2, 0),
        '0' => (3, 1),
        _ => panic!("bad target {}", c),
    }
}

fn compute_sequence(code: &str, indirections: usize) -> usize {
    // given a short code, e.g. instructions for one character of the input, find the length of the
    // shortest instructions to do it with `indirections` keypads in the way.
    fn expand(
        cache: &mut HashMap<(String, usize), usize>,
        code: &str,
        indirections: usize,
        avoid: (usize, usize),
        start: (usize, usize),
    ) -> usize {
        if indirections == 0 {
            return code.len();
        }
        if let Some(&cost) = cache.get(&(code.to_string(), indirections)) {
            return cost;
        }
        let mut total_len = 0;
        let mut pos = start;
        for c in code.chars() {
            let mut moves = Vec::new();

            let target = get_arrow_target(c, start);
            if target.1 > pos.1 {
                moves.extend(vec![Direction::Right; target.1 - pos.1]);
            } else if target.1 < pos.1 {
                moves.extend(vec![Direction::Left; pos.1 - target.1]);
            }
            if target.0 > pos.0 {
                moves.extend(vec![Direction::Down; target.0 - pos.0]);
            } else if target.0 < pos.0 {
                moves.extend(vec![Direction::Up; pos.0 - target.0]);
            }

            let mut min_len = None;
            //let mut min_len_s = String::new();
            'moves: for m in moves.iter().permutations(moves.len()).unique() {
                let mut cur = pos;

                // NEVER get to blank square.
                for d in m.iter() {
                    cur = d.neighbor(cur).unwrap();
                    if cur == avoid {
                        // This one's no good, chief
                        continue 'moves;
                    }
                }
                let s = m
                    .iter()
                    .map(|d| match d {
                        Direction::Up => "^",
                        Direction::Down => "v",
                        Direction::Left => "<",
                        Direction::Right => ">",
                    })
                    .collect::<String>();
                let cost = expand(
                    cache,
                    &(s.clone() + "A"),
                    indirections - 1,
                    /*avoid=*/ (0, 0),
                    /*start=*/ (0, 2),
                );
                if min_len.is_none_or(|len| len > cost) {
                    min_len = Some(cost);
                    //min_len_s = s + "A";
                }
            }
            pos = target;
            //println!("...for {} ({}) at {} indirections, {} is best", code, c, indirections, min_len_s);
            total_len += min_len.unwrap();
        }
        cache.insert((code.to_string(), indirections), total_len);

        total_len
    }

    let mut cache = HashMap::new();
    expand(&mut cache, code, indirections, (3, 0), (3, 2))
}

fn complexity(code: &str, indirections: usize) -> usize {
    let numeric_part = code.strip_suffix('A').unwrap().parse::<usize>().unwrap();
    let sequence_len = compute_sequence(code, indirections);
    println!(
        "for {}, {} * {}\n=========================\n",
        code, numeric_part, sequence_len
    );
    numeric_part * sequence_len
}

fn sum_complexities(codes: &[String], indirections: usize) -> usize {
    codes.iter().map(|s| complexity(s, indirections)).sum()
}

fn main() {
    let codes = util::get_lines().map_while(Result::ok).collect::<Vec<_>>();
    println!("{}", sum_complexities(&codes, 3));
    println!("{}", sum_complexities(&codes, 26));
}
