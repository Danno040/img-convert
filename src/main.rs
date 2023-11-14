use clap::Parser;
use std::{
    fs,
    io::Cursor,
    process::{ExitCode, Termination},
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    source: String,
    destination: String,
}

#[derive(Debug)]
struct CliResult {
    number: u8,
    msg: String,
}

impl CliResult {
    fn error(msg: String) -> CliResult {
        return CliResult {
            number: 1,
            msg: msg,
        };
    }

    fn success() -> CliResult {
        return CliResult {
            number: 0,
            msg: "Success".to_string(),
        };
    }
}

impl Termination for CliResult {
    fn report(self) -> ExitCode {
        match self.number {
            0 => ExitCode::from(0),
            _ => {
                eprintln!("{}", self.msg);
                ExitCode::from(self.number)
            }
        }
    }
}

fn main() -> CliResult {
    let args = Args::parse();
    println!("Got: {:?}", args);

    // Check that destination doesn't already exist
    if fs::metadata(&args.destination).is_ok() {
        return CliResult::error(format!(
            "{} already exists. Exiting.",
            args.destination.as_str()
        ));
    }

    let read_result = std::fs::read(&args.source);
    if read_result.is_err() {
        return CliResult::error(format!(
            "Error reading {}. Make sure your path is correct.",
            args.source.as_str()
        ));
    }

    let cursor = Cursor::new(read_result.unwrap());
    let img = image::io::Reader::new(cursor)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let result = img.save(args.destination);
    if result.is_err() {
        return CliResult::error(format!(
            "Error writing result: {}",
            result.unwrap_err().to_string()
        ));
    }

    return CliResult::success();
}
