use clap::{App, Arg};
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

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
            Arg::with_name("number")
                .long("number")
                .help("Number lines")
                .conflicts_with("number_nonblank")
                .short("n")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .long("number-nonblank")
                .help("Number of lines with text")
                .short("b")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("Files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        let h = open(&filename);
        match h {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(hol) => {
                let mut last_num = 0;
                for (line_num, line_result) in hol.lines().enumerate() {
                    let line = line_result?;

                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
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
