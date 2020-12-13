use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("answer: {}", answer(&input));
    //     println!(
    //         "answer: {}",
    //         answer(
    //             "
    // F10
    // N3
    // F7
    // R90
    // F11
    //     "
    //         )
    //     );
}

fn answer(input: &str) -> i32 {
    // (W-E), (N-S)
    let mut pos: (i32, i32) = (0, 0);
    let dirs = [
        (1 as i32, 0 as i32), // E
        (0, 1),               // S
        (-1, 0),              // W
        (0, -1),              // N
    ];
    let mut dir: usize = 0;
    for line in input.trim().split("\n") {
        let chars: Vec<char> = line.chars().collect();
        let command = chars[0];
        let arg: String = chars[1..].into_iter().collect();
        let num: i32 = arg.parse().unwrap();
        match command {
            'N' => pos.1 += -num,
            'S' => pos.1 += num,
            'W' => pos.0 += -num,
            'E' => pos.0 += num,
            'L' => {
                for _ in 0..num / 90 {
                    if dir == 0 {
                        dir = dirs.len() - 1
                    } else {
                        dir -= 1;
                    }
                }
            }
            'R' => {
                for _ in 0..num / 90 {
                    if dir + 1 == dirs.len() {
                        dir = 0
                    } else {
                        dir += 1;
                    }
                }
            }
            'F' => {
                pos.0 += (dirs[dir % dirs.len()].0 as i32) * num;
                pos.1 += (dirs[dir % dirs.len()].1 as i32) * num;
            }
            _ => panic!("bad command"),
        }
        println!("x={} y={}", pos.0, pos.1);
    }
    pos.0.abs() + pos.1.abs()
}
