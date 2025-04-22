use colored::*;
use std::fmt;
use std::str::FromStr;
use std::{fs, io};

pub struct Config {
    pub prompt: String,
    pub cursor: CursorStyle,
}

pub enum CursorStyle {
    BlinkingBlock,     // 0 or 1
    SteadyBlock,       // 2
    BlinkingUnderline, // 3
    SteadyUnderline,   // 4
    BlinkingBar,       // 5
    SteadyBar,         // 6
}

impl CursorStyle {
    pub fn to_ansi_code(&self) -> String {
        match self {
            CursorStyle::BlinkingBlock => "\x1b[1 q".to_string(),
            CursorStyle::SteadyBlock => "\x1b[2 q".to_string(),
            CursorStyle::BlinkingUnderline => "\x1b[3 q".to_string(),
            CursorStyle::SteadyUnderline => "\x1b[4 q".to_string(),
            CursorStyle::BlinkingBar => "\x1b[5 q".to_string(),
            CursorStyle::SteadyBar => "\x1b[6 q".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prompt: "{time24} {user ifnotgit} {host ifnotgit}{git} > ".into(),
            cursor: CursorStyle::BlinkingBar,
        }
    }
}

impl FromStr for CursorStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "blinkingblock" => Ok(CursorStyle::BlinkingBlock),
            "steadyblock" => Ok(CursorStyle::SteadyBlock),
            "blinkingunderline" => Ok(CursorStyle::BlinkingUnderline),
            "steadyunderline" => Ok(CursorStyle::SteadyUnderline),
            "blinkingbar" => Ok(CursorStyle::BlinkingBar),
            "steadybar" => Ok(CursorStyle::SteadyBar),
            _ => Err(()),
        }
    }
}

struct SyntaxError {
    line_number: usize,
    line: String,
    message: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "config: {} on line {}:\n  {}\n  {}\n{}",
            "Syntax error".red().bold(),
            self.line_number,
            self.line,
            "^".repeat(self.line.len()).bright_red(),
            self.message
        )
    }
}

pub fn load_config() -> Result<Config, io::Error> {
    let config_path = match dirs::config_dir() {
        Some(mut path) => {
            path.push("ryn/config");
            path
        }
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Config directory not found",
            ));
        }
    };

    let contents = fs::read_to_string(&config_path).unwrap_or_default();
    let mut config = Config::default();

    for (i, line) in contents.lines().enumerate() {
        let line_number = i + 1;
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = trimmed.splitn(2, '=').collect();

        if parts.len() != 2 {
            print_syntax_error(SyntaxError {
                line_number,
                message: format!("Expected {}", "key = value".bold()),
                line: trimmed.to_string(),
            });
            return Ok(Config::default());
        }

        let key = parts[0].trim();
        let value = parts[1].trim().trim_matches('"');

        match key {
            "prompt" => config.prompt = value.to_string(),
            "cursor" => match value.parse::<CursorStyle>() {
                Ok(style) => config.cursor = style,
                Err(_) => {
                    print_syntax_error(SyntaxError {
                        line_number,
                        message: format!("Invalid cursor style: '{}'", value),
                        line: trimmed.to_string(),
                    });
                    return Ok(Config::default());
                }
            },
            _ => {
                print_syntax_error(SyntaxError {
                    line_number,
                    message: format!("Unknown config key: '{}'", key),
                    line: trimmed.to_string(),
                });
                return Ok(Config::default());
            }
        }
    }

    Ok(config)
}

fn print_syntax_error(err: SyntaxError) {
    eprintln!("{err}");
}
