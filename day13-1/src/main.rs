use anyhow::{Error, Result};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    println!("answer: {}", answer(&input));
}

fn answer(input: &str) -> u64 {
    let schedule = parse_input(input).unwrap();
    let mut time = schedule.time;
    loop {
        for bus in schedule.buses.iter() {
            if time % bus == 0 {
                return (time - schedule.time) * bus;
            }
        }
        time += 1;
    }
}

fn parse_input(input: &str) -> Result<Schedule> {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    if lines.len() != 2 {
        return Err(Error::msg("bad number of lines"));
    }

    let time = lines[0].parse()?;
    let mut buses = [].to_vec();

    for bus in lines[1].split(",") {
        if let Ok(bus) = bus.parse::<u64>() {
            buses.push(bus);
        }
    }

    Ok(Schedule { time, buses })
}

#[derive(Debug)]
struct Schedule {
    time: u64,
    buses: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::answer;

    #[test]
    fn test_answer() {
        let got = answer(
            "
939
7,13,x,x,59,x,31,19
",
        );
        assert_eq!(got, 295);
    }
}
