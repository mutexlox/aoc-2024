use aoc_2024::util;

fn is_satisfiable_add_mul(equation: &[i64]) -> bool {
    let (target, rest) = (equation[0], &equation[1..]);
    for bits in 0..2_usize.pow((rest.len() - 1) as u32) {
        let mut tmp = rest[0];
        for (i, val) in rest.iter().skip(1).enumerate() {
            if tmp > target {
                break;
            }
            if bits & (1 << i) != 0 {
                tmp *= val;
            } else {
                tmp += val;
            }
        }
        if tmp == target {
            return true;
        }
    }
    false
}

fn is_satisfiable_add_mul_concat(equation: &[i64]) -> bool {
    fn helper(target: i64, rest: &mut [i64]) -> bool {
        if let Some(&next) = rest.last() {
            let new_len = rest.len() - 1;
            if target >= next && helper(target - next, &mut rest[..new_len]) {
                return true;
            }
            if target % next == 0 && helper(target / next, &mut rest[..new_len]) {
                return true;
            }
            let multiplier = 10_i64.pow(next.ilog10() + 1);
            if target % multiplier == next && helper(target / multiplier, &mut rest[..new_len]) {
                return true;
            }
            false
        } else {
            // Empty
            return target == 0;
        }
    }
    let target = equation[0];
    let mut rest = equation[1..].to_vec();
    helper(target, &mut rest)
}

fn sum_satisfiable_equations(equations: &[Vec<i64>], f: fn(&[i64]) -> bool) -> i64 {
    equations.iter().filter(|e| f(e)).map(|e| e[0]).sum()
}

fn main() {
    let mut equations = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let mut sides = line.split(":");
        let lhs = sides.next().unwrap().parse::<i64>().unwrap();
        let rhs = sides
            .next()
            .unwrap()
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap());
        let mut equation = Vec::new();
        equation.push(lhs);
        equation.extend_from_slice(&rhs.collect::<Vec<_>>());
        equations.push(equation);
    }
    println!(
        "sum: {}",
        sum_satisfiable_equations(&equations, is_satisfiable_add_mul)
    );
    println!(
        "sum: {}",
        sum_satisfiable_equations(&equations, is_satisfiable_add_mul_concat)
    );
}
