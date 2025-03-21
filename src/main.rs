use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::error::Error;
use std::io::Write;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> Result<(), Box<dyn Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Handle Ctrl+C
    ctrlc::set_handler(move || {
        print!("\nminimal-shell> ");
        std::io::stdout().flush().unwrap();
    })
    .expect("Error setting Ctrl-C handler");

    let mut rl = DefaultEditor::new()?;
    let _ = rl.load_history("history.txt");

    while r.load(Ordering::SeqCst) {
        let readline = rl.readline("minimal-shell> ");
        match readline {
            Ok(line) => {
                let line = line.trim().to_string();
                if line.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(line.as_str());

                let args: Vec<String> = line.split_whitespace().map(String::from).collect();
                if args.is_empty() {
                    continue;
                }

                if args[0] == "exit" {
                    break;
                }

                execute_command(&args);
            }
            Err(ReadlineError::Interrupted) => {
                // Ignore Ctrl+C
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Handle Ctrl+D (exit)
                println!("exit");
                break;
            }
            Err(err) => {
                eprintln!("Error reading input: {:?}", err);
                break;
            }
        }
    }

    rl.save_history("history.txt").ok();
    Ok(())
}

fn execute_command(args: &[String]) {
    if let Some((command, args)) = args.split_first() {
        let status = Command::new(command).args(args).status();
        if let Err(err) = status {
            eprintln!("Error executing command: {}", err);
        }
    }
}
