use std::io::{self, Write};
use std::process::Command;

fn prompt() {
    print!("meowsh> ");
    io::stdout().flush().expect("Could not flush stdout");
}

fn main() {
    loop {
        prompt();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        let input = input.trim();
        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let command = parts[0];
        let args = &parts[1..];

        let mut child = Command::new(command)
            .args(args)
            .spawn()
            .expect("Could not execute command");

        child.wait().expect("Command failed");
    }
}
