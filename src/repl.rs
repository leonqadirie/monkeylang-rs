use crate::lexing::lexer::Lexer;
use std::io::{self, BufRead, Result, Write};

const PROMPT: &str = ">>";

pub fn start_repl() -> Result<()> {
    println!("Hello. This is the Monkey programming language");
    println!("Feel free to type in commands");

    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();

    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;

        line.clear();
        if handle.read_line(&mut line)? == 0 {
            return Ok(()); // EOF on stdin
        }

        let mut lexer = Lexer::new(line.trim().to_string());
        while lexer.pos <= lexer.input.len() {
            let token = lexer.next_token();
            println!("{:?}", token);
        }
    }
}
