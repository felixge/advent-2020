use anyhow::Result;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", answer(&input).unwrap());
}

fn answer(input: &str) -> Result<i64> {
    let mut nums = to_numbers(input)?;
    nums.sort();

    let mut d1 = 0;
    let mut d3 = 1;
    let mut prev = 0;
    for cur in nums.iter() {
        let diff = cur - prev;
        match diff {
            1 => d1 += 1,
            3 => d3 += 1,
            _ => (),
        }
        prev = *cur;
    }

    Ok(d1 * d3)
}

fn to_numbers(s: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    s.trim().split("\n").map(|x| x.parse()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn answer() {
        let input = "
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
        ";
        let got = super::answer(input).unwrap();
        assert_eq!(got, 22 * 10);
    }
}
