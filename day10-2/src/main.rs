use anyhow::Result;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("{}", answer(&input).unwrap());
}

fn answer(input: &str) -> Result<i64> {
    let mut nums = to_numbers(input)?;
    nums.sort();
    nums.insert(0, 0);
    nums.push(nums[nums.len() - 1] + 3);
    Ok(recursive_answer(nums, &mut HashMap::new()))
}

fn recursive_answer(nums: Vec<i64>, cache: &mut HashMap<i64, i64>) -> i64 {
    if nums.len() <= 2 {
        return 1;
    }

    let mut variants = 0;
    for (i, num) in nums[1..].iter().enumerate() {
        if num - nums[0] > 3 {
            break;
        }

        variants += match cache.get(num) {
            Some(cached) => *cached,
            None => {
                let computed = recursive_answer(nums[i + 1..].to_vec(), cache);
                cache.insert(*num, computed);
                computed
            }
        }
    }
    variants
}

fn to_numbers(s: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    s.trim().split("\n").map(|x| x.parse()).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn answer() {
        let input = "
16
10
15
5
1
11
7
19
6
12
4        
        ";
        let got = super::answer(input).unwrap();
        assert_eq!(got, 8);

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
        assert_eq!(got, 19208);
    }
}
