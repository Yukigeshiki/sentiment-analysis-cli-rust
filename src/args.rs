use clap::{Parser, Subcommand};

use crate::{extract_text_from_html, import_file_from_path, make_request, ErrorKind};

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
/// A CLI tool to perform simple sentiment analysis on provided text
pub struct Args {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    #[command(subcommand)]
    Analyse(Type),
}

#[derive(Subcommand)]
pub enum Type {
    /// Performs sentiment analysis on provided HTML
    Html {
        #[arg(short, long)]
        /// A path to an HTML document (this can be a path to a local file or a URL)
        path: String,

        #[arg(short, long)]
        /// A CSS selector for an HTML element containing text
        selector: String,
    },
    /// Performs sentiment analysis on provided text
    Text {
        #[arg(short, long)]
        /// A path to a file containing text
        path: String,
    },
}

impl Args {
    pub fn get_text(self) -> Result<String, ErrorKind> {
        match self.cmd {
            Cmd::Analyse(format) => match format {
                Type::Text { path } => import_file_from_path(&path),
                Type::Html { path, selector } => {
                    if path.starts_with("http") {
                        let html = make_request(path)?;
                        extract_text_from_html(&html, &selector)
                    } else {
                        let html = import_file_from_path(&path)?;
                        extract_text_from_html(&html, &selector)
                    }
                }
            },
        }
    }
}
