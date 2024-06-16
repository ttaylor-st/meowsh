use std::env;
use std::io::{self, Write};
use std::process::Command;

fn prompt() {
    print!("meowsh> ");
    io::stdout().flush().expect("Could not flush stdout");
}

fn main() {
    let mut history = Vec::new();

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
            println!("No command provided");
            continue;
        }

        history.push(input.to_string());
        let command = parts[0];
        let args = &parts[1..];

        match command {
            "history" => {
                for (i, cmd) in history.iter().enumerate() {
                    println!("{}: {}", i + 1, cmd);
                }
            }
            "cd" => {
                if args.is_empty() {
                    eprintln!("cd: expected argument");
                } else {
                    let new_dir = args[0];
                    if let Err(e) = env::set_current_dir(new_dir) {
                        eprintln!("cd: {}", e);
                    }
                }
            }
            _ => match Command::new(command).args(args).spawn() {
                Ok(mut child) => {
                    child.wait().expect("Command wasn't running");
                }
                Err(e) => {
                    eprintln!("Failed to execute command: {}", e);
                }
            },
        }
    }
}
