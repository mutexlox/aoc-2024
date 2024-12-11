use aoc_2024::util;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Square {
    Empty,
    Full,
}

fn add_checked(x: usize, y: i32) -> Option<usize> {
    if y < 0 {
        x.checked_sub((-y) as usize)
    } else {
        x.checked_add(y as usize)
    }
}

// Simulate, returning the set of visited squares if there was no loop.
fn simulate(board: &[Vec<Square>], guard_start: (usize, usize)) -> Option<HashSet<(usize, usize)>> {
    let mut visited = HashSet::new();
    visited.insert(guard_start);
    let mut direction = (-1, 0);
    let mut loop_checker = HashSet::new();
    let (mut guard_i, mut guard_j) = guard_start;
    loop {
        let key = ((guard_i, guard_j), direction);
        if loop_checker.contains(&key) {
            return None;
        }
        loop_checker.insert(key);
        if let (Some(next_guard_i), Some(next_guard_j)) = (
            add_checked(guard_i, direction.0),
            add_checked(guard_j, direction.1),
        ) {
            // break if we go out of bounds
            if next_guard_i >= board.len() || next_guard_j >= board[guard_i].len() {
                break;
            }
            // turn if needed
            if board[next_guard_i][next_guard_j] == Square::Full {
                // Don't commit this; instead turn
                // (-1, 0) -> (0, 1) -> (1, 0) -> (0, -1)
                direction = match direction {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => panic!("bad direction {:?}", direction),
                };
            } else {
                (guard_i, guard_j) = (next_guard_i, next_guard_j);
                visited.insert((guard_i, guard_j));
            }
        } else {
            // ... or if we go negative
            break;
        }
    }

    Some(visited)
}

// Simulate, returning both the # positions visited in a successful run and the number of ways
// to cause a loop.
fn simulate_and_count_positions(
    board: &[Vec<Square>],
    guard_start: (usize, usize),
) -> (usize, usize) {
    let visited = simulate(board, guard_start).unwrap();

    let mut modified_board = board.to_vec();
    let mut loop_count = 0;
    for &(i, j) in visited.iter() {
        modified_board[i][j] = Square::Full;
        if simulate(&modified_board, guard_start).is_none() {
            loop_count += 1;
        }
        modified_board[i][j] = Square::Empty;
    }

    (visited.len(), loop_count)
}

fn main() {
    let mut board = Vec::new();
    let mut guard_pos = (None, None);
    for (i, line) in util::get_lines().map_while(Result::ok).enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let square = match c {
                '#' => Square::Full,
                '.' => Square::Empty,
                '^' => {
                    guard_pos = (Some(i), Some(j));
                    Square::Empty
                }
                _ => panic!("invalid char {}", c),
            };
            row.push(square);
        }
        board.push(row);
    }
    let guard_pos = (guard_pos.0.unwrap(), guard_pos.1.unwrap());
    println!(
        "visited, ways to loop {:?}",
        simulate_and_count_positions(&board, guard_pos)
    );
}
