use std::process::Command;

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
