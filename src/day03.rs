use anyhow::Result;
use aoc_2024::util;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;
use std::vec::Vec;

#[derive(Debug)]
enum Ops {
    Mul(u64, u64),
    Disable,
    Enable,
}

#[derive(Debug)]
struct Computer {
    ops: Vec<Ops>,
    enabled: bool,
    pub ignore_disable: bool,
}

impl Computer {
    fn new(ops: Vec<Ops>) -> Self {
        Self {
            ops,
            enabled: true,
            ignore_disable: false,
        }
    }

    fn evaluate(&mut self) -> u64 {
        let mut out = 0;
        for op in &self.ops {
            match op {
                Ops::Mul(lhs, rhs) => {
                    if self.enabled || self.ignore_disable {
                        out += lhs * rhs
                    }
                }
                Ops::Disable => self.enabled = false,
                Ops::Enable => self.enabled = true,
            }
        }
        out
    }
}

impl FromStr for Computer {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        static RE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"mul\((?<lhs>\d+),(?<rhs>\d+)\)|do\(\)|don't\(\)").unwrap()
        });
        let mut out = Vec::new();
        for captures in RE.captures_iter(s) {
            let full = captures.get(0).unwrap().as_str();
            if full == "don't()" {
                out.push(Ops::Disable);
            } else if full == "do()" {
                out.push(Ops::Enable);
            } else {
                out.push(Ops::Mul(
                    captures["lhs"].parse::<u64>()?,
                    captures["rhs"].parse::<u64>()?,
                ));
            }
        }
        Ok(Computer::new(out))
    }
}

fn main() {
    let contents = util::get_all_input();
    let mut parsed = contents.parse::<Computer>().unwrap();
    parsed.ignore_disable = true;
    println!("evaluated pt 1: {}", parsed.evaluate());
    parsed.ignore_disable = false;
    println!("evaluated pt 2: {}", parsed.evaluate());
}
