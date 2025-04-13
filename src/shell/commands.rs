use std::process::Command;

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
