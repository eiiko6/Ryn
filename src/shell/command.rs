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

pub fn execute_command(args: &[String]) -> bool {
    if let Some((command, args)) = args.split_first() {
        match Command::new(command).args(args).status() {
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

    let (command, cmd_args) = args.split_first().unwrap();
    let mut cmd = Command::new(command);
    cmd.args(cmd_args).stdin(stdin).stdout(stdout);

    if let Some(err) = stderr {
        cmd.stderr(err);
    }

    cmd.spawn()
}
