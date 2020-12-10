use std::fs;
use std::result::Result;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let nums = to_numbers(&input).unwrap();
    println!("first bad num: {}", first_bad_num(nums, 25));
}

fn to_numbers(s: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    s.trim().split("\n").map(|x| x.parse()).collect()
}

fn first_bad_num(nums: Vec<i64>, preamble: usize) -> i64 {
    for (i, &v) in nums[preamble..].iter().enumerate() {
        let prev = &nums[i as usize..(i as usize + preamble)];
        let mut found = false;
        for (i1, v1) in prev.iter().enumerate() {
            for (i2, v2) in prev.iter().enumerate() {
                if i1 != i2 && v1 + v2 == v {
                    found = true;
                    break;
                }
            }
        }
        if !found {
            return v;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_numbers() {
        let nums = to_numbers(
            "
5
8
10",
        )
        .unwrap();
        assert_eq!(nums, [5, 8, 10].to_vec());
    }

    #[test]
    fn test_first_bad_num() {
        let input = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
        ";
        let nums = to_numbers(input).unwrap();
        let got = first_bad_num(nums, 5);
        assert_eq!(got, 127);
    }
}
