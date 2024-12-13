use aoc_2024::util;
use std::collections::VecDeque;

fn sum_costs(garden: &[Vec<char>]) -> u64 {
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let mut cost = 0;
    for (i, row) in garden.iter().enumerate() {
        for (j, &start) in row.iter().enumerate() {
            if visited[i][j] {
                continue;
            }
            // bfs from here for extent
            let mut queue = VecDeque::new();
            queue.push_back((i, j));
            let mut perim = 0;
            let mut area = 0;

            while let Some((next_i, next_j)) = queue.pop_front() {
                if visited[next_i][next_j] {
                    continue;
                }
                visited[next_i][next_j] = true;

                area += 1;
                if next_i > 0 && garden[next_i - 1][next_j] == start {
                    queue.push_back((next_i - 1, next_j));
                } else {
                    perim += 1;
                }
                if next_i < garden.len() - 1 && garden[next_i + 1][next_j] == start {
                    queue.push_back((next_i + 1, next_j));
                } else {
                    perim += 1;
                }
                if next_j > 0 && garden[next_i][next_j - 1] == start {
                    queue.push_back((next_i, next_j - 1));
                } else {
                    perim += 1;
                }
                if next_j < garden[0].len() - 1 && garden[next_i][next_j + 1] == start {
                    queue.push_back((next_i, next_j + 1));
                } else {
                    perim += 1;
                }
            }

            cost += area * perim;
        }
    }
    cost
}

fn sum_bulk_costs(garden: &[Vec<char>]) -> u64 {
    let mut visited = vec![vec![false; garden[0].len()]; garden.len()];
    let mut cost = 0;
    for (i, row) in garden.iter().enumerate() {
        for (j, &start) in row.iter().enumerate() {
            if visited[i][j] {
                continue;
            }
            // bfs from here for extent
            let mut queue = VecDeque::new();
            queue.push_back((i, j));
            let mut sides = 0;
            let mut area = 0;

            while let Some((next_i, next_j)) = queue.pop_front() {
                if visited[next_i][next_j] {
                    continue;
                }
                visited[next_i][next_j] = true;

                area += 1;
                if next_i > 0 && garden[next_i - 1][next_j] == start {
                    queue.push_back((next_i - 1, next_j));
                } else {
                    // check if left or right neighbor are visited && have this neighbor above; if not add 1 to sides
                    let mut continue_side = false;
                    if (next_j > 0
                        && visited[next_i][next_j - 1]
                        && garden[next_i][next_j - 1] == start)
                        && (next_i == 0 || garden[next_i - 1][next_j - 1] != start)
                    {
                        println!("line 80");
                        continue_side = true;
                    }
                    if (next_j < garden[0].len() - 1
                        && visited[next_i][next_j + 1]
                        && garden[next_i][next_j + 1] == start)
                        && (next_i == 0 || garden[next_i - 1][next_j + 1] != start)
                    {
                        println!("line 83");
                        continue_side = true;
                    }
                    println!(
                        "... for {} at {},{} up border continues a side? {}",
                        start, next_i, next_j, continue_side
                    );
                    if !continue_side {
                        sides += 1;
                    }
                }

                if next_i < garden.len() - 1 && garden[next_i + 1][next_j] == start {
                    queue.push_back((next_i + 1, next_j));
                } else {
                    // check if left or right neighbor are visited && have this neighbor below; if not add 1 to sides
                    let mut continue_side = false;
                    if (next_j > 0
                        && visited[next_i][next_j - 1]
                        && garden[next_i][next_j - 1] == start)
                        && (next_i == garden.len() - 1 || garden[next_i + 1][next_j - 1] != start)
                    {
                        println!("line 97");
                        continue_side = true;
                    }
                    if (next_j < garden[0].len() - 1
                        && visited[next_i][next_j + 1]
                        && garden[next_i][next_j + 1] == start)
                        && (next_i == garden.len() - 1 || garden[next_i + 1][next_j + 1] != start)
                    {
                        println!("line 100");
                        continue_side = true;
                    }
                    println!(
                        "... for {} at {},{} down border continues a side? {}",
                        start, next_i, next_j, continue_side
                    );
                    if !continue_side {
                        sides += 1;
                    }
                }

                if next_j > 0 && garden[next_i][next_j - 1] == start {
                    queue.push_back((next_i, next_j - 1));
                } else {
                    // check if up or down neighbor are visited && have this neighbor left; if not add 1 to sides
                    let mut continue_side = false;
                    if (next_i > 0
                        && visited[next_i - 1][next_j]
                        && garden[next_i - 1][next_j] == start)
                        && (next_j == 0 || garden[next_i - 1][next_j - 1] != start)
                    {
                        println!("line 114");
                        continue_side = true;
                    }
                    if (next_i < garden.len() - 1
                        && visited[next_i + 1][next_j]
                        && garden[next_i + 1][next_j] == start)
                        && (next_j == 0 || garden[next_i + 1][next_j - 1] != start)
                    {
                        println!("line 117XXX");
                        continue_side = true;
                    }
                    println!(
                        "... for {} at {},{} left border continues a side? {}",
                        start, next_i, next_j, continue_side
                    );
                    if !continue_side {
                        sides += 1;
                    }
                }

                if next_j < garden[0].len() - 1 && garden[next_i][next_j + 1] == start {
                    queue.push_back((next_i, next_j + 1));
                } else {
                    // check if up or down neighbor are visited && have this neighbor right; if not add 1 to sides
                    let mut continue_side = false;
                    if (next_i > 0
                        && visited[next_i - 1][next_j]
                        && garden[next_i - 1][next_j] == start)
                        && (next_j == garden[0].len() - 1
                            || garden[next_i - 1][next_j + 1] != start)
                    {
                        println!("line 131");
                        continue_side = true;
                    }
                    if (next_i < garden.len() - 1
                        && visited[next_i + 1][next_j]
                        && garden[next_i + 1][next_j] == start)
                        && (next_j == garden[0].len() - 1
                            || garden[next_i + 1][next_j + 1] != start)
                    {
                        println!("line 134XXX");
                        continue_side = true;
                    }
                    println!(
                        "... for {} at {},{} right border continues a side? {}",
                        start, next_i, next_j, continue_side
                    );
                    if !continue_side {
                        sides += 1;
                    }
                }
            }

            println!(
                "for {}: cost {}, area {}, sides {}",
                start,
                area * sides,
                area,
                sides
            );
            cost += area * sides;
        }
    }
    cost
}

