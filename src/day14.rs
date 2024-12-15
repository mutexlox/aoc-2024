use aoc_2024::util;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Robot {
    position: Point,
    velocity: Point,
}

const GRID_X_SIZE: i64 = 101;
const GRID_Y_SIZE: i64 = 103;

fn robot_position_after_time(secs: i64, robot: &Robot) -> Point {
    Point {
        x: (robot.position.x + secs * robot.velocity.x).rem_euclid(GRID_X_SIZE),
        y: (robot.position.y + secs * robot.velocity.y).rem_euclid(GRID_Y_SIZE),
    }
}

fn all_robot_positions_after_time(secs: i64, robots: &[Robot]) -> Vec<Point> {
    robots
        .iter()
        .map(|r| robot_position_after_time(secs, r))
        .collect()
}

fn score_after_secs(secs: i64, robots: &[Robot]) -> u64 {
    let positions = all_robot_positions_after_time(secs, robots);
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bot_left = 0;
    let mut bot_right = 0;

    let mid_x = GRID_X_SIZE / 2;
    let mid_y = GRID_Y_SIZE / 2;

    for &p in positions.iter() {
        match (p.x.cmp(&mid_x), p.y.cmp(&mid_y)) {
            (Ordering::Less, Ordering::Less) => top_left += 1,
            (Ordering::Less, Ordering::Greater) => top_right += 1,
            (Ordering::Greater, Ordering::Less) => bot_left += 1,
            (Ordering::Greater, Ordering::Greater) => bot_right += 1,
            (_, _) => {}
        }
    }

    top_left * top_right * bot_left * bot_right
}

const LONG_LINE_LENGTH: i64 = 20;

fn has_long_horizontal_line(points: &[Point]) -> bool {
    let points: HashSet<Point> = HashSet::from_iter(points.iter().cloned());
    for point in points.iter() {
        let mut count = 1;
        for delta in 1..LONG_LINE_LENGTH {
            if points.contains(&Point {
                x: point.x + delta,
                y: point.y,
            }) {
                count += 1;
            } else {
                break;
            }
        }
        for delta in (-LONG_LINE_LENGTH..-1).rev() {
            if points.contains(&Point {
                x: point.x + delta,
                y: point.y,
            }) {
                count += 1;
            } else {
                break;
            }
        }
        if count >= LONG_LINE_LENGTH {
            return true;
        }
    }
    false
}

const LONG_TIME: i64 = 10_000;

// Find any pictures with horizontal lines longer than 20 robots long,
// along with the time at which they appear.
fn find_long_horizontal_lines(robots: &[Robot]) -> Vec<(i64, Vec<Point>)> {
    (0..LONG_TIME)
        .filter_map(|secs| {
            let points = all_robot_positions_after_time(secs, robots);
            if has_long_horizontal_line(&points) {
                Some((secs, points))
            } else {
                None
            }
        })
        .collect()
}

fn pretty_print(secs: i64, points: &[Point]) {
    println!("After {} secs:", secs);
    let points: HashSet<Point> = HashSet::from_iter(points.iter().cloned());
    for i in 0..GRID_Y_SIZE {
        for j in 0..GRID_X_SIZE {
            if points.contains(&Point { x: j, y: i }) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!("\n\n");
}

fn find_and_print_candidates(robots: &[Robot]) {
    for (secs, points) in find_long_horizontal_lines(robots).iter() {
        pretty_print(*secs, points);
    }
}

fn main() {
    static ROBOT_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap());
    let mut robots = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        if let Some(caps) = ROBOT_RE.captures(&line) {
            robots.push(Robot {
                position: Point {
                    x: caps["px"].parse::<i64>().unwrap(),
                    y: caps["py"].parse::<i64>().unwrap(),
                },
                velocity: Point {
                    x: caps["vx"].parse::<i64>().unwrap(),
                    y: caps["vy"].parse::<i64>().unwrap(),
                },
            });
        }
    }
    println!("{}", score_after_secs(100, &robots));

    find_and_print_candidates(&robots);
}
