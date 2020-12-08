use std::collections::HashMap;

fn main() {
    let acc = get_acc("
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
");

    println!("{}", acc);
}

fn get_acc(input: &str) -> isize {
    let ops: Vec<&str> = input.trim().split("\n").collect();
    let mut prev_pcs = HashMap::new();

    let mut pc: isize = 0;
    let mut acc: isize = 0;
    loop {
        let op = ops[pc as usize];
        let parts: Vec<&str> = op.split(" ").collect();
        if let [op, n_str] = parts[..] {
            let n = n_str.parse::<isize>().expect(n_str);
            match op {
                "nop" => pc += 1,
                "acc" => {
                    acc += n;
                    pc += 1
                },
                "jmp" => {
                    pc += n
                }
                _ => println!("unknown op: {}", op)
            }
            if prev_pcs.contains_key(&pc) {
                return acc
            }
            prev_pcs.insert(pc, true);
        } else {
            println!("bad op: {}", op)
        }
    }
}
