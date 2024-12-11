use aoc_2024::util;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_tuple(self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

// Simulate, returning the set of visited squares if there was no loop.
fn simulate(board: &[Vec<Square>], guard_start: (usize, usize), loop_checker: &mut [u8]) -> bool {
    let mut direction = Direction::Up;
    let ptr = loop_checker.as_mut_ptr();
    unsafe {
        // Safety: Writing only |len| bytes to a vector with |len| u8s
        std::ptr::write_bytes(ptr, 0, loop_checker.len());
    }

    let (mut guard_i, mut guard_j) = guard_start;
    loop {
        if loop_checker[guard_i * board[0].len() + guard_j] & (1 << (direction as usize)) != 0 {
            return false;
        }
        loop_checker[guard_i * board[0].len() + guard_j] |= 1 << (direction as usize);
        if let (Some(next_guard_i), Some(next_guard_j)) = (
            add_checked(guard_i, direction.to_tuple().0),
            add_checked(guard_j, direction.to_tuple().1),
        ) {
            // break if we go out of bounds
            if next_guard_i >= board.len() || next_guard_j >= board[guard_i].len() {
                break;
            }
            // turn if needed
            if board[next_guard_i][next_guard_j] == Square::Full {
                // Don't commit this; instead turn
                direction = direction.next();
            } else {
                (guard_i, guard_j) = (next_guard_i, next_guard_j);
            }
        } else {
            // ... or if we go negative
            break;
        }
    }

    true
}

// Simulate, returning both the # positions visited in a successful run and the number of ways
// to cause a loop.
fn simulate_and_count_positions(
    board: &[Vec<Square>],
    guard_start: (usize, usize),
) -> (usize, usize) {
    let mut loop_checker: Vec<u8> = vec![0; board.len() * board[0].len()];

    simulate(board, guard_start, &mut loop_checker);

    let mut visited = Vec::new();
    for (i, &s) in loop_checker.iter().enumerate() {
        if s != 0 {
            visited.push((i / board[0].len(), i % board[0].len()));
        }
    }

    let mut modified_board = board.to_vec();
    let mut loop_count = 0;
    for &(i, j) in visited.iter() {
        modified_board[i][j] = Square::Full;
        if !simulate(&modified_board, guard_start, &mut loop_checker) {
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
