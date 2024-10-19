use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Input file(s) [default: -1]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short = 'c', long)]
    bytes: bool,

    /// Show character count
    #[arg(short = 'm', long)]
    chars: bool,
}

impl Config {
    pub fn get_args() -> MyResult<Self> {
        let mut config = Config::parse();
        if [config.lines, config.words, config.bytes, config.chars]
            .iter()
            .all(|v| v == &false)
        {
            config.lines = true;
            config.words = true;
            config.bytes = true;
        }
        Ok(config)
    }

    pub fn run(&self) -> MyResult<()> {
        for filename in &self.files {
            match open(filename) {
                Err(err) => eprintln!("{}: {}", filename, err),
                Ok(file) => {
                    if let Ok(info) = count(file) {
                        println!(
                            "{}{}{}{}{}",
                            format_field(info.num_lines, self.lines),
                            format_field(info.num_words, self.words),
                            format_field(info.num_bytes, self.bytes),
                            format_field(info.num_chars, self.chars),
                            if filename == "-" {
                                "".to_string()
                            } else {
                                format!(" {}", filename)
                            }
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();
    while file.read_line(&mut line).unwrap_or(0) > 0 {
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += line.as_bytes().len();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
