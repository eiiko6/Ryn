mod shell;

fn main() {
    if let Err(err) = shell::run() {
        eprintln!("Shell exited with error: {}", err);
    }
}
