use clap::{App, Arg};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fun get_args(){
    let matches = App::new("catr")
        .version("0.1.0")
        .author("User01")
        .about("Rust cat")
        .arg()
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    println!("Hello World!");
    Ok(())
}
