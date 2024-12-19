use aoc_2024::util;
use std::collections::HashMap;

fn word_possible(seen: &mut HashMap<String, usize>, tokens: &[String], word: &str) -> usize {
    if word.is_empty() {
        return 1;
    }
    if let Some(&count) = seen.get(word) {
        return count;
    }
    let mut count = 0;
    for t in tokens.iter() {
        if let Some(rest) = word.strip_prefix(t) {
            count += word_possible(seen, tokens, rest);
        }
    }
    seen.insert(word.to_string(), count);
    count
}

fn count_possible_words(tokens: &[String], words: &[String]) -> (usize, usize) {
    let mut seen = HashMap::new();
    let doable = words
        .iter()
        .map(|w| word_possible(&mut seen, tokens, w))
        .filter(|&x| x != 0)
        .collect::<Vec<_>>();
    (doable.len(), doable.iter().sum())
}

fn main() {
    let mut tokens = Vec::new();
    let mut words = Vec::new();
    let mut seen_empty = false;
    for line in util::get_lines().map_while(Result::ok) {
        if line.is_empty() {
            seen_empty = true;
            continue;
        }
        if !seen_empty {
            tokens = line.split(',').map(|s| s.trim().to_string()).collect();
        } else {
            words.push(line);
        }
    }
    println!("{:?}", count_possible_words(&tokens, &words));
}
