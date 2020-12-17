use anyhow::{Error, Result};
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("answer: {}", answer(&input).unwrap());
}

fn answer(input: &str) -> Result<usize> {
    let mut world = World::from_seed(input).unwrap();
    for _ in 0..6 {
        world = world.iterate();
    }
    Ok(world.active.len())
}

struct World {
    active: HashMap<Point, bool>,
    min: Point,
    max: Point,
}

impl World {
    fn new() -> World {
        World {
            active: HashMap::new(),
            min: Point::new(),
            max: Point::new(),
        }
    }

    fn from_seed(seed: &str) -> Result<World> {
        let mut world = World::new();
        for (y, line) in seed.trim().split("\n").enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = Point {
                    x: x as isize,
                    y: y as isize,
                    z: 0,
                    w: 0,
                };
                match c {
                    '#' => {
                        world.activate(&point);
                    }
                    '.' => {}
                    _ => return Err(Error::msg(format!("bad char: {}", c))),
                }
            }
        }
        Ok(world)
    }

    fn activate(&mut self, p: &Point) {
        self.active.insert(*p, true);
        self.min.x = min(self.min.x, p.x);
        self.min.y = min(self.min.y, p.y);
        self.min.z = min(self.min.z, p.z);
        self.min.w = min(self.min.w, p.w);
        self.max.x = max(self.max.x, p.x);
        self.max.y = max(self.max.y, p.y);
        self.max.z = max(self.max.z, p.z);
        self.max.w = max(self.max.w, p.w);
    }

    fn iterate(&mut self) -> World {
        let mut next = World::new();
        let mut points_simulated = 0;
        for z in self.min.z - 1..self.max.z + 2 {
            for x in self.min.x - 1..self.max.x + 2 {
                for y in self.min.y - 1..self.max.y + 2 {
                    for w in self.min.w - 1..self.max.w + 2 {
                        let p = Point { x, y, z, w };
                        let mut active_neighbors = 0;
                        for sp in p.surrounding() {
                            if self.active.get(&sp) != None {
                                active_neighbors += 1;
                            }
                        }
                        points_simulated += 1;
                        if self.active.get(&p) == None {
                            if active_neighbors == 3 {
                                next.activate(&p);
                            }
                        } else if active_neighbors == 2 || active_neighbors == 3 {
                            next.activate(&p);
                        }
                    }
                }
            }
        }
        // println!("points simulated {}", points_simulated);

        next
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
    }

    fn surrounding(&self) -> Vec<Point> {
        let mut points = vec![Point::new(); 80];
        let mut i = 0;
        for x in self.x - 1..self.x + 2 {
            for y in self.y - 1..self.y + 2 {
                for z in self.z - 1..self.z + 2 {
                    for w in self.w - 1..self.w + 2 {
                        let p = Point { x, y, z, w };
                        if p == *self {
                            continue;
                        }
                        points[i] = p;
                        i += 1;
                    }
                }
            }
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        let input = "
.#.
..#
###
        ";
        let got = answer(input).unwrap();
        assert_eq!(got, 848);
    }

    #[test]
    fn test_from_seed() {
        let input = "
.#.
..#
###
        ";
        let got = World::from_seed(input).unwrap();
        assert_eq!(got.active.len(), 5);
        assert_eq!(
            got.min,
            Point {
                x: 0,
                y: 0,
                w: 0,
                z: 0
            }
        );
        assert_eq!(
            got.max,
            Point {
                x: 2,
                y: 2,
                w: 0,
                z: 0
            }
        );
    }
}
