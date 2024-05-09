use clap::Parser;
use colored::Colorize;
use comfy_table::{Cell, Color, Table};

use crate::args::Args;

mod args;

fn main() {
    let args = Args::parse();
    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();

    match args.get_text() {
        Ok(text) => {
            let analysed = analyzer.polarity_scores(&text);
            let mut table = Table::new();
            table
                .set_header(vec![
                    Cell::new("Positive").fg(Color::Green),
                    Cell::new("Negative").fg(Color::Green),
                    Cell::new("Neutral").fg(Color::Green),
                    Cell::new("Compound").fg(Color::Green),
                ])
                .add_row(vec![
                    analysed["pos"],
                    analysed["neg"],
                    analysed["neu"],
                    analysed["compound"],
                ]);
            println!("{table}");
        }
        Err(e) => {
            eprintln!("{} {e}", "Error:".to_string().bright_red())
        }
    }
}
