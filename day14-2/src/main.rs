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
                for addr in mask.apply(addr).expand().iter() {
                    memory.insert(*addr, val);
                }
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

    fn to_str(&self) -> String {
        let mut s = String::new();
        for i in 0..Mask::bits() {
            let bit = 1 << (Mask::bits() - i - 1);
            s.push(if self.ones & bit > 0 {
                '1'
            } else if self.zeros & bit > 0 {
                '0'
            } else {
                'X'
            })
        }
        s
    }

    fn apply(&self, val: u64) -> Mask {
        let floating = self.ones | self.zeros;
        Mask {
            zeros: (!val) & floating,
            ones: (val & floating) | self.ones,
        }
    }

    // TODO: I'm sure this is a hilariously cumbersome way to expand a mask
    // into all the possible adresses it describes, but oh well o_O.
    fn expand(&self) -> Vec<u64> {
        let mut floating: Vec<u32> = [].to_vec();
        for i in 0..Mask::bits() {
            let bit: u64 = 1 << i;
            if (self.ones | self.zeros) & bit == 0 {
                floating.push(i);
            }
        }

        let combinations = 1 << (Mask::bits() - (self.ones | self.zeros).count_ones());
        let mut r: Vec<u64> = [].to_vec();
        for i in 0..combinations {
            let mut val = self.ones;
            for (j, offset) in floating.iter().enumerate() {
                if i & (1 << j) > 0 {
                    val = val | (1 << offset);
                }
            }
            r.push(val);
        }
        r
    }

    fn bits() -> u32 {
        36
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
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
    ";

    #[test]
    fn example_answer() {
        let got = answer(EXAMPLE).unwrap();
        assert_eq!(got, 208);
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
        let mask = Mask::from("000000000000000000000000000000X1001X").unwrap();
        assert_eq!(
            mask.apply(0b000000000000000000000000000000101010).to_str(),
            "000000000000000000000000000000X1101X"
        );
    }

    #[test]
    fn mask_to_str() {
        let str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let mask = Mask::from(str).unwrap();
        assert_eq!(mask.to_str(), str);
    }

    #[test]
    fn mask_expand() {
        let str = "000000000000000000000000000000X1101X";
        let want: Vec<u64> = [
            0b000000000000000000000000000000011010,
            0b000000000000000000000000000000011011,
            0b000000000000000000000000000000111010,
            0b000000000000000000000000000000111011,
        ]
        .to_vec();
        let got = Mask::from(str).unwrap().expand();
        assert_eq!(got, want);
    }
}
