use anyhow::Result;
use std::fmt;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", answer(&input).unwrap());
}

fn answer(input: &str) -> Result<u64> {
    let mut seats = build_seats(input);

    let mut occupied_before: Option<u64> = None;
    loop {
        let occupied = seats.iterate();
        match occupied_before {
            Some(before) => {
                if before == occupied {
                    return Ok(occupied);
                }
            }
            None => {}
        }
        occupied_before = Some(occupied);
    }
}

#[derive(Debug, Clone)]
struct Seats {
    data: Vec<char>,
    rows: u64,
    cols: u64,
}

impl Seats {
    fn iterate(&mut self) -> u64 {
        let before = self.clone();
        for row in 0..before.rows {
            for col in 0..before.cols {
                let offset = before.offset(row, col).unwrap();
                match before.data[offset] {
                    '#' => {
                        if before.nearby_occupied(row, col) >= 4 {
                            self.data[offset] = 'L';
                        }
                    }
                    'L' => {
                        if before.nearby_occupied(row, col) == 0 {
                            self.data[offset] = '#';
                        }
                    }
                    _ => {}
                }
            }
        }
        self.occupied()
    }

    fn nearby_occupied(&self, row: u64, col: u64) -> u8 {
        let mut occupied = 0;
        for r in row as i64 - 1..row as i64 + 2 {
            for c in col as i64 - 1..col as i64 + 2 {
                if r as u64 == row && c as u64 == col {
                    continue;
                }

                match self.offset(r as u64, c as u64) {
                    Some(o) => {
                        if self.data[o] == '#' {
                            occupied += 1
                        }
                    }
                    None => {}
                }
            }
        }

        occupied
    }

    fn occupied(&self) -> u64 {
        let mut occupied = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let offset = self.offset(row, col).unwrap();
                if self.data[offset] == '#' {
                    occupied += 1;
                }
            }
        }
        occupied
    }

    fn offset(&self, row: u64, col: u64) -> Option<usize> {
        if row >= self.rows {
            return None;
        } else if col >= self.cols {
            return None;
        }

        Some((row * (self.cols + 1) + col) as usize)
    }
}

impl fmt::Display for Seats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data.iter().collect::<String>())
    }
}

fn build_seats(input: &str) -> Seats {
    let mut seats = Seats {
        data: input.trim().chars().collect(),
        cols: 0,
        rows: 1,
    };
    let mut cols = 0;
    for c in seats.data.iter() {
        if *c == '\n' {
            seats.rows += 1;
            seats.cols = cols;
            cols = 0;
        } else {
            cols += 1;
        }
    }
    seats
}

#[cfg(test)]
mod tests {
    #[test]
    fn answer() {
        let input = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
        ";
        let got = super::answer(input).unwrap();
        assert_eq!(got, 37);
    }
}
