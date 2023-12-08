use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        eprintln!("Usage: {} <repository> <from> <to> <file>", args[0]);
        return ExitCode::FAILURE;
    }

    let repository = &args[1];
    let from = &args[2];
    let to = &args[3];
    let file = &args[4];

    println!("Repository: {}", repository);
    println!("From: {}", from);
    println!("To: {}", to);
    println!("File: {}", file);

    return ExitCode::SUCCESS;
}
