use anyhow::{Error, Result};
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("answer: {}", answer(&input).unwrap());
}

fn answer(input: &str) -> Result<u64> {
    let mut p = Parser::new(input);
    let input = p.parse()?;

    let mut program = vm_compile(&input.rules)?;
    // println!("{:?}\n\nlen = {}", program, program.len());

    let mut count = 0;
    for sample in input.samples {
        let matched = vm_matches(&sample, &program)?;
        if matched {
            count += 1;
        }
    }
    Ok(count)
}

#[derive(Debug)]
enum Op {
    Rule(u64),
    Call(u64),
    Return,
    Char(char),
    Jump(isize),
    Fork(usize),
    EOF,
}

#[derive(Clone)]
struct Thread {
    pc: usize,
    returns: Vec<usize>,
}

impl Thread {
    fn with_pc(&self, pc: usize) -> Thread {
        return Thread {
            pc: pc,
            returns: self.returns.clone(),
        };
    }
}

fn vm_matches(s: &str, program: &Vec<Op>) -> Result<bool> {
    // each thread is just an program counter (pc) offset into the program vec
    let mut threads: Vec<Thread> = vm_forward_threads(
        &vec![Thread {
            pc: 0,
            returns: vec![],
        }],
        program,
    )?;
    let chars: Vec<char> = s.chars().collect();

    for str_c in chars {
        let mut new_threads = vec![];
        for thread in threads.iter() {
            let op = &program[thread.pc];
            match op {
                Op::Char(op_c) => {
                    if str_c == *op_c {
                        new_threads.push(thread.with_pc(thread.pc + 1));
                    }
                }
                Op::EOF => {}
                _ => return Err(Error::msg(format!("unexpected op: {:?}", op))),
            };
        }
        threads = vm_forward_threads(&new_threads, program)?;
    }
    for thread in threads.iter() {
        if let Some(Op::EOF) = program.get(thread.pc) {
            return Ok(true);
        }
    }
    Ok(false)
}

// forward_threads forwards all given threads until they hit an op that consumes
// a character or halts the thread.
fn vm_forward_threads(threads: &Vec<Thread>, program: &Vec<Op>) -> Result<Vec<Thread>> {
    let mut finalized = vec![];
    let mut pending = threads.clone();
    while pending.len() > 0 {
        let mut next_pending = vec![];
        for thread in pending.iter() {
            let op = program
                .get(thread.pc)
                .ok_or(Error::msg("pc out of bounds"))?;
            match op {
                Op::Rule(_) => next_pending.push(thread.with_pc(thread.pc + 1)),
                Op::Return => {
                    let mut returns = thread.returns.clone();
                    let return_pc = returns.pop().ok_or(Error::msg("invalid return"))?;
                    next_pending.push(Thread {
                        pc: return_pc,
                        returns: returns,
                    })
                }
                Op::Call(id) => {
                    let mut call_pc = 0;
                    for (search_pc, search_op) in program.iter().enumerate() {
                        if let Op::Rule(rule_id) = search_op {
                            if rule_id == id {
                                let mut returns = thread.returns.clone();
                                returns.push(thread.pc + 1);
                                next_pending.push(Thread {
                                    pc: search_pc,
                                    returns: returns,
                                });
                                break;
                            }
                        }
                    }
                }
                Op::Jump(j) => {
                    next_pending.push(thread.with_pc((thread.pc as isize + j) as usize));
                }
                Op::Fork(j) => {
                    next_pending.push(thread.with_pc(thread.pc + 1));
                    next_pending.push(thread.with_pc(thread.pc + j));
                }
                Op::Char(_) | Op::EOF => finalized.push(thread.clone()),
                _ => return Err(Error::msg(format!("unexpected op: {:?}", op))),
            };
        }
        pending = next_pending.clone();
    }
    Ok(finalized)
}

fn vm_compile(rules: &Vec<Rule>) -> Result<Vec<Op>> {
    let mut ops = vec![Op::Call(0), Op::EOF];
    for rule in rules.iter() {
        ops.append(&mut vm_compile_rule(rule)?);
    }
    Ok(ops)
}

fn vm_compile_rule(rule: &Rule) -> Result<Vec<Op>> {
    let mut ops = vec![];
    for alt in rule.alts.iter() {
        let mut alt_ops = vec![];
        for el in alt {
            let mut ops = match el {
                Element::Char(c) => vec![Op::Char(*c)],
                Element::Num(n) => vec![Op::Call(*n)],
            };
            alt_ops.append(&mut ops);
        }

        if ops.len() == 0 {
            ops = alt_ops;
        } else {
            let mut combined = vec![Op::Fork(ops.len() + 2)];
            combined.append(&mut ops);
            combined.push(Op::Jump(alt_ops.len() as isize + 1));
            combined.append(&mut alt_ops);
            ops = combined;
        }
    }

    let mut final_ops = vec![Op::Rule(rule.id)];
    final_ops.append(&mut ops);
    final_ops.push(Op::Return);
    Ok(final_ops)
}

