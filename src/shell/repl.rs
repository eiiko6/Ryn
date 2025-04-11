use crate::shell::config::load_config;
use crate::shell::history::{load_history, save_history, setup_history};
use crate::shell::parser::parse_and_execute;
use crate::shell::prompt::parse_prompt;
use std::io::{self, Write};

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    setup_ctrlc_handler();

    let mut rl = DefaultEditor::new()?;
    let history = setup_history()?;
    load_history(&mut rl, &history)?;

    let config = load_config()?;
    let prompt_string = config.prompt;

    loop {
        let prompt = parse_prompt(prompt_string.clone());
        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line).ok();
                if let Some(should_exit) = parse_and_execute(line) {
                    if should_exit {
                        break;
                    }
                }
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

pub fn setup_ctrlc_handler() {
    ctrlc::set_handler(move || {
        print!("\nminimal-shell> ");
        io::stdout().flush().unwrap();
    })
    .expect("Error setting Ctrl-C handler");
}
