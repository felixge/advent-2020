use anyhow::{anyhow, Error, Result};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = fs::read_to_string("./input.txt")?;
    println!("answer: {}", answer(&input)?);
    Ok(())
}

fn answer(input: &str) -> Result<u64> {
    let mut tiles = parse_tiles(input)?;
    let mut puzzle = Puzzle::new(tiles.remove(0).flip_y());
    while tiles.len() > 0 {
        let mut remaining = vec![];
        for tile in tiles {
            if let Some(_) = puzzle.assign(&tile) {
                // println!("assigned {} to {:?}", tile.id, point);
            } else {
                // println!("could not assign {}", tile.id);
                remaining.push(tile);
            }
        }
        tiles = remaining;
    }

    let puzzle = puzzle;

    for y in puzzle.map.min.y..puzzle.map.max.y + 1 {
        for x in puzzle.map.min.x..puzzle.map.max.x + 1 {
            let tile = puzzle.map.get(&Point { x, y }).unwrap();
            print!("{} ", tile.id);
        }
        print!("\n");
    }

    let monster = Tile::from_str(
        "Tile 0:
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   
",
    )
    .unwrap();

    let monster_width = monster.size;
    let monster_height = 3;
    let compacted = puzzle.compact();

    for y in puzzle.map.min.y..puzzle.map.max.y + 1 {
        for ty in 0..10 {
            for x in puzzle.map.min.x..puzzle.map.max.x + 1 {
                let p = Point { x: x, y: y };
                let tile = puzzle.map.get(&p).unwrap();
                for tx in 0..tile.size {
                    print!("{}", tile.get(tx, ty).unwrap());
                }
                print!(" ");
            }
            println!("");
        }
        println!("");
    }

    // println!("{}", compacted);

    for variation in compacted.variations() {
        // let mut roughness = 0;
        let mut monsters_found = vec![];
        for offset_x in 0..(variation.size - monster_width) {
            for offset_y in 0..(variation.size - monster_height) {
                let mut monster_found = true;
                for monster_x in 0..monster_width {
                    for monster_y in 0..monster_height {
                        let variation_x = monster_x + offset_x;
                        let variation_y = monster_y + offset_y;
                        let v_c = variation.get(variation_x, variation_y).unwrap();
                        let monster_c = monster.get(monster_x, monster_y).unwrap();
                        if *monster_c == '#' && *v_c != '#' {
                            monster_found = false;
                            break;
                        }
                    }
                }

                if monster_found {
                    monsters_found.push(Point {
                        x: offset_x as i64,
                        y: offset_y as i64,
                    });
                }
            }
        }

        if monsters_found.len() > 0 {
            let mut roughness = 0;
            for x in 0..variation.size {
                for y in 0..variation.size {
                    let mut in_monster = false;
                    for mp in monsters_found.iter() {
                        let mx = x as i64 - mp.x;
                        let my = y as i64 - mp.y;
                        if mx < 0
                            || my < 0
                            || mx >= monster_width as i64
                            || my >= monster_height as i64
                        {
                            continue;
                        }
                        let mc = monster.get(mx as usize, my as usize);
                        if let Some('#') = mc {
                            in_monster = true;
                            break;
                        }
                    }
                    let vc = variation.get(x, y).unwrap();
                    if !in_monster && *vc == '#' {
                        roughness += 1;
                    }
                }
            }
            return Ok(roughness);
            // println!("{} rough", roughness);
        }
    }
    println!("no monster : (");

    let mut answer = 1;
    for corner in puzzle.corners() {
        answer *= corner.id;
    }
    Ok(answer)
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0 }
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct SparseGrid<T> {
    min: Point,
    max: Point,
    map: HashMap<Point, T>,
}

