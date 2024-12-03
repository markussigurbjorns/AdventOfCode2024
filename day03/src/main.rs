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
    _solve_part2(&contents);
    Ok(())
}

#[derive(Debug, PartialEq)]
enum Token {
    Identifier(String),
    Number(i64),
    LeftParen,
    RightParen,
    Comma,
    Invalid(char),
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
                    if ch.is_alphanumeric() {
                        identifier.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(identifier));
            }
            c => {
                tokens.push(Token::Invalid(c));
                chars.next();
            }
        }
    }
    tokens
}

fn solve_part1(input: &String) {
    for line in input.lines() {
        let mut l: Vec<Token> = lexer(line);
        for v in l {
            println!("{:?}", v)
        }
    }
}

fn _solve_part2(_input: &String) {}
