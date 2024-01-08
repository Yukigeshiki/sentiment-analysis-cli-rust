use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
/// A CLI tool to perform sentiment analysis on a provided text file
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Performs sentiment analysis on a provided text file
    Analyse {
        #[arg(short, long)]
        /// The file path to the provided text file
        path: String,

        #[arg(short, long)]
        /// The format of the text file
        format: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Cmd::Analyse { path, format } => {
            println!("{path}, {format}");
        }
    }
}
