use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("User01")
        .about("Rust cat")
        .arg(
            Arg::with_name("Files")
                .value_name("FILE")
                .help("Input files")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .value_name("NUMBER_LINES")
                .help("Show number lines")
                .required(false)
                .short("n")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .value_name("NUMBER_NON_BLANK_LINES")
                .help("Number of lines with text")
                .short("b")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    let files_arg = matches.values_of_lossy("Files").unwrap();
    let number_lines_arg = matches.is_present("NUMBER_LINES");
    let blank_number_lines_arg = matches.is_present("NUMBER_NON_BLANK_LINES");

    Ok(Config {
        files: files_arg,
        number_lines: number_lines_arg,
        number_nonblank_lines: blank_number_lines_arg,
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
