use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        print!("minimal-shell> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let input = match read_input() {
            Some(input) => input,
            None => continue,
        };

        let args = parse_command(&input);
        if args.is_empty() {
            continue;
        }

        if args[0] == "exit" {
            break;
        }

        execute_command(&args);
    }
}

fn read_input() -> Option<String> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim().to_string();
        if !trimmed.is_empty() {
            return Some(trimmed);
        }
    }
    None
}

fn parse_command(input: &str) -> Vec<String> {
    input.split_whitespace().map(String::from).collect()
}

fn execute_command(args: &[String]) {
    if let Some((command, args)) = args.split_first() {
        let status = Command::new(command).args(args).status();
        if let Err(err) = status {
            eprintln!("Error executing command: {}", err);
        }
    }
}
