use anyhow::{Error, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("answer = {}", answer(&input).unwrap());
}

fn answer(input: &str) -> Result<u64> {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = Mask { zeros: 0, ones: 0 };
    for line in input.trim().split("\n") {
        let ins = Instruction::from(line)?;
        match ins {
            Instruction::Mask(m) => {
                mask = m;
            }
            Instruction::Mem(addr, val) => {
                memory.insert(addr, mask.apply(val));
            }
        }
    }
    let mut sum = 0;
    for (_, val) in memory.iter() {
        sum += *val;
    }
    Ok(sum)
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mask(Mask),
    Mem(u64, u64),
}

#[derive(Debug, PartialEq)]
struct Mask {
    zeros: u64,
    ones: u64,
}

impl Mask {
    fn from(s: &str) -> Result<Mask> {
        let mut mask = Mask { zeros: 0, ones: 0 };
        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '0' => mask.zeros = mask.zeros | 1 << i,
                '1' => mask.ones = mask.ones | 1 << i,
                'X' => {}
                _ => return Err(Error::msg("bad mask val")),
            }
        }
        Ok(mask)
    }

    fn apply(&self, val: u64) -> u64 {
        (val | self.ones) & !self.zeros
    }
}

impl Instruction {
    fn from(s: &str) -> Result<Instruction> {
        let re = Regex::new(r"^(mem|mask)(?:\[(\d+)\])? = (.+)$")?;
        let caps = re.captures(s).ok_or(Error::msg("regex did not match"))?;
        let cmd = caps.get(1).ok_or(Error::msg("cmd did not match"))?.as_str();
        let val = caps.get(3).ok_or(Error::msg("val did not match"))?.as_str();
        match cmd {
            "mem" => Ok(Instruction::Mem(
                caps.get(2)
                    .ok_or(Error::msg("mem addr did not match"))?
                    .as_str()
                    .parse::<u64>()?,
                val.parse::<u64>()?,
            )),
            "mask" => Ok(Instruction::Mask(Mask::from(val)?)),
            _ => Err(Error::msg(format!("invalid cmd: {}", cmd))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
    ";

    #[test]
    fn example_answer() {
        let got = answer(EXAMPLE).unwrap();
        assert_eq!(got, 165);
    }

    #[test]
    fn instruction_from() {
        assert_eq!(
            Instruction::from("mem[8] = 11").unwrap(),
            Instruction::Mem(8, 11)
        );
        assert_eq!(
            Instruction::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap(),
            Instruction::Mask(Mask {
                ones: 0b000000000000000000000000000001000000,
                zeros: 0b000000000000000000000000000000000010,
            })
        );
    }

    #[test]
    fn mask_apply() {
        let mask = Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);
    }
}
