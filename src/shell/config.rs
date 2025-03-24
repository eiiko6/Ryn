use std::{fs, io};

pub struct Config {
    pub prompt: String,
}

pub fn load_config() -> Result<Config, io::Error> {
    // Try to get the config directory
    let config_path = match dirs::config_dir() {
        Some(path) => {
            let mut path_string = path
                .to_str()
                .unwrap_or_else(|| panic!("Failed to convert config path to string"))
                .to_string();
            path_string.push_str("/ryn/config");
            path_string
        }
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Config directory not found",
            ));
        }
    };

    // Try to read the config file
    let contents = fs::read_to_string(&config_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Failed to read config file: {}", e),
        )
    })?;

    // Try to find a line starting with "prompt = "
    let prompt_line = contents
        .lines()
        .find(|line| line.starts_with("prompt = "))
        .unwrap_or("prompt = {user}@{home}> ");

    // Extract the prompt value
    let prompt = prompt_line[9..].trim_matches('"').to_string();

    // Return the Config struct
    Ok(Config { prompt })
}
