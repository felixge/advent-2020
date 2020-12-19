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

    /*
    Grammar for recursive descent parser below:

    expression     → multiply ;
    multiply       → add ( "*" add )* ;
    add            → primary ( "+" primary )* ;
    primary        → number | "(" expression ")" ;

    see http://craftinginterpreters.com/parsing-expressions.html
    */
    fn expression(&mut self) -> Result<Expr> {
        self.multiply()
    }

    fn multiply(&mut self) -> Result<Expr> {
        let mut expr = self.add()?;
        while match self.current_token() {
            Some(Token::Star) => true,
            _ => false,
        } {
            self.index += 1;
            let right = self.add()?;
            expr = Expr::Binary(Op::Multiply, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    fn add(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;
        while match self.current_token() {
            Some(Token::Plus) => true,
            _ => false,
        } {
            self.index += 1;
            let right = self.primary()?;
            expr = Expr::Binary(Op::Add, Box::new(expr), Box::new(right));
        }
        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr> {
        let token = self.next_token();
        match token {
            Some(Token::Number(n)) => Ok(Expr::Number(*n)),
            Some(Token::OpenParen) => {
                let expr = self.expression();
                match self.next_token() {
                    Some(Token::CloseParen) => expr,
                    _ => Err(Error::msg("expected ')' after expression")),
                }
            }
            _ => Err(Error::msg("expected number or '('")),
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn next_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }
}

fn parse(tokens: Vec<Token>) -> Result<Expr> {
    let mut p = Parser::new(tokens);
    p.expression()
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Multiply,
}

#[derive(Debug, PartialEq)]
enum Expr {
    Binary(Op, Box<Expr>, Box<Expr>),
    Number(i64),
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Binary(op, a, b) => match op {
                Op::Add => a.eval() + b.eval(),
                Op::Multiply => a.eval() * b.eval(),
            },
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
        assert_eq!(answer("2 + 3 * 4").unwrap(), 20);
        assert_eq!(answer("2 * 3 + 4").unwrap(), 14);
        assert_eq!(answer("1 + 2 * 3 + 4 * 5 + 6").unwrap(), 231);
        assert_eq!(answer("1 + (2 * 3) + (4 * (5 + 6))").unwrap(), 51);
        assert_eq!(answer("2 * 3 + (4 * 5)").unwrap(), 46);
        assert_eq!(answer("5 + (8 * 3 + 9 + 3 * 4 * 3)").unwrap(), 1445);
        assert_eq!(
            answer("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").unwrap(),
            669060
        );
        assert_eq!(
            answer("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").unwrap(),
            23340
        );
    }
}
