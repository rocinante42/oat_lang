use std::{
    env::args,
    io::{self, BufRead},
};

use anyhow::Result;

fn main() {
    let args: Vec<String> = args().collect();
    println!("args: {:?}", args);
    if args.len() > 2 {
        println!("Usage: oat-rs [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => {}
            Err(m) => eprintln!("{} at line: {}", m.message, m.line),
        }
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) -> Result<(), OatError> {
    if let Ok(buffer) = std::fs::read_to_string(path) {
        run(buffer.as_str());
    } else {
        return Err(OatError {
            message: String::from("Error reading path"),
            line: 0,
        });
    }

    Ok(())
}

fn run_prompt() -> Result<(), OatError> {
    let stdin = io::stdin();
    print!("> ");
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            } else {
                run(&line.as_str())?;
            }
        }
    }
    Ok(())
}
fn run(buffer: &str) -> Result<(), OatError> {
    let scanner = Scanner { buffer };
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}

#[derive(Debug)]
struct OatError {
    message: String,
    line: usize,
}

struct Scanner<'a> {
    buffer: &'a str,
}

impl<'a> Scanner<'a> {
    pub fn scan_tokens(&self) -> Vec<TokenType> {
        todo!()
    }
}

#[derive(Debug)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    True,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    While,
    Var,

    // End of file
    Eof,
}
