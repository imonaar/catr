use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Parser, Debug)]
#[command(name = "catr")]
#[command(version = "0.1.0")]
#[command(author = "Kevin Monari")]
#[command(about = "concatenate and print files")]
pub struct Config {
    #[arg(num_args=0.., default_value="-", help="input file(s)")]
    files: Vec<String>,

    #[arg(
        short = 'n',
        long = "number",
        conflicts_with = "number_nonblank_lines",
        help = "Number the output lines, starting at 1."
    )]
    number_lines: bool,

    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "Number the non-blank output lines, starting at 1."
    )]
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();
    Ok(config)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(buff_file) => {
                let mut i = 0;
                for line_result in buff_file.lines() {
                    let line = line_result?;

                    if config.number_lines {
                        println!("{:>6}\t{}", i + 1, line)
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                            continue;
                        }
                        println!("{:>6}\t{}", i + 1, line)
                    } else {
                        println!("{}", line)
                    }
                    i += 1;
                }
            }
        }
    }
    Ok(())
}
