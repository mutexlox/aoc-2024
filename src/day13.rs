use aoc_2024::util;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Case {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

const A_COST: u64 = 3;
const B_COST: u64 = 1;

// compute gcd(a, b) and return Some((gcd, x, y)) s.t. a*x + b*y == gcd
fn extended_gcd(a: i64, b: i64) -> (u64, i64, i64) {
    if a == 0 {
        return (b.try_into().unwrap(), 0, 1);
    }
    let (gcd, x, y) = extended_gcd(b % a, a);
    (gcd, y - (b / a) * x, x)
}

// Represents a **class** of solutions to an equation a * x + b * y = target
// where any solution with x = start_x + steps * x_step, y = start_y + steps * y_step
// will be a valid solution with non-negative integral x and y,
// (as long as steps <= max_steps)
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct EquationSolution {
    start_x: i64,
    start_y: i64,
    max_steps: i64,
    x_step: i64,
    y_step: i64,
}

// Find (x, y) such that x and y are valid for both a and b, if possible.
// If there is no such x or y, return None, or otherwise return the list of valid options.
fn find_intersection(a: &EquationSolution, b: &EquationSolution) -> Option<(i64, i64)> {
    // (x, y) such that:
    // x = a.start_x + m * a.x_step
    // y = a.start_y + m * a.y_step
    // x = b.start_x + n * b.x_step
    // y = b.start_y + n * b.y_step
    // for 0 <= m <= a.max_steps
    //     0 <= n <= b.max_steps
    //
    // a.start_x + m * a.x_step = b.start_x + n * b.x_step
    // a.start_y + m * a.y_step = b.start_y + n * b.y_step
    //
    // a.start_x + m * a.x_step - b.start_x - n * b.x_step = 0
    // a.start_y + m * a.y_step - b.start_y - n * b.y_step = 0
    //
    // (a.start_x - b.start_x) + m * a.x_step - n * b.x_step = 0
    // (a.start_y - b.start_y) + m * a.y_step - n * b.y_step = 0
    let a1 = a.x_step;
    let b1 = -b.x_step;
    let c1 = a.start_x - b.start_x;
    let a2 = a.y_step;
    let b2 = -b.y_step;
    let c2 = a.start_y - b.start_y;

    // Avoid div by zero.
    if a1 * b2 == a2 * b1 {
        return None;
    }

    let denom = a1 * b2 - a2 * b1;
    let m_num = b1 * c2 - b2 * c1;
    let n_num = c1 * a2 - c2 * a1;
    if m_num % denom != 0 || n_num % denom != 0 {
        return None;
    }
    let (m, n) = (m_num / denom, n_num / denom);
    if !(0..=a.max_steps).contains(&m) {
        return None;
    }
    if !(0..=b.max_steps).contains(&n) {
        return None;
    }
    Some((a.start_x + m * a.x_step, a.start_y + m * a.y_step))
}

// Tries to solve for x and y such that a * x + b * y = target. See EquationSolution struct
// for more docs.
fn solve_equation(a: u64, b: u64, target: u64) -> Option<EquationSolution> {
    let (gcd, a_out, b_out) = extended_gcd(a.try_into().unwrap(), b.try_into().unwrap());

    if target % gcd != 0 {
        return None;
    }
    let b_coeff: i64 = (b / gcd).try_into().unwrap();
    let a_coeff = (a / gcd).try_into().unwrap();
    let multiple: i64 = (target / gcd).try_into().unwrap();
    let x1: i64 = a_out * multiple;
    let y1: i64 = b_out * multiple;
    assert_eq!(a as i64 * x1 + b as i64 * y1, target as i64);

    // all solutions are of form: x = x1 - r * b_coeff and y = y1 + r * a_coeff
    // more specifically we need x >= 0, so r * b_coeff >= x1, so x1 / b_coeff <= r
    // similarly we need y >= 0 so (y1 + r * a_coeff >= 0, so r >= -y1/a_coeff)

    let min_r = -y1 / a_coeff + if y1 > 0 || y1 % a_coeff == 0 { 0 } else { 1 };
    let max_r = x1 / b_coeff;
    let max_steps = max_r - min_r + 1;
    if max_r < min_r {
        return None;
    }

    // Now we have a **class** of solutions to an equation a * x + b * y = target
    // where any solution with x = x1 - r * b_coeff, y = y1 + r * a_coeff
    // will be a valid solution with non-negative integral x and y,
    // for min_r <= r <= max_r.
    assert_eq!(
        a as i64 * (x1 - min_r * b_coeff) + b as i64 * (y1 + min_r * a_coeff),
        target as i64
    );
    assert_eq!(
        a as i64 * (x1 - max_r * b_coeff) + b as i64 * (y1 + max_r * a_coeff),
        target as i64
    );
    Some(EquationSolution {
        start_x: x1 - min_r * b_coeff,
        start_y: y1 + min_r * a_coeff,
        max_steps,
        x_step: -b_coeff,
        y_step: a_coeff,
    })
}

fn min_tokens_required_large_offset(case: &Case, offset: u64) -> Option<u64> {
    let prize = Point {
        x: case.prize.x + offset,
        y: case.prize.y + offset,
    };

    // Both must have solutions.
    let x_solution = solve_equation(case.button_a.x, case.button_b.x, prize.x)?;
    let y_solution = solve_equation(case.button_a.y, case.button_b.y, prize.y)?;
    let (a, b) = find_intersection(&x_solution, &y_solution)?;
    assert_eq!(
        a as u64 * case.button_a.x + b as u64 * case.button_b.x,
        prize.x
    );
    assert_eq!(
        a as u64 * case.button_a.y + b as u64 * case.button_b.y,
        prize.y
    );

    Some(A_COST * a as u64 + B_COST * b as u64)
}

fn min_tokens_required(cases: &[Case], offset: u64) -> u64 {
    cases
        .iter()
        .map(|c| min_tokens_required_large_offset(c, offset).unwrap_or(0))
        .sum()
}

fn main() {
    static BUTTON_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"Button (?<butt>A|B): X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap());
    static PRIZE_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap());
    let mut tmp_button_a = None;
    let mut tmp_button_b = None;
    let mut tmp_prize = None;

    let mut cases = Vec::new();

    for line in util::get_lines().map_while(Result::ok) {
        if let Some(caps) = BUTTON_RE.captures(&line) {
            let effect = Some(Point {
                x: caps["x"].parse::<u64>().unwrap(),
                y: caps["y"].parse::<u64>().unwrap(),
            });
            if &caps["butt"] == "A" {
                tmp_button_a = effect;
            } else {
                tmp_button_b = effect;
            }
        } else if let Some(caps) = PRIZE_RE.captures(&line) {
            tmp_prize = Some(Point {
                x: caps["x"].parse::<u64>().unwrap(),
                y: caps["y"].parse::<u64>().unwrap(),
            });
        }
        if let (Some(button_a), Some(button_b), Some(prize)) =
            (tmp_button_a, tmp_button_b, tmp_prize)
        {
            cases.push(Case {
                button_a,
                button_b,
                prize,
            });
            tmp_button_a = None;
            tmp_button_b = None;
            tmp_prize = None;
        }
    }

    println!("{}", min_tokens_required(&cases, 0));
    println!("{}", min_tokens_required(&cases, 10000000000000));
}
