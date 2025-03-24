use chrono::Local;
use hostname::get as get_hostname;
use std::collections::HashMap;
use std::env;
use std::process::Command;

pub fn parse_prompt(prompt_string: String) -> String {
    let mut variables = HashMap::new();

    // Check what variables exist in the input
    if prompt_string.contains("{user}") {
        variables.insert(
            "user",
            env::var("USER").unwrap_or_else(|_| "user".to_string()),
        );
    }
    if prompt_string.contains("{host}") {
        let hostname = get_hostname()
            .unwrap_or_else(|_| "host".into())
            .into_string()
            .unwrap_or_else(|_| "host".to_string());
        variables.insert("host", hostname);
    }
    if prompt_string.contains("{dir}") {
        if let Some(compact_dir) = get_dir() {
            variables.insert("dir", compact_dir);
        }
    }
    if prompt_string.contains("{git}") {
        let git_info = get_git_info().unwrap_or_else(|| "no-git".to_string());
        variables.insert("git", git_info);
    }
    if prompt_string.contains("{time24}") {
        let now = Local::now();

        variables.insert("time24", now.format("%H:%M:%S").to_string());
    }

    // Replace variables in prompt
    let mut output = prompt_string;
    for (key, value) in variables {
        output = output.replace(&format!("{{{}}}", key), &value);
    }

    output
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
            "✗"
        };

        return Some(format!("{} ({}) {}", repo_name, branch_name, status_icon));
    }

    None
}
