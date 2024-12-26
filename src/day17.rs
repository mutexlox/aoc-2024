use aoc_2024::util;
use regex::Regex;
use std::collections::HashSet;
use std::sync::LazyLock;

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
    disable_jumps: bool,
}

impl Program {
    fn evaluate_manual_decompilation(&mut self) -> Vec<u8> {
        let mut out = Vec::new();

        while self.reg_a != 0 {
            let mut reg_b = self.reg_a & 7;
            reg_b ^= 2;
            let reg_c = self.reg_a >> reg_b;
            self.reg_a /= 8;
            reg_b ^= reg_c;
            reg_b ^= 7;
            out.push((reg_b & 7) as u8);
        }
        out
    }

    // Assumes that the program loops with one jnz at the end, processes 3 bits at a time and
    // outputs a single 3-bit value based on reg_a per iteration
    fn get_desired_a(&mut self) -> u64 {
        // track possibilities in `out`
        let mut out: HashSet<u64> = HashSet::new();
        out.insert(0);
        self.disable_jumps = true;
        for &code in self.clone().code.iter().rev() {
            let mut new_options = HashSet::new();

            for p in out.into_iter() {
                for attempt in 0..8_u64 {
                    self.reg_a = p * 8 + attempt;
                    self.pc = 0;
                    self.reg_b = 0;
                    self.reg_c = 0;
                    let step_out = self.evaluate();
                    if step_out[0] == code.into() {
                        new_options.insert(p * 8 + attempt);
                    }
                }
            }

            assert_ne!(new_options.len(), 0);
            out = new_options;
        }
        self.disable_jumps = false;
        *out.iter().min().unwrap()
    }

    fn evaluate(&mut self) -> Vec<u8> {
        let mut out = Vec::new();
        while self.pc < self.code.len() {
            match self.code[self.pc].try_into().unwrap() {
                OpCodes::Adv => self.reg_a /= 1 << self.combo_op(self.code[self.pc + 1]),
                OpCodes::Bxl => self.reg_b ^= self.code[self.pc + 1] as u64,
                OpCodes::Bst => self.reg_b = self.combo_op(self.code[self.pc + 1]) & 7,
                OpCodes::Jnz => {
                    if self.reg_a != 0 && !self.disable_jumps {
                        self.pc = self.code[self.pc + 1].into();
                        continue;
                    }
                }
                OpCodes::Bxc => self.reg_b ^= self.reg_c,
                OpCodes::Out => out.push((self.combo_op(self.code[self.pc + 1]) & 7) as u8),
                OpCodes::Bdv => {
                    self.reg_b = self.reg_a / (1 << self.combo_op(self.code[self.pc + 1]))
                }
                OpCodes::Cdv => {
                    self.reg_c = self.reg_a / (1 << self.combo_op(self.code[self.pc + 1]))
                }
            }
            self.pc += 2;
        }
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

    fn find_quine_input(&mut self) -> u64 {
        let a = self.get_desired_a();
        self.reg_a = a;
        self.reg_b = 0;
        self.reg_c = 0;
        self.pc = 0;
        let out = self.evaluate_manual_decompilation();
        assert_eq!(out, self.code);
        a
    }
}

fn main() {
    let mut program = Vec::new();
    static REGISTER_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"Register (?<reg>A|B|C): (?<val>\d+)").unwrap());
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

    let mut program = Program {
        code: program,
        pc: 0,
        reg_a,
        reg_b,
        reg_c,
        disable_jumps: false,
    };
    let mut p = program.clone();
    let out = program.evaluate();
    println!(
        "{}",
        out.iter()
            .map(|i| format!("{}", i))
            .collect::<Vec<_>>()
            .join(",")
    );
    assert_eq!(p.evaluate_manual_decompilation(), out);

    println!("{:?}", program.find_quine_input());
}
