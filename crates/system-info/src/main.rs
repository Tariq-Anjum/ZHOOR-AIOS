use std::io::{self, Read, Write};

use zhoor_system_info::{CompletionStatus, Engine, MAX_REQUEST_BYTES};

fn main() {
    if std::env::args_os().len() != 1 {
        eprintln!("this capability accepts one JSON request on stdin and no command arguments");
        std::process::exit(2);
    }

    let mut input = Vec::new();
    if io::stdin()
        .take((MAX_REQUEST_BYTES + 1) as u64)
        .read_to_end(&mut input)
        .is_err()
    {
        eprintln!("failed to read request");
        std::process::exit(2);
    }

    let output = Engine::system_info().process(&input);
    let succeeded = output.response.status == CompletionStatus::Completed;
    let mut stdout = io::stdout().lock();
    if serde_json::to_writer(&mut stdout, &output).is_err() || stdout.write_all(b"\n").is_err() {
        eprintln!("failed to write response");
        std::process::exit(2);
    }
    if !succeeded {
        std::process::exit(2);
    }
}
