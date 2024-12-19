use aoc_2024::util;
use aoc_2024::util::Direction;
use std::collections::{HashSet, VecDeque};

const LINES_TO_READ: usize = 1024;
const GRID_SIZE: usize = 71;

fn steps_to_end(points: &[(usize, usize)]) -> Option<usize> {
    let start = (0, 0);
    let goal = (GRID_SIZE - 1, GRID_SIZE - 1);

    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    let mut visited = HashSet::new();

    let points: HashSet<(usize, usize)> = HashSet::from_iter(points.iter().cloned());

    while let Some((steps, location)) = queue.pop_front() {
        if visited.contains(&location) {
            continue;
        }
        visited.insert(location);

        if location == goal {
            return Some(steps);
        }
        for new_location in Direction::directions()
            .iter()
            .map(|d| d.neighbor(location))
            .flatten()
        {
            if new_location.0 < GRID_SIZE
                && new_location.1 < GRID_SIZE
                && !points.contains(&new_location)
            {
                queue.push_back((steps + 1, new_location));
            }
        }
    }

    None
}

fn first_to_cut_off(points: &[(usize, usize)]) -> Option<(usize, usize)> {
    // start at LINES_TO_READ; we know that up to there is good.
    let mut lo = LINES_TO_READ;
    let mut hi = points.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if steps_to_end(&points[..mid + 1]).is_none() {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    Some(points[lo])
}

fn main() {
    let mut points = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let point = line
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        points.push((point[0], point[1]));
    }
    println!("steps: {:?}", steps_to_end(&points[..LINES_TO_READ]));
    println!("first to cut off: {:?}", first_to_cut_off(&points));
}
