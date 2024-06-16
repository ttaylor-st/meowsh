use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

use termion::event::Key;
use termion::input::TermRead;

fn get_prompt() -> String {
    format!(
        "{}$ ",
        env::current_dir()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
    )
}

fn read_line() -> String {
    let stdin = io::stdin();
    let stdin = stdin.keys();
    let mut line = String::new();
    let mut cursor = 0;

    for c in stdin {
        let c = c.unwrap();
        match c {
            Key::Char('\n') => break,
            Key::Char(c) => {
                line.insert(cursor, c);
                cursor += 1;
            }
            Key::Backspace => {
                if cursor > 0 {
                    cursor -= 1;
                    line.remove(cursor);
                    print!("\u{8} \u{8}");
                }
            }
            // TODO: implement history navigation, cursor movement & tab completion
            _ => {}
        }
        io::stdout().flush().unwrap();
    }

    line
}

fn execute_command(line: &str, history: &mut Vec<String>) {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    let command = parts[0];
    let args = &parts[1..];

    match command {
        "exit" => std::process::exit(0),
        "history" => {
            for (i, cmd) in history.iter().enumerate() {
                println!("{}: {}", i + 1, cmd);
            }
        }
        "cd" => {
            if let Some(new_dir) = args.get(0) {
                if let Err(e) = env::set_current_dir(new_dir) {
                    eprintln!("cd: {}", e);
                }
            } else {
                eprintln!("cd: expected argument");
            }
        }
        _ => match Command::new(command).args(args).spawn() {
            Ok(mut child) => {
                child.wait().expect("Command failed to run");
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    eprintln!("{}: command not found", command);
                } else {
                    eprintln!("Error: {}", e);
                }
            }
        },
    }
}

fn read_history() -> Vec<String> {
    let home = env::var("HOME").expect("$HOME not set");
    let history_file = format!("{}/.meowsh_history", home);
    let history = fs::read_to_string(history_file).unwrap_or_default();
    history.lines().map(|s| s.to_string()).collect()
}

fn write_history(history: &[String]) {
    let home = env::var("HOME").expect("$HOME not set");
    let history_file = format!("{}/.meowsh_history", home);
    fs::write(history_file, history.join("\n")).expect("Could not write history");
}

fn main() {
    let mut history = read_history();
    loop {
        let prompt = get_prompt();
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let line = read_line();
        history.push(line.clone());
        write_history(&history);

        execute_command(&line, &mut history);
    }
}
