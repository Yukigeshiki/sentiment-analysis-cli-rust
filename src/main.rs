extern crate vader_sentiment;

use clap::{Parser, Subcommand};
use colored::Colorize;
use std::fs;

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
/// A CLI tool to perform simple sentiment analysis on provided text
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Performs sentiment analysis on provided text
    Analyse {
        #[arg(short, long)]
        /// The file path to the provided text
        path: String,

        #[arg(short, long)]
        /// The format of the file containing the text
        format: String,
    },
}

fn main() {
    let args = Args::parse();
    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();
    match args.cmd {
        Cmd::Analyse { path, format } => match format.as_str().try_into() {
            Ok(f) => match f {
                Format::Txt => match import_txt(&path) {
                    Ok(contents) => {
                        let analysed = analyzer.polarity_scores(contents.as_str());
                        println!(
                            "{0: <20} | {1: <20} | {2: <20} | {3: <20}",
                            "positive".to_string().bright_green(),
                            "negative".to_string().bright_green(),
                            "neutral".to_string().bright_green(),
                            "compound".to_string().bright_green(),
                        );
                        println!(
                            "{0: <20} | {1: <20} | {2: <20} | {3: <20}",
                            analysed["pos"], analysed["neg"], analysed["compound"], analysed["neu"],
                        );
                    }
                    Err(e) => {
                        eprintln!("{} {e}", "error:".to_string().bright_red())
                    }
                },
            },
            Err(e) => eprintln!("{} {e}", "error:".to_string().bright_red()),
        },
    }
}

enum Format {
    Txt,
    // other formats
}

impl TryFrom<&str> for Format {
    type Error = ErrorKind;

    fn try_from(format: &str) -> Result<Self, Self::Error> {
        match format.to_lowercase().as_str() {
            "txt" => Ok(Self::Txt),
            other => Err(ErrorKind::ConvertToFormat(other.to_string())),
        }
    }
}

fn import_txt(path: &str) -> Result<String, ErrorKind> {
    fs::read_to_string(path).map_err(|e| ErrorKind::ReadToString(e.to_string()))
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ErrorKind {
    #[error("'{0}' is not a supported file format.")]
    ConvertToFormat(String),

    #[error("Text could not be imported: {0}")]
    ReadToString(String),
}

#[cfg(test)]
mod tests {
    use crate::import_txt;
    use std::fs;
    use std::fs::File;

    #[test]
    fn test_txt_import_succeeds() {
        let file_path = "foo.txt";
        File::create(file_path).expect("Error creating file for test.");
        let text = import_txt(file_path).expect("Unable to import text in test");
        fs::remove_file(file_path).expect("Unable to remove file for test.");
        assert_eq!(text, "")
    }

    #[test]
    fn test_txt_import_fails() {
        let e = import_txt("foo.txt").unwrap_err();
        assert_eq!(
            e.to_string(),
            "Text could not be imported: No such file or directory (os error 2)"
        );
    }
}
