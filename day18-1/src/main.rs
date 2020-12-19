use anyhow::{Error, Result};
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("answer: {}", answer(&input).unwrap());
}

fn answer(input: &str) -> Result<i64> {
    let mut sum = 0;
    for line in input.trim().split("\n") {
        let tokens = tokenize(line)?;
        let expr = parse(tokens)?;
        sum += expr.eval();
    }
    Ok(sum)
}

fn tokenize(expr: &str) -> Result<Vec<Token>> {
    let mut tokens = vec![];
    let mut place_value = 1;
    for c in expr.chars() {
        let token = match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                Token::Number(c.to_digit(10).unwrap() as i64)
            }
            '+' => Token::Plus,
            '*' => Token::Star,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            ' ' => continue,
            _ => return Err(Error::msg(format!("tokenize: bad char: \"{}\"", c))),
        };

        if let Token::Number(num) = token {
            if tokens.len() > 0 {
                let i = tokens.len() - 1;
                let prev_token = &tokens[i];
                if let Token::Number(prev_num) = prev_token {
                    tokens[i] = Token::Number(prev_num * place_value + num);
                    continue;
                }
            }
        }

        place_value = match token {
            Token::Number(_) => place_value * 10,
            _ => 1,
        };
        tokens.push(token);
    }
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            index: 0,
        }
    }

    fn parse(&mut self, in_parens: bool) -> Result<Expr> {
        if self.eof() {
            return Ok(Expr::Nil);
        }

        let mut left = self.parse_parens_or_number()?;
        loop {
            let token = self.token();
            let op = match token {
                Some(Token::Plus) => Op::Add,
                Some(Token::Star) => Op::Mulitply,
                Some(Token::CloseParen) => {
                    if in_parens {
                        return Ok(left);
                    } else {
                        return Err(Error::msg("unexpected CloseParen"));
                    }
                }
                _ => {
                    return Err(Error::msg(format!(
                        "expected Plus or Star, got: {:?}",
                        token
                    )))
                }
            };

            let right = self.parse_parens_or_number()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right));

            if self.eof() {
                return Ok(left);
            }
        }
    }

    fn parse_parens_or_number(&mut self) -> Result<Expr> {
        let token = self.token();
        match token {
            Some(Token::Number(n)) => Ok(Expr::Number(*n)),
            Some(Token::OpenParen) => self.parse(true),
            _ => Err(Error::msg(format!(
                "expected Number or OpenParen, got: {:?}",
                token
            ))),
        }
    }

    fn eof(&self) -> bool {
        self.index >= self.tokens.len()
    }

    fn token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }
}

fn parse(tokens: Vec<Token>) -> Result<Expr> {
    let mut p = Parser::new(tokens);
    p.parse(false)
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Mulitply,
}

#[derive(Debug, PartialEq)]
enum Expr {
    Binary(Op, Box<Expr>, Box<Expr>),
    Number(i64),
    Nil,
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Binary(op, a, b) => match op {
                Op::Add => a.eval() + b.eval(),
                Op::Mulitply => a.eval() * b.eval(),
            },
            Expr::Nil => 0,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Number(i64),
    Plus,
    Star,
    OpenParen,
    CloseParen,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("1 + 23").unwrap(),
            vec![Token::Number(1), Token::Plus, Token::Number(23)]
        );
        assert_eq!(
            tokenize("1+ (3*7 )").unwrap(),
            vec![
                Token::Number(1),
                Token::Plus,
                Token::OpenParen,
                Token::Number(3),
                Token::Star,
                Token::Number(7),
                Token::CloseParen,
            ]
        );
    }

    #[test]
    fn test_answer() {
        assert_eq!(answer("2 + 3").unwrap(), 5);
        assert_eq!(answer("2 * 3").unwrap(), 6);
        assert_eq!(answer("2 + 3 * 4").unwrap(), 20);
        assert_eq!(answer("1 + 2 * 3 + 4 * 5 + 6").unwrap(), 71);
        assert_eq!(answer("1 + (2 * 3) + (4 * (5 + 6))").unwrap(), 51);
        assert_eq!(answer("2 * 3 + (4 * 5)").unwrap(), 26);
        assert_eq!(answer("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap(), 437);
        assert_eq!(
            answer("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap(),
            12240
        );
        assert_eq!(
            answer("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(),
            13632
        );
    }
}
