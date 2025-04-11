use crate::shell::builtin::handle_builtin;
use crate::shell::commands::execute_command;

pub fn parse_and_execute(input: &str) -> Option<bool> {
    let args = parse_input(input);

    if args.is_empty() {
        return Some(false);
    }

    if let Some(result) = handle_builtin(&args) {
        return Some(result);
    }

    execute_command(&args);
    Some(false)
}

fn parse_input(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_token = String::new();
    let mut inside_quotes = false;

    for char in input.chars() {
        match char {
            '"' => {
                if inside_quotes {
                    result.push(current_token.clone());
                    current_token.clear();
                    inside_quotes = false;
                } else {
                    inside_quotes = true;
                }
            }
            ' ' => {
                if inside_quotes {
                    current_token.push(char);
                } else if !current_token.is_empty() {
                    result.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(char);
            }
        }
    }

    if !current_token.is_empty() {
        result.push(current_token);
    }

    result
}
