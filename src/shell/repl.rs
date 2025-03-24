use crate::shell::commands::execute_command;
use crate::shell::config::load_config;
use crate::shell::ctrlc_handler::setup_ctrlc_handler;
use crate::shell::history::{load_history, save_history, setup_history};
use crate::shell::prompt::parse_prompt;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::env;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    setup_ctrlc_handler();

    let mut rl = DefaultEditor::new()?;
    let history = setup_history()?;
    load_history(&mut rl, &history)?;

    let config = load_config()?;
    let prompt = parse_prompt(config.prompt);

    loop {
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line).ok();
                let args = parse_input(line);

                if args[0] == "exit" {
                    break;
                }

                if args[0] == "cd" {
                    let new_dir = if let Some(path) = dirs::home_dir() {
                        if let Some(p) = path.to_str() {
                            p.to_string()
                        } else {
                            "/".to_string()
                        }
                    } else {
                        "/".to_string()
                    };

                    if let Err(err) = env::set_current_dir(&new_dir) {
                        eprintln!("cd: {}: {}", new_dir, err);
                    }
                    continue;
                }

                execute_command(&args);
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            Err(err) => {
                eprintln!("Error reading input: {:?}", err);
                break;
            }
        }
    }

    save_history(&mut rl, &history)?;

    Ok(())
}

pub fn parse_input(input: &str) -> Vec<String> {
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
