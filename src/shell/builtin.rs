use std::env;

pub fn handle_builtin(args: &[String]) -> Option<bool> {
    match args.first().map(String::as_str) {
        Some("exit") => Some(true),
        Some("cd") => {
            let new_dir = if args.len() > 1 {
                args[1].clone()
            } else if let Some(path) = dirs::home_dir() {
                path.to_string_lossy().to_string()
            } else {
                "/".to_string()
            };

            if let Err(err) = env::set_current_dir(&new_dir) {
                eprintln!("cd: {}: {}", new_dir, err);
            }

            Some(false)
        }
        _ => None,
    }
}