impl<T> SparseGrid<T> {
    fn new() -> SparseGrid<T> {
        SparseGrid {
            min: Point::new(),
            max: Point::new(),
            map: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn insert(&mut self, p: Point, v: T) {
        if p.x < self.min.x {
            self.min.x = p.x;
        } else if p.x > self.max.x {
            self.max.x = p.x;
        }
        if p.y < self.min.y {
            self.min.y = p.y;
        } else if p.y > self.max.y {
            self.max.y = p.y;
        }
        self.map.insert(p, v);
    }

    fn get(&self, p: &Point) -> Option<&T> {
        self.map.get(p)
    }

    fn iter(&self) -> std::collections::hash_map::Iter<Point, T> {
        self.map.iter()
    }
}

#[derive(Debug)]
struct Puzzle {
    map: SparseGrid<Tile>,
}

impl Puzzle {
    fn new(tile: Tile) -> Puzzle {
        let mut map = SparseGrid::new();
        map.insert(Point::new(), tile);
        Puzzle { map: map }
    }

    fn assign(&mut self, new_tile: &Tile) -> Option<Point> {
        let directions = vec![
            (Side::Top, Point { x: 0, y: -1 }),
            (Side::Bottom, Point { x: 0, y: 1 }),
            (Side::Right, Point { x: 1, y: 0 }),
            (Side::Left, Point { x: -1, y: 0 }),
        ];
        for variation in new_tile.variations() {
            for (puzzle_point, puzzle_tile) in self.map.iter() {
                for direction in directions.iter() {
                    let new_point = puzzle_point.add(&direction.1);
                    if self.map.get(&new_point).is_some() {
                        continue;
                    } else if puzzle_tile.fits(&direction.0, &variation) {
                        self.map.insert(new_point, variation.clone());
                        return Some(new_point);
                    }
                }
            }
        }
        None
    }

    fn corners(&self) -> Vec<Tile> {
        vec![
            self.map.get(&self.map.min).unwrap().clone(),
            self.map
                .get(&Point {
                    x: self.map.min.x,
                    y: self.map.max.y,
                })
                .unwrap()
                .clone(),
            self.map.get(&self.map.max).unwrap().clone(),
            self.map
                .get(&Point {
                    x: self.map.max.x,
                    y: self.map.min.y,
                })
                .unwrap()
                .clone(),
        ]
    }

    fn compact(&self) -> Tile {
        let puzzle_size = self.map.max.x - self.map.min.x + 1;
        let first_tile = self.map.map.iter().next().unwrap().1;
        let inner_size = first_tile.size - 2;
        let compact_size = puzzle_size as usize * inner_size;
        let mut compact_tile = Tile::new(compact_size);
        for (point, tile) in self.map.iter() {
            for tile_x in 1..first_tile.size - 1 {
                for tile_y in 1..first_tile.size - 1 {
                    let puzzle_x = (point.x - self.map.min.x) as usize;
                    let puzzle_y = (point.y - self.map.min.y) as usize;
                    let compact_x = puzzle_x * inner_size + (tile_x - 1);
                    let compact_y = puzzle_y * inner_size + (tile_y - 1);
                    compact_tile.set(compact_x, compact_y, *tile.get(tile_x, tile_y).unwrap());
                }
            }
        }
        compact_tile
    }
}

fn parse_tiles(input: &str) -> Result<Vec<Tile>> {
    input
        .trim()
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<_>>()
}

#[derive(Debug, Clone)]
struct Tile {
    id: u64,
    data: Vec<char>,
    size: usize,
}

impl Tile {
    fn new(size: usize) -> Tile {
        let mut data = vec![];
        for _ in 0..size {
            for _ in 0..size {
                data.push('.');
            }
            data.push('\n');
        }
        Tile {
            id: 0,
            data: data,
            size: size,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.data.get(self.offset(x, y))
    }

    fn offset(&self, x: usize, y: usize) -> usize {
        y * (self.size + 1) + x
    }

    fn set(&mut self, x: usize, y: usize, c: char) {
        let offset = self.offset(x, y);
        self.data[offset] = c;
    }

    fn rotate(&self) -> Tile {
        let mut rotated = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                rotated.set(self.size - y - 1, x, *self.get(x, y).unwrap());
            }
        }
        rotated
    }

    fn flip_y(&self) -> Tile {
        let mut rotated = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                rotated.set(x, self.size - y - 1, *self.get(x, y).unwrap());
            }
        }
        rotated
    }

    fn flip_x(&self) -> Tile {
        let mut rotated = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                rotated.set(self.size - x - 1, y, *self.get(x, y).unwrap());
            }
        }
        rotated
    }

    /*

    123
    456
    789


    7: 0,2 <-> 2,0
    4: 0,1 <-> 2,1
    8: 1,2 <-> 1,0

    */
    fn flip_diagonal(&self) -> Tile {
        let mut flipped = self.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                flipped.set(self.size - x - 1, y, *self.get(x, y).unwrap());
            }
        }
        flipped
    }

    fn fits(&self, side: &Side, other: &Tile) -> bool {
        match side {
            Side::Left => {
                for y in 0..self.size {
                    if self.get(0, y) != other.get(self.size - 1, y) {
                        return false;
                    }
                }
                true
            }
            Side::Right => {
                for y in 0..self.size {
                    if self.get(self.size - 1, y) != other.get(0, y) {
                        return false;
                    }
                }
                true
            }
            Side::Top => {
                for x in 0..self.size {
                    if self.get(x, 0) != other.get(x, self.size - 1) {
                        return false;
                    }
                }
                true
            }
            Side::Bottom => {
                for x in 0..self.size {
                    if self.get(x, self.size - 1) != other.get(x, 0) {
                        return false;
                    }
                }
                true
            }
        }
    }

    fn variations(&self) -> Vec<Tile> {
        let mut variations = vec![];
        let mut tile = self.clone();
        for i in 0..4 {
            variations.push(tile.clone());
            variations.push(tile.flip_x());
            tile = tile.rotate();
        }
        variations
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Side {
    Left,
    Right,
    Top,
    Bottom,
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

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "tile {}:\n{}",
            self.id,
            self.data.iter().collect::<String>()
        )
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
