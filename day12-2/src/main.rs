use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("answer: {}", answer(&input));
}

#[derive(Debug, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    // fn rotate(&self, degree: i64) -> Pos {
    //     let times = degree / 90;
    //     let (mut x, mut y) = (self.x, self.y);
    //     for _ in 0..times.abs() {
    //         if ((x > 0 && y > 0) || (x < 0 && y < 0)) == (times > 0) {
    //             y = -y;
    //         } else {
    //             x = -x;
    //         }
    //     }
    //     Pos { x: x, y: y }
    // }
    fn rotate(&self, degree: i64) -> Pos {
        let times = degree / 90;
        let mut pos = Pos {
            x: self.x,
            y: self.y,
        };
        for _ in 0..times.abs() {
            if times < 0 {
                pos = Pos {
                    x: pos.y,
                    y: -pos.x,
                };
            } else {
                pos = Pos {
                    x: -pos.y,
                    y: pos.x,
                };
            }
        }
        pos
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl Action {
    fn from(s: &str) -> Option<Action> {
        if s.len() < 2 {
            return None;
        }
        let chars: Vec<char> = s.chars().collect();
        let arg: String = chars[1..].into_iter().collect();
        let arg_n = match arg.parse::<i64>() {
            Err(_) => return None,
            Ok(n) => n,
        };
        Some(match chars[0] {
            'N' => Action::North(arg_n),
            'S' => Action::South(arg_n),
            'E' => Action::East(arg_n),
            'W' => Action::West(arg_n),
            'F' => Action::Forward(arg_n),
            'L' => Action::Left(arg_n),
            'R' => Action::Right(arg_n),
            _ => return None,
        })
    }
}

fn answer(input: &str) -> i64 {
    let mut ship = Pos { x: 0, y: 0 };
    let mut waypoint = Pos { x: 10, y: -1 };
    for line in input.trim().split("\n") {
        let action = Action::from(line).expect("bad action");
        match action {
            Action::North(n) => waypoint.y -= n,
            Action::South(n) => waypoint.y += n,
            Action::West(n) => waypoint.x -= n,
            Action::East(n) => waypoint.x += n,
            Action::Forward(n) => {
                ship.x += waypoint.x * n;
                ship.y += waypoint.y * n;
            }
            Action::Left(n) => waypoint = waypoint.rotate(-n),
            Action::Right(n) => waypoint = waypoint.rotate(n),
        }
        println!("{:?} => ship: {:?} waypoint: {:?}", action, ship, waypoint);
    }
    ship.x.abs() + ship.y.abs()
}

#[cfg(test)]
mod tests {
    use super::{answer, Action, Pos};
    #[test]
    fn test_example() {
        let input = "
F10
N3
F7
R90
F11
        ";
        assert_eq!(answer(input), 286);
    }

    #[test]
    fn pos_rotate() {
        let pos = &Pos { x: 10, y: -4 }; // 10E 4N
        assert_eq!(pos.rotate(90), Pos { x: 4, y: 10 }); // 4E 10S
        assert_eq!(pos.rotate(0), Pos { x: 10, y: -4 });
    }

    #[test]
    fn action_from() {
        assert_eq!(Action::from("N3"), Some(Action::North(3)));
        assert_eq!(Action::from("S4"), Some(Action::South(4)));
        assert_eq!(Action::from("E5"), Some(Action::East(5)));
        assert_eq!(Action::from("W6"), Some(Action::West(6)));
        assert_eq!(Action::from("F10"), Some(Action::Forward(10)));
        assert_eq!(Action::from("R90"), Some(Action::Right(90)));
        assert_eq!(Action::from("L180"), Some(Action::Left(180)));
    }
}
