use aoc_2024::util;
use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Write;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum OpCodes {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl OpCodes {
    const fn ordinal(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for OpCodes {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            val if val == OpCodes::Adv.ordinal() => Ok(OpCodes::Adv),
            val if val == OpCodes::Bxl.ordinal() => Ok(OpCodes::Bxl),
            val if val == OpCodes::Bst.ordinal() => Ok(OpCodes::Bst),
            val if val == OpCodes::Jnz.ordinal() => Ok(OpCodes::Jnz),
            val if val == OpCodes::Bxc.ordinal() => Ok(OpCodes::Bxc),
            val if val == OpCodes::Out.ordinal() => Ok(OpCodes::Out),
            val if val == OpCodes::Bdv.ordinal() => Ok(OpCodes::Bdv),
            val if val == OpCodes::Cdv.ordinal() => Ok(OpCodes::Cdv),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Program {
    code: Vec<u8>,
    pc: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

impl Program {
    fn evaluate(&mut self) -> String {
        let mut out = String::new();
        while self.pc < self.code.len() {
            match self.code[self.pc].try_into().unwrap() {
                OpCodes::Adv => self.reg_a /= 1 << self.combo_op(self.code[self.pc + 1]),
                OpCodes::Bxl => self.reg_b ^= self.code[self.pc + 1] as u64,
                OpCodes::Bst => self.reg_b = self.combo_op(self.code[self.pc + 1]) & 7,
                OpCodes::Jnz => {
                    if self.reg_a != 0 {
                        self.pc = self.code[self.pc + 1].into();
                        continue;
                    }
                }
                OpCodes::Bxc => self.reg_b ^= self.reg_c,
                OpCodes::Out => {
                    write!(out, "{},", self.combo_op(self.code[self.pc + 1]) & 7).unwrap()
                }
                OpCodes::Bdv => {
                    self.reg_b = self.reg_a / (1 << self.combo_op(self.code[self.pc + 1]))
                }
                OpCodes::Cdv => {
                    self.reg_c = self.reg_a / (1 << self.combo_op(self.code[self.pc + 1]))
                }
            }
            self.pc += 2;
        }
        out.pop();
        out
    }
    fn combo_op(&self, operand: u8) -> u64 {
        match operand {
            x if (0..=3).contains(&x) => x.into(),
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("reserved"),
            _ => panic!("bad operand {}", operand),
        }
    }
}

fn main() {
    let mut program = Vec::new();
    static REGISTER_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"Register (?<reg>A|B|C): (?<val>\d+)").unwrap());
    let mut saw_blank_line = false;
    let mut reg_a = 0;
    let mut reg_b = 0;
    let mut reg_c = 0;
    for line in util::get_lines().map_while(Result::ok) {
        if line.is_empty() {
            saw_blank_line = true;
            continue;
        }
        if saw_blank_line {
            program = line
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .split(",")
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>();
            continue;
        }
        if let Some(caps) = REGISTER_RE.captures(&line) {
            let val = caps["val"].parse::<u64>().unwrap();
            if &caps["reg"] == "A" {
                reg_a = val;
            } else if &caps["reg"] == "B" {
                reg_b = val;
            } else {
                assert_eq!(&caps["reg"], "C");
                reg_c = val;
            }
        }
    }

    println!(
        "program: {:?}, reg_a: {:?}, reg_b: {:?}, reg_c: {:?}",
        program, reg_a, reg_b, reg_c
    );

    let mut program = Program {
        code: program,
        pc: 0,
        reg_a,
        reg_b,
        reg_c,
    };
    println!("{}", program.evaluate());
}
