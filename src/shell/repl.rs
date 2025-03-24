use crate::shell::commands::execute_command;
use crate::shell::ctrlc_handler::setup_ctrlc_handler;
use crate::shell::history::load_history;
use hostname::get as get_hostname;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::env;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    setup_ctrlc_handler();

    let mut rl = DefaultEditor::new()?;
    load_history(&mut rl)?;

    let username = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let hostname = get_hostname()
        .unwrap_or_else(|_| "host".into())
        .into_string()
        .unwrap_or_else(|_| "host".to_string());

    loop {
        let prompt = format!("{}@{}> ", username, hostname);
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
