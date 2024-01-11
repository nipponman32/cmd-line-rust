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
                let mut count = 0;
                let mut count_nb = 0;

                for line in hol.lines() {
                    let output = String::from_str(&line.unwrap());
                    let mut empty_line = false;
                    if output.as_ref().unwrap() == "" {
                        empty_line = true;
                    }

                    match (config.number_lines, config.number_nonblank_lines) {
                        (true, _) => println!("{} {}", count, output.unwrap()),
                        (false, true) => {
                            let mut out = String::from("");
                            println!(
                                "{} {}",
                                if empty_line {
                                    out
                                } else {
                                    format!("{} {}", count_nb, output.as_ref().unwrap())
                                },
                                output.unwrap()
                            )
                        }
                        (false, false) => println!("{}", output.unwrap()),
                    }

                    if config.number_nonblank_lines {
                        if empty_line {
                            count_nb += 0;
                        } else {
                            count_nb += 1;
                        }
                    } else {
                        count += 1;
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
