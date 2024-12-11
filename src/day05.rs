use aoc_2024::util;
use std::collections::{HashMap, HashSet, VecDeque};

fn obeys_constraints(job: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> bool {
    for (before, all_after) in rules.iter() {
        if let Some(idx) = job.iter().position(|x| x == before) {
            if job.iter().take(idx).any(|s| all_after.contains(s)) {
                return false;
            }
        }
    }
    true
}

fn toposort_get_mid(job: &[usize], rules: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut sorted = Vec::new();
    let mut reversed = HashMap::new();
    let mut graph = HashMap::new();
    for (before, after) in rules.iter().filter(|(page, _)| job.contains(page)) {
        for a in after.iter().filter(|page| job.contains(page)) {
            graph.entry(*before).or_insert_with(HashSet::new).insert(a);
            reversed
                .entry(*a)
                .or_insert_with(HashSet::new)
                .insert(before);
        }
    }
    let mut no_incoming = VecDeque::from_iter(
        job.iter()
            .filter(|page| !reversed.contains_key(page))
            .cloned(),
    );
    while let Some(page) = no_incoming.pop_front() {
        sorted.push(page);
        if let Some(afters) = graph.remove(&page) {
            for &neighbor in afters.iter() {
                if reversed
                    .entry(*neighbor)
                    .and_modify(|set| {
                        set.remove(&page);
                    })
                    .or_default()
                    .iter()
                    .filter(|n| graph.contains_key(n))
                    .count()
                    == 0
                {
                    no_incoming.push_back(*neighbor);
                }
            }
        }
    }
    sorted[sorted.len() / 2]
}

fn sums(jobs: &[Vec<usize>], rules: &HashMap<usize, HashSet<usize>>) -> (usize, usize) {
    let (good, bad): (Vec<_>, Vec<_>) = jobs.iter().partition(|j| obeys_constraints(j, rules));
    (
        good.iter().map(|job| job[job.len() / 2]).sum(),
        bad.iter().map(|j| toposort_get_mid(j, rules)).sum(),
    )
}

fn main() {
    let mut rules = HashMap::new();
    let mut saw_empty = false;
    let mut jobs = Vec::new();
    for line in util::get_lines().map_while(Result::ok) {
        if line.is_empty() {
            saw_empty = true;
            continue;
        }
        if !saw_empty {
            let mut rule = line.split("|").map(|s| s.parse::<usize>().unwrap());
            rules
                .entry(rule.next().unwrap())
                .or_insert_with(HashSet::new)
                .insert(rule.next().unwrap());
        } else {
            let job = line
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            jobs.push(job);
        }
    }

    println!("middle sums: {:?}", sums(&jobs, &rules));
}
