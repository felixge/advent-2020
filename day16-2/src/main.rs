use anyhow::{Error, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("answer = {}", answer(&input).unwrap());
}

fn answer(input: &str) -> Result<i64> {
    let input = Input::from(input)?;
    let valid_tickets = valid_tickets(&input.nearby_tickets, &input.fields);
    let field_cols = field_columns(&valid_tickets, &input.fields);
    let mut result = 1;
    for (field_i, field) in input.fields.iter().enumerate() {
        if field.name.starts_with("departure") {
            let col_i = field_cols[field_i];
            let val = input.your_ticket[col_i];
            result *= val;
        }
    }

    Ok(result)
}

fn valid_tickets(tickets: &Vec<Vec<i64>>, fields: &Vec<Field>) -> Vec<Vec<i64>> {
    let mut valid_tickets: Vec<Vec<i64>> = vec![];
    'outer: for ticket in tickets.iter() {
        for val in ticket.iter() {
            let mut has_valid = false;
            for field in fields.iter() {
                if field.valid(*val) {
                    has_valid = true;
                    break;
                }
            }
            if !has_valid {
                continue 'outer;
            }
        }
        valid_tickets.push(ticket.clone());
    }
    valid_tickets
}

fn field_columns(tickets: &Vec<Vec<i64>>, fields: &Vec<Field>) -> Vec<usize> {
    // ugh, the code below is really verbose and ugly, there has got to be a
    // more elegant way to do this ...
    let mut cols = tickets[0].len();
    let mut field_to_col: HashMap<_, _> = HashMap::new();
    let mut col_to_field: HashMap<_, _> = HashMap::new();

    while field_to_col.len() < fields.len() {
        for (field_i, field) in fields.iter().enumerate() {
            if field_to_col.get(&field_i) != None {
                continue;
            }

            let mut unique_col: Option<usize> = None;
            for col_i in 0..cols {
                if col_to_field.get(&col_i) != None {
                    continue;
                }

                let mut col_valid = true;
                for ticket in tickets.iter() {
                    if !field.valid(ticket[col_i]) {
                        col_valid = false;
                        break;
                    }
                }
                if !col_valid {
                    continue;
                } else if unique_col == None {
                    unique_col = Some(col_i);
                } else {
                    unique_col = None;
                    break;
                }
            }

            if let Some(col_i) = unique_col {
                field_to_col.insert(field_i, col_i);
                col_to_field.insert(col_i, field_i);
            }
        }
    }

    let mut result = vec![];
    for (field_i, _) in fields.iter().enumerate() {
        result.push(*field_to_col.get(&field_i).unwrap());
    }
    result
}

struct Input {
    fields: Vec<Field>,
    your_ticket: Vec<i64>,
    nearby_tickets: Vec<Vec<i64>>,
}

impl Input {
    fn from(input: &str) -> Result<Input> {
        let mut result = Input {
            fields: vec![],
            your_ticket: vec![],
            nearby_tickets: vec![],
        };

        enum State {
            Fields,
            YourTicket,
            NearbyTickets,
        }
        let mut s = State::Fields;

        for line in input.trim().split("\n") {
            match s {
                State::Fields => {
                    if line == "your ticket:" {
                        s = State::YourTicket;
                    } else if line != "" {
                        result.fields.push(Field::from(line)?);
                    }
                }
                State::YourTicket => {
                    if line == "nearby tickets:" {
                        s = State::NearbyTickets;
                    } else if line != "" {
                        result.your_ticket = parse_line(line)?;
                    }
                }
                State::NearbyTickets => {
                    result.nearby_tickets.push(parse_line(line)?);
                }
            }
        }

        Ok(result)
    }
}

fn parse_line(line: &str) -> std::result::Result<Vec<i64>, std::num::ParseIntError> {
    line.split(",").map(|x| x.parse()).collect()
}

#[derive(Debug, PartialEq, Clone)]
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
        let r = Regex::new(r"^([\w ]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
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

    fn example() -> String {
        String::from(
            "
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
    ",
        )
    }

    #[test]
    fn test_answer() {
        let got = answer(example().as_str()).unwrap();
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

    #[test]
    fn test_field_columns() {
        let input = Input::from(example().as_str()).unwrap();
        let got = field_columns(input.nearby_tickets, input.fields);
        assert_eq!(got, vec![1, 0, 2]);
    }

    #[test]
    fn test_input_from() {
        let got = Input::from(example().as_str()).unwrap();
        assert_eq!(got.fields.len(), 3);
        assert_eq!(got.your_ticket, vec![11, 12, 13]);
        assert_eq!(
            got.nearby_tickets,
            vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9],]
        );
    }
}
