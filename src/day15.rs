use aoc_2024::util;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Square {
    Wall,
    Empty,
    Box,
    BoxLeft,
    BoxRight,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Step {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<Square>], robot: (usize, usize)) {
    println!("Grid state:");
    for (i, row) in grid.iter().enumerate() {
        for (j, s) in row.iter().enumerate() {
            if (i, j) == robot {
                print!("@");
            } else {
                match s {
                    Square::Wall => print!("#"),
                    Square::Empty => print!("."),
                    Square::Box => print!("O"),
                    Square::BoxLeft => print!("["),
                    Square::BoxRight => print!("]"),
                }
            }
        }
        println!();
    }
    println!();
}

fn point_from_step(pos: (usize, usize), step: Step) -> (usize, usize) {
    match step {
        Step::Up => (pos.0 - 1, pos.1),
        Step::Down => (pos.0 + 1, pos.1),
        Step::Left => (pos.0, pos.1 - 1),
        Step::Right => (pos.0, pos.1 + 1),
    }
}

fn run_step(
    grid: &mut [Vec<Square>],
    start_poses: &[(usize, usize)],
    step: Step,
) -> Vec<(usize, usize)> {
    let attempted_locations: Vec<_> = start_poses
        .iter()
        .map(|&sp| point_from_step(sp, step))
        .collect();

    let mut squares_to_check = HashSet::new();
    for &location in attempted_locations.iter() {
        match (grid[location.0][location.1], step) {
            (Square::Empty, _) => {}
            (Square::Wall, _) => {
                // failed to move some so don't move any
                return start_poses.to_vec();
            }
            (Square::Box, _) => {
                squares_to_check.insert(location);
            }
            (Square::BoxLeft | Square::BoxRight, Step::Left | Step::Right) => {
                squares_to_check.insert(location);
            }
            (Square::BoxLeft, Step::Up | Step::Down) => {
                squares_to_check.insert(location);
                squares_to_check.insert((location.0, location.1 + 1));
            }
            (Square::BoxRight, Step::Up | Step::Down) => {
                squares_to_check.insert((location.0, location.1 - 1));
                squares_to_check.insert(location);
            }
        }
    }
    if squares_to_check.is_empty() {
        return attempted_locations;
    }
    let squares_to_check: Vec<_> = squares_to_check.iter().cloned().collect();
    let new_locations = run_step(grid, &squares_to_check, step);
    if new_locations == squares_to_check {
        // Couldn't move them!
        start_poses.to_vec()
    } else {
        for (old, new) in squares_to_check.iter().zip(new_locations.iter()) {
            assert_eq!(grid[new.0][new.1], Square::Empty);
            grid[new.0][new.1] = grid[old.0][old.1];
            grid[old.0][old.1] = Square::Empty;
        }
        attempted_locations
    }
}

fn apply_steps(
    grid: &mut [Vec<Square>],
    mut robot_pos: (usize, usize),
    steps: &[Step],
) -> (usize, usize) {
    for &step in steps.iter() {
        robot_pos = run_step(grid, &[robot_pos], step)[0];
    }
    robot_pos
}

fn apply_steps_and_score(
    grid: &mut [Vec<Square>],
    robot_pos: (usize, usize),
    steps: &[Step],
) -> usize {
    apply_steps(grid, robot_pos, steps);
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, &s)| {
                    if matches!(s, Square::Box | Square::BoxLeft) {
                        100 * i + j
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let mut grid = Vec::new();
    let mut steps = Vec::new();
    let mut robot = None;
    let mut reading_grid = true;
    for (i, line) in util::get_lines().map_while(Result::ok).enumerate() {
        if line.is_empty() {
            reading_grid = false;
            continue;
        }
        if reading_grid {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                row.push(match c {
                    '#' => Square::Wall,
                    '.' => Square::Empty,
                    'O' => Square::Box,
                    '@' => {
                        robot = Some((i, j));
                        Square::Empty
                    }
                    _ => {
                        panic!("invalid square {}", c);
                    }
                });
            }
            grid.push(row);
        } else {
            for c in line.chars() {
                steps.push(match c {
                    '<' => Step::Left,
                    'v' => Step::Down,
                    '>' => Step::Right,
                    '^' => Step::Up,
                    _ => {
                        panic!("invalid step {}", c);
                    }
                });
            }
        }
    }
    let mut large_board = grid
        .iter()
        .map(|row| {
            row.iter().fold(Vec::new(), |mut v, s| {
                match s {
                    Square::Wall => {
                        v.push(Square::Wall);
                        v.push(Square::Wall);
                    }
                    Square::Empty => {
                        v.push(Square::Empty);
                        v.push(Square::Empty);
                    }
                    Square::Box => {
                        v.push(Square::BoxLeft);
                        v.push(Square::BoxRight);
                    }
                    _ => panic!("large box in small board"),
                };
                v
            })
        })
        .collect::<Vec<_>>();
    let robot = robot.unwrap();
    let large_robot_position = (robot.0, robot.1 * 2);

    println!("{}", apply_steps_and_score(&mut grid, robot, &steps));

    println!(
        "{}",
        apply_steps_and_score(&mut large_board, large_robot_position, &steps)
    );
}
