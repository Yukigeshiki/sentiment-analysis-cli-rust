use clap::{Parser, Subcommand};

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
    Analyse(Format),
}

#[derive(Subcommand)]
pub enum Format {
    /// Performs sentiment analysis on provided HTML
    Html {
        #[arg(short, long)]
        /// A path to an HTML document (this can be a path to a local file or a URL)
        path: String,

        #[arg(short, long)]
        /// A selector for an HTML element containing text
        selector: String,
    },
    /// Performs sentiment analysis on provided text
    Text {
        #[arg(short, long)]
        /// A path to a file containing text
        path: String,
    },
}
