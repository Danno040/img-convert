use clap::Parser;
use image::imageops;
use std::{
    fs,
    io::Cursor,
    process::{ExitCode, Termination},
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // If set, will resize the image to given dimentions. Format is wxh, e.g. 1080x720
    #[arg(short)]
    resize: Option<String>,
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
    // println!("Got: {:?}", args);

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

    if args.resize.is_some() {
        let resize = args.resize.unwrap();
        let parts = Vec::from_iter(resize.split("x"));

        let height: u32;
        let width: u32;

        if parts.len() != 2 {
            return CliResult::error(format!("Got an invalid resize string: {}", resize))
        }

        if parts[0].is_empty() {
            // Do proportional scaling of width
            height = parts[1].parse().unwrap();
            width = (height * img.width()) / img.height();
        } else if parts[1].is_empty() {
            // Do proportional scaling of height
            width = parts[0].parse().unwrap();
            height = (width * img.height()) / img.width();
        } else {
            // Do normal scaling
            width = parts[0].parse().unwrap();
            height = parts[1].parse().unwrap();
        }

        let result = imageops::resize(&img, width, height, image::imageops::FilterType::CatmullRom).save(args.destination);
        if result.is_err() {
            return CliResult::error(format!(
                "Error writing result: {}",
                result.unwrap_err().to_string()
            ));
        }

        return CliResult::success();
    }

    let result = img.save(args.destination);
    if result.is_err() {
        return CliResult::error(format!(
            "Error writing result: {}",
            result.unwrap_err().to_string()
        ));
    }

    return CliResult::success();
}
