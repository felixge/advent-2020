use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    println!("answer: {}", answer(&input)?);
    Ok(())
}

fn answer(input: &str) -> Result<u64> {
    let tiles = parse_tiles(input)?;
    let mut border_map: HashMap<String, Vec<&Tile>> = HashMap::new();
    for tile in tiles.iter() {
        for border in tile.borders() {
            border_map
                .entry(border)
                .and_modify(|x| x.push(tile))
                .or_insert(vec![tile]);
        }
    }

    let mut corner_product = 1;
    for tile in tiles.iter() {
        let mut neighbors = HashSet::new();
        for border in tile.borders() {
            for tile in border_map.get(&border).unwrap() {
                neighbors.insert(tile.id);
            }
        }
        neighbors.remove(&tile.id);
        if neighbors.len() == 2 {
            corner_product *= tile.id;
        }
    }

    Ok(corner_product)
}

fn parse_tiles(input: &str) -> Result<Vec<Tile>> {
    input
        .trim()
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<_>>()
}

#[derive(Debug)]
struct Tile {
    id: u64,
    data: Vec<char>,
    size: usize,
}

impl Tile {
    fn borders(&self) -> Vec<String> {
        let mut top = String::new();
        let mut bottom = String::new();
        for x in 0..self.size {
            top.push(*self.get(x, 0).unwrap());
            bottom.push(*self.get(x, self.size - 1).unwrap());
        }
        let mut left = String::new();
        let mut right = String::new();
        for y in 0..self.size {
            left.push(*self.get(0, y).unwrap());
            right.push(*self.get(self.size - 1, y).unwrap());
        }

        let mut borders = vec![top, right, bottom, left];
        let mut flipped = vec![];
        for border in borders.iter() {
            flipped.push(border.chars().rev().collect::<String>());
        }
        borders.append(&mut flipped);
        borders
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        let offset = y * (self.size + 1) + x;
        self.data.get(offset)
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Tile> {
        let parts = s
            .split("Tile ")
            .collect::<Vec<_>>()
            .get(1)
            .ok_or(anyhow!("bad tile"))?
            .split(":\n")
            .collect::<Vec<_>>();
        let id = parts.get(0).ok_or(anyhow!("no tile id"))?.parse()?;
        let data = *parts.get(1).ok_or(anyhow!("no data"))?;
        let size = data
            .split("\n")
            .next()
            .ok_or(anyhow!("no first row"))?
            .len();
        let data = data.chars().collect();
        Ok(Tile { id, size, data })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_answer() {
        let input = include_str!("./example.txt");
        let got = answer(input).unwrap();
        assert_eq!(got, 20899048083289);
    }
}
