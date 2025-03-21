use std::io::{self, Write};

pub fn setup_ctrlc_handler() {
    ctrlc::set_handler(move || {
        print!("\nminimal-shell> ");
        io::stdout().flush().unwrap();
    })
    .expect("Error setting Ctrl-C handler");
}
