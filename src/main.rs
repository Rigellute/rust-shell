use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // Everything after the first whitespace is considered an arg
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();

        match command {
            "cd" => {
                // default to '/' if no arg was provided
                let new_dir = parts.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            command => {
                let child = Command::new(command).args(parts).spawn();

                match child {
                    Ok(mut child) => {
                        child.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
