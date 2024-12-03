use std::fs::File;
use std::io::prelude::*;
use std::result;

type Result<T> = result::Result<T, ()>;

fn main() -> Result<()> {
    let mut file = File::open("day03/input.txt").map_err(|err| {
        eprintln!("ERROR: could not open file {err}");
    })?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|err| {
        eprintln!("ERROR: could not map contents of a file to a string {err}");
    })?;
    solve_part1(&contents);
    solve_part2(&contents);
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Token {
    Mul,
    Number(i64),
    LeftParen,
    RightParen,
    Comma,
    Do,
    Dont,
    Invalid,
}

fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '(' => {
                tokens.push(Token::LeftParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RightParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            c if c.is_digit(10) => {
                let mut number = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) {
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()))
            }
            c if c.is_alphabetic() => {
                let mut identifier = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphabetic() || ch == '\'' {
                        identifier.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if identifier.ends_with("don't") {
                    tokens.push(Token::Dont);
                } else if identifier.ends_with("do") {
                    tokens.push(Token::Do);
                } else if identifier.ends_with("mul") {
                    tokens.push(Token::Mul);
                } else {
                    tokens.push(Token::Invalid);
                }
            }
            _c => {
                tokens.push(Token::Invalid);
                chars.next();
            }
        }
    }
    tokens
}

fn solve_part1(input: &String) {
    let mut sum = 0;
    for line in input.lines() {
        let l: Vec<Token> = lexer(line);
        let mut iter = l.iter().peekable();
        while let Some(token) = iter.next() {
            if let Token::Mul = token {
                if let Some(Token::LeftParen) = iter.next() {
                    if let Some(Token::Number(x)) = iter.next() {
                        if let Some(Token::Comma) = iter.next() {
                            if let Some(Token::Number(y)) = iter.next() {
                                if let Some(Token::RightParen) = iter.next() {
                                    sum += x * y;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{sum}")
}

fn solve_part2(input: &String) {
    let mut sum = 0;
    let mut dont = false;
    for line in input.lines() {
        let l: Vec<Token> = lexer(line);
        let mut iter = l.iter().peekable();
        while let Some(token) = iter.next() {
            if let Token::Do = token {
                dont = false;
                continue;
            }
            if let Token::Dont = token {
                dont = true;
                continue;
            }
            if let Token::Mul = token {
                if dont {
                    continue;
                }
                if let Some(Token::LeftParen) = iter.next() {
                    if let Some(Token::Number(x)) = iter.next() {
                        if let Some(Token::Comma) = iter.next() {
                            if let Some(Token::Number(y)) = iter.next() {
                                if let Some(Token::RightParen) = iter.next() {
                                    sum += x * y;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{sum}")
}
