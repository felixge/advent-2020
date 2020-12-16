use anyhow::{Error, Result};
use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("answer = {}", answer(&input).unwrap());
}

fn answer(input: &str) -> Result<i64> {
    enum State {
        Fields,
        YourTicket,
        NearbyTickets,
    }

    let mut s = State::Fields;
    let mut fields: Vec<Field> = vec![];
    let mut invalid_sum: i64 = 0;
    for line in input.trim().split("\n") {
        match s {
            State::Fields => {
                if line == "your ticket:" {
                    s = State::YourTicket;
                } else if line != "" {
                    fields.push(Field::from(line)?);
                }
            }
            State::YourTicket => {
                if line == "nearby tickets:" {
                    s = State::NearbyTickets;
                }
            }
            State::NearbyTickets => {
                for val in line.split(",") {
                    let val = val.parse::<i64>()?;
                    let mut valid = false;
                    for field in fields.iter() {
                        if field.valid(val) {
                            valid = true;
                            break;
                        }
                    }
                    if !valid {
                        invalid_sum += val;
                    }
                }
            }
        }
    }
    Ok(invalid_sum)
}

#[derive(Debug, PartialEq)]
struct Field {
    name: String,
    ranges: [(i64, i64); 2],
}

impl Field {
    fn valid(&self, input: i64) -> bool {
        (input >= self.ranges[0].0 && input <= self.ranges[0].1)
            || (input >= self.ranges[1].0 && input <= self.ranges[1].1)
    }
}

impl Field {
    fn from(input: &str) -> Result<Field> {
        let r = Regex::new(r"([a-z]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let catpures = r.captures(input).ok_or(Error::msg("regex doesn't match"))?;
        let name = String::from(catpures.get(1).unwrap().as_str());
        Ok(Field {
            name,
            ranges: [
                (
                    catpures.get(2).unwrap().as_str().parse()?,
                    catpures.get(3).unwrap().as_str().parse()?,
                ),
                (
                    catpures.get(4).unwrap().as_str().parse()?,
                    catpures.get(5).unwrap().as_str().parse()?,
                ),
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        let example = "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";
        let got = answer(example).unwrap();
        assert_eq!(got, 71);
    }

    #[test]
    fn test_field_from() {
        assert_eq!(
            Field::from("class: 1-3 or 5-7").unwrap(),
            Field {
                name: String::from("class"),
                ranges: [(1, 3), (5, 7)],
            }
        )
    }
}
