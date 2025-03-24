use crate::shell::commands::execute_command;
use crate::shell::config::load_config;
use crate::shell::ctrlc_handler::setup_ctrlc_handler;
use crate::shell::history::load_history;
use crate::shell::prompt::parse_prompt;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::env;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    setup_ctrlc_handler();

    let mut rl = DefaultEditor::new()?;
    load_history(&mut rl)?;

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
                let args: Vec<String> = line.split_whitespace().map(String::from).collect();

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

    if let Err(err) = rl.save_history("history.txt") {
        eprintln!("Failed to save history: {}", err);
    }

    Ok(())
}
