use crate::shell::completion::RynHelper;
use crate::shell::config::load_config;
use crate::shell::history::{load_history, save_history, setup_history};
use crate::shell::parser::parse_and_execute;
use crate::shell::prompt::parse_prompt;
use std::io::{self, Write};
use std::time::Instant;

use rustyline::Editor;
use rustyline::config::{ColorMode, Config};
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    setup_ctrlc_handler();

    // Setup rustyline
    let config = Config::builder()
        .color_mode(ColorMode::Forced)
        .completion_type(rustyline::config::CompletionType::List) // maybe circular in config later
        .build();

    let mut rl = Editor::<RynHelper, FileHistory>::with_config(config)?;
    rl.set_helper(Some(RynHelper::new()));

    let history = setup_history()?;
    load_history(&mut rl, &history)?;

    let config = load_config()?;
    let prompt_string = config.prompt;

    let mut last_duration = None;

    loop {
        let prompt = parse_prompt(prompt_string.clone(), last_duration);

        print!("{}", config.cursor.to_ansi_code());
        io::stdout().flush().unwrap();

        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line).ok();

                let start_time = Instant::now();

                match parse_and_execute(line, &config.aliases) {
                    Ok(value) => {
                        if value {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                }

                last_duration = Some(start_time.elapsed());
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
        io::stdout().flush().unwrap();
    })
    .expect("Error setting Ctrl-C handler");
}
