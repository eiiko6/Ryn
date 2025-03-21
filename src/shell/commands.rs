use std::process::Command;

pub fn execute_command(args: &[String]) {
    if let Some((command, args)) = args.split_first() {
        let status = Command::new(command).args(args).status();
        if let Err(err) = status {
            eprintln!("Error executing command: {}", err);
        }
    }
}
