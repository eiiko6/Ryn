use std::process::{Command, Stdio};

#[derive(Debug)]
pub enum CommandExpr {
    Sequence(Vec<CommandExpr>),              // a ; b ; c
    Pipeline(Vec<CommandExpr>),              // a | b | c
    And(Box<CommandExpr>, Box<CommandExpr>), // a && b
    Or(Box<CommandExpr>, Box<CommandExpr>),  // a || b
    // Redirect {
    //     command: Box<CommandExpr>,
    //     kind: RedirectKind,
    //     target: String,
    // },
    // Background(Box<CommandExpr>), // a &
    Command(Vec<String>), // basic command + args
}

#[cfg(windows)]
fn is_builtin(cmd: &str) -> bool {
    matches!(
        cmd,
        "echo" | "dir" | "cd" | "cls" | "set" | "pause" | "type"
    )
}

pub fn execute_command(args: &[String]) -> bool {
    if let Some((command, rest)) = args.split_first() {
        #[cfg(windows)]
        let result = if is_builtin(command) {
            let mut full = vec![command.clone()];
            full.extend(rest.iter().cloned());
            Command::new("cmd").arg("/C").args(&full).status()
        } else {
            Command::new(command).args(rest).status()
        };

        #[cfg(not(windows))]
        let result = Command::new(command).args(rest).status();

        match result {
            Ok(status) => status.success(),
            Err(err) => {
                eprintln!("error executing command: {}", err);
                false
            }
        }
    } else {
        false
    }
}

pub fn spawn_command(
    args: &[String],
    stdin: Stdio,
    stdout: Stdio,
    stderr: Option<Stdio>,
) -> Result<std::process::Child, std::io::Error> {
    if args.is_empty() {
        eprintln!("error: Attempted to spawn an empty command.");
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Empty command",
        ));
    }

    #[cfg(windows)]
    let (command, cmd_args): (&str, Vec<String>) = {
        let (c, rest) = args.split_first().unwrap();
        if is_builtin(c) {
            let mut cmdline = vec![c.clone()];
            cmdline.extend(rest.iter().cloned());
            (
                "cmd",
                vec!["/C".into()].into_iter().chain(cmdline).collect(),
            )
        } else {
            (c.as_str(), rest.to_vec())
        }
    };

    #[cfg(not(windows))]
    let (command, cmd_args): (&str, Vec<String>) = {
        let (c, rest) = args.split_first().unwrap();
        (c.as_str(), rest.to_vec())
    };

    let mut cmd = Command::new(command);
    cmd.args(cmd_args).stdin(stdin).stdout(stdout);

    if let Some(err) = stderr {
        cmd.stderr(err);
    }

    cmd.spawn()
}
