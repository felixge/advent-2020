use anyhow::Result;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("answer: {}", answer(&input).unwrap());
}

struct Memory {
    map: HashMap<u64, u64>,
}

impl Memory {
    fn new() -> Memory {
        return Memory {
            map: HashMap::new(),
        };
    }

    fn speak(&mut self, number: u64, round: u64) -> Option<u64> {
        let last_spoken = match self.map.get(&number) {
            Some(n) => Some(*n),
            None => None,
        };
        self.map.insert(number, round);
        last_spoken
    }
}

fn answer(input: &str) -> Result<u64> {
    let mut mem = Memory::new();
    let mut round: u64 = 0;
    let mut last_spoken: Option<u64> = None;
    let lines: Vec<&str> = input.trim().split(",").collect();
    for line in lines.iter() {
        round += 1;
        last_spoken = mem.speak(line.parse()?, round);
    }

    loop {
        round += 1;
        let new_num = match last_spoken {
            Some(last_round) => round - last_round - 1,
            None => 0,
        };
        last_spoken = mem.speak(new_num, round);
        if round == 30000000 {
            return Ok(new_num);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(answer("0,3,6").unwrap(), 175594);
    }
}