fn rotate_90(garden: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut out = vec![vec![' '; garden.len()]; garden[0].len()];
    for (i, row) in garden.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            out[j][garden.len() - 1 - i] = item;
        }
    }
    out
}

fn mirror(garden: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut out = vec![vec![' '; garden[0].len()]; garden.len()];
    for (i, row) in garden.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            out[garden.len() - 1 - i][j] = item;
        }
    }
    out
}

fn main() {
    let mut garden = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        garden.push(line.chars().collect());
    }
    println!("{}", sum_costs(&garden));
    let bulk = sum_bulk_costs(&garden);
    println!("{}", bulk);

    let rotated_90 = rotate_90(&garden);
    //assert_eq!(bulk, sum_bulk_costs(&rotated_90));
    let rotated_180 = rotate_90(&rotated_90);
    //assert_eq!(bulk, sum_bulk_costs(&rotated_180));
    let rotated_270 = rotate_90(&rotated_180);
    //assert_eq!(bulk, sum_bulk_costs(&rotated_270));
    let mirror = mirror(&garden);
    //assert_eq!(bulk, sum_bulk_costs(&mirror));
    println!(
        "\n\n =============================== rotated 90 =============================== \n\n"
    );
    let costs_90 = sum_bulk_costs(&rotated_90);
    println!(
        "\n\n =============================== rotated 180 =============================== \n\n"
    );
    let costs_180 = sum_bulk_costs(&rotated_180);
    println!(
        "\n\n =============================== rotated 270 =============================== \n\n"
    );
    let costs_270 = sum_bulk_costs(&rotated_270);
    println!("\n\n =============================== mirrored =============================== \n\n");
    let costs_mirror = sum_bulk_costs(&mirror);
    println!(
        "normal: {}  90: {} 180: {} 270: {} mirror {}",
        bulk, costs_90, costs_180, costs_270, costs_mirror
    );
}
