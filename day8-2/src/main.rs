use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("read input");
    println!("acc: {}", get_acc(&input));
}

fn get_acc(input: &str) -> isize {
    let mut op = 0;
    loop {
        let acc = get_acc_flip(op, &input);
        if acc != -1 {
            break acc;
        }
        op += 1;
    }
}

fn get_acc_flip(op_index: i32, input: &str) -> isize {
    let ops: Vec<&str> = input.trim().split("\n").collect();
    let mut prev_pcs = HashMap::new();

    let mut pc: isize = 0;
    let mut acc: isize = 0;
    let mut op_count: i32 = 0;
    loop {
        if pc < 0 {
            panic!("pc < 0")
        } else if pc as usize >= ops.len() {
            return acc;
        }

        let op = ops[pc as usize];

        let parts: Vec<&str> = op.split(" ").collect();
        if let [mut op, n_str] = parts[..] {
            let n = n_str.parse::<isize>().expect(n_str);
            if op_count == op_index {
                if op == "nop" {
                    op = "jmp";
                } else if op == "jmp" {
                    op = "nop";
                }
            }

            match op {
                "nop" => {
                    pc += 1;
                    op_count += 1;
                }
                "acc" => {
                    acc += n;
                    pc += 1
                }
                "jmp" => {
                    pc += n;
                    op_count += 1;
                }
                _ => panic!("unknown op: {}", op),
            }
            if prev_pcs.contains_key(&pc) {
                return -1;
            }
            prev_pcs.insert(pc, true);
        } else {
            panic!("bad op: {}", op)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let acc = get_acc(
            "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6",
        );
        assert_eq!(acc, 8);
    }
}
