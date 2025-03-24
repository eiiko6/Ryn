use rustyline::DefaultEditor;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;

pub struct HistorySetup {
    path: String,
}

pub fn setup_history() -> Result<HistorySetup, Box<dyn Error>> {
    match dirs::data_local_dir() {
        Some(path) => {
            let path = path.join("ryn");
            fs::create_dir_all(&path)?;
            let mut path_string = path
                .to_str()
                .unwrap_or_else(|| panic!("Failed to convert data path to string"))
                .to_string();
            path_string.push_str("/history.txt");
            Ok(HistorySetup { path: path_string })
        }
        None => Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "Data directory not found",
        ))),
    }
}

pub fn load_history(rl: &mut DefaultEditor, history: &HistorySetup) -> Result<(), Box<dyn Error>> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(&history.path)?;

    rl.load_history(&history.path)?;
    Ok(())
}

pub fn save_history(rl: &mut DefaultEditor, history: &HistorySetup) -> Result<(), Box<dyn Error>> {
    // Save the history to the file
    rl.save_history(&history.path)?;

    Ok(())
}