/*
Grammar used by the recursive descent parser below:

input          -> rules "\n\n" samples
samples        -> sample ("\n sample)*
sample         -> ( any char not "\n" ) +
rules          -> rule ("\n" rule)*
rule           -> number ": " sequence (" | " sequence)*
sequence       -> element | (" " element)*
element        -> ( "\"" char "\"" | number )
number         -> digit+
char           -> any char not "\""
digit          -> "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

This is totally overkill for parsing this input format ... but also kind of
fun : p.
*/

struct Parser {
    input: Vec<char>,
    offset: usize,
}

impl Parser {
    fn new(input: &str) -> Parser {
        Parser {
            input: input.trim().chars().collect(),
            offset: 0,
        }
    }

    fn match_str(&mut self, want: &str) -> Result<bool> {
        let chars: Vec<char> = want.chars().collect();
        for (i, want_char) in chars.iter().enumerate() {
            let got = self.input.get(self.offset + i);
            match got {
                Some(got_char) if got_char == want_char => {}
                _ => {
                    return Err(Error::msg(format!(
                        "expected {:?}, but got {:?}",
                        want_char, got,
                    )))
                }
            };
        }
        self.offset += chars.len();
        Ok(true)
    }

    fn parse(&mut self) -> Result<Input> {
        let rules = self.rules()?;
        self.match_str("\n\n")?;
        let samples = self.samples()?;

        Ok(Input {
            rules: rules,
            samples: samples,
        })
    }

    fn rules(&mut self) -> Result<Vec<Rule>> {
        let mut rules = vec![self.rule()?];
        while self.match_str("\n").is_ok() {
            match self.rule() {
                Ok(rule) => rules.push(rule),
                Err(_) => {
                    self.offset -= 1;
                    break;
                }
            };
        }
        Ok(rules)
    }

    fn rule(&mut self) -> Result<Rule> {
        let id = self.number()?;
        self.match_str(": ")?;

        let mut alts = vec![self.sequence()?];
        while self.match_str(" | ").is_ok() {
            alts.push(self.sequence()?);
        }

        Ok(Rule { id, alts })
    }

    fn number(&mut self) -> Result<u64> {
        let mut first = true;
        let mut number = 0;
        loop {
            let got_char = self.input.get(self.offset);
            let digit = match got_char {
                Some(c) => c.to_digit(10),
                _ => None,
            };
            if let Some(d) = digit {
                number = number * 10 + d as u64;
                self.offset += 1;
                first = false;
            } else if first {
                return Err(Error::msg(format!("expected 0-9, got: {:?}", got_char)));
            } else {
                return Ok(number);
            }
        }
    }

    fn sequence(&mut self) -> Result<Vec<Element>> {
        let mut elements = vec![self.element()?];
        while self.match_str(" ").is_ok() {
            // this is an unfortunate way to bail out as sequence is now
            // aware of the rule above it : /
            if let Some(n) = self.input.get(self.offset) {
                if *n == '|' {
                    self.offset -= 1;
                    break;
                }
            }
            elements.push(self.element()?);
        }
        Ok(elements)
    }

    fn element(&mut self) -> Result<Element> {
        if let Ok(num) = self.number() {
            return Ok(Element::Num(num));
        } else if let Ok(c) = self.char() {
            return Ok(Element::Char(c));
        } else {
            return Err(Error::msg(format!(
                "expected number or char but found: {:?}",
                self.input.get(self.offset)
            )));
        }
    }

    fn char(&mut self) -> Result<char> {
        self.match_str("\"")?;
        let got = self.input.get(self.offset);
        let c = match got {
            Some(c) if *c != '"' => {
                self.offset += 1;
                Ok(*c)
            }
            _ => Err(Error::msg(format!("expected char, but got {:?}", got))),
        }?;
        self.match_str("\"")?;
        Ok(c)
    }

    fn samples(&mut self) -> Result<Vec<String>> {
        let mut samples = vec![self.sample()?];
        while self.match_str("\n").is_ok() {
            samples.push(self.sample()?);
        }
        Ok(samples)
    }

    fn sample(&mut self) -> Result<String> {
        let mut first = true;
        let mut s = String::new();
        loop {
            let got_char = match self.input.get(self.offset) {
                Some(c) if *c != '\n' => Some(*c),
                _ => None,
            };
            if let Some(c) = got_char {
                s.push(c);
                self.offset += 1;
                first = false;
            } else if first {
                return Err(Error::msg(format!("expected char, got: {:?}", got_char)));
            } else {
                return Ok(s);
            }
        }
    }
}

#[derive(Debug)]
struct Input {
    rules: Vec<Rule>,
    samples: Vec<String>,
}

#[derive(Debug)]
struct Rule {
    id: u64,
    alts: Vec<Vec<Element>>,
}

#[derive(Debug)]
enum Element {
    Char(char),
    Num(u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    // "a" (("aa" | "bb") ("ab" | "ba") | ("ab" | "ba") ("aa" | "bb")) "b"

    #[test]
    fn test_answer() {
        let input = "
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
        ";
        let got = answer(input).unwrap();
        assert_eq!(got, 2);
    }
}
