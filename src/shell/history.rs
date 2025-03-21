use rustyline::DefaultEditor;
use std::error::Error;

pub fn load_history(rl: &mut DefaultEditor) -> Result<(), Box<dyn Error>> {
    let _ = rl.load_history("history.txt");
    Ok(())
}
