use aoc_2024::util;

fn is_safe(report: &[i64]) -> bool {
    if report.len() <= 1 {
        return true;
    }
    let increasing = report[0] < report[1];
    let mut last = report[0];
    for &n in report.iter().skip(1) {
        if last == n {
            return false;
        }
        if increasing != (last < n) {
            return false;
        }
        if last.abs_diff(n) > 3 {
            return false;
        }
        last = n;
    }
    true
}

fn is_safe_dampened(report: &[i64]) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let start = report.iter().take(i);
        let end = report.iter().skip(i + 1);
        if is_safe(&start.chain(end).cloned().collect::<Vec<_>>()) {
            return true;
        }
    }
    false
}

fn count_safe(reports: &[Vec<i64>], checker: fn(&[i64]) -> bool) -> usize {
    reports.iter().filter(|r| checker(r)).count()
}

fn main() {
    let mut reports = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        let report = line
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        reports.push(report);
    }
    println!("{:?}", count_safe(&reports, is_safe));
    println!("{:?}", count_safe(&reports, is_safe_dampened));
}
