use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Input file(s) [default: -]
    files: Vec<String>,

    /// Number lines
    #[arg(short, long)]
    number_lines: bool,

    /// Number nonblank lines
    #[arg(short = 'b', long)]
    number_nonblank_lines: bool,
}

impl Config {
    pub fn get_args() -> MyResult<Self> {
        Ok(Self::parse())
    }

    pub fn run(&self) -> MyResult<()> {
        for filename in &self.files {
            match open(filename) {
                Err(err) => eprintln!("Failed to open {}: {}", filename, err),
                Ok(file) => self.print_lines(file)?,
            }
        }
        Ok(())
    }

    fn print_lines(&self, file: Box<dyn BufRead>) -> MyResult<()> {
        let mut last_num = 0;
        for (line_num, line) in file.lines().enumerate() {
            let line = line?;
            if self.number_lines {
                println!("{:>6}\t{}", line_num + 1, line);
            } else if self.number_nonblank_lines {
                if !line.is_empty() {
                    last_num += 1;
                    println!("{:>6}\t{}", last_num, line);
                }
            } else {
                println!("{}", line);
            }
        }
        Ok(())
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
