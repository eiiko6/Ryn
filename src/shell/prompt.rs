use chrono::Local;
use colored::Colorize;
use hostname::get as get_hostname;
use humantime::format_duration;
use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::time::Duration;

pub fn parse_prompt(prompt_string: String, time_taken: Option<Duration>) -> String {
    let mut variables = HashMap::new();

    let mut output = prompt_string.clone();

    // Replace ifnotgit variables
    if prompt_string.contains("ifnotgit") {
        let valid_keys = ["user", "host", "dir", "time24"];

        for key in valid_keys {
            let placeholder_ifnotgit = format!("{{{} ifnotgit}}", key);

            if output.contains(&placeholder_ifnotgit) && get_git_info().is_some() {
                // If we're in a git repo, remove the variable
                output = output.replace(&placeholder_ifnotgit, "");
            } else {
                // Otherwise, replace it with the original variable
                output = output.replace(&placeholder_ifnotgit, &format!("{{{}}}", key));
            }
        }
    }

    // Check what variables exist in the input
    if output.contains("{user}") {
        variables.insert(
            "user",
            env::var("USER")
                .unwrap_or_else(|_| "user".to_string())
                .green()
                .to_string(),
        );
    }
    if output.contains("{host}") {
        let hostname = get_hostname()
            .unwrap_or_else(|_| "host".into())
            .into_string()
            .unwrap_or_else(|_| "host".to_string());
        variables.insert("host", hostname.blue().to_string());
    }
    if output.contains("{dir}") {
        if let Some(compact_dir) = get_dir() {
            variables.insert("dir", compact_dir);
        }
    }
    if output.contains("{git}") {
        let git_info = get_git_info().unwrap_or_default();
        variables.insert("git", git_info);
    }
    if output.contains("{time24}") {
        let now = Local::now();

        variables.insert(
            "time24",
            now.format("%H:%M:%S")
                .to_string()
                .bright_yellow()
                .to_string(),
        );
    }
    if output.contains("{timetaken}") {
        let duration_str = match time_taken {
            Some(dur) => {
                if dur < Duration::from_secs(1) {
                    String::new()
                } else {
                    format_duration(Duration::new(dur.as_secs(), 0))
                        .to_string()
                        .yellow()
                        .to_string()
                }
            }
            None => String::default(),
        };

        variables.insert("timetaken", duration_str);
    }

    // Replace variables in prompt
    for (key, value) in variables.iter() {
        // Replace the normal {variable} placeholders
        output = output.replace(&format!("{{{}}}", key), value);
    }

    output.trim_start().replace("  ", " ").to_string()
}

// Function to get directory
fn get_dir() -> Option<String> {
    if let Ok(dir) = env::current_dir() {
        // Convert PathBuf to String
        dir.to_str().map(|s| s.to_string())
    } else {
        None
    }
}

// Function to get Git repository name and status
fn get_git_info() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;

    if let Ok(path) = String::from_utf8(output.stdout) {
        let repo_name = path.trim().rsplit('/').next()?;

        let branch_output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()
            .ok()?;

        // If not a git repo
        if !output.status.success() {
            return None;
        }

        let branch_name = String::from_utf8(branch_output.stdout)
            .ok()?
            .trim()
            .to_string();

        let status_output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .ok()?;

        let status_icon = if status_output.stdout.is_empty() {
            "✔"
        } else {
            "!"
        };

        return Some(format!(
            "{} on {} {}",
            repo_name.cyan(),
            format!(" {}", branch_name).to_string().purple(),
            format!("[{}]", status_icon).to_string().red()
        ));
    }

    None
}
