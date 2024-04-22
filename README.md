# sentiment-analysis-cli-rust

[![build](https://github.com/Yukigeshiki/sentiment-analysis-cli-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/Yukigeshiki/sentiment-analysis-cli-rust/actions/workflows/ci.yml)

A CLI tool to perform simple sentiment analysis written in Rust, using a Rust port of [VADER](https://github.com/ckw017/vader-sentiment-rust).

Analysis can be performed on a text file or text within an HTML file. For HTML, you can supply a path to a file locally, or you can scrape HTML from the web. Currently, a CSS selector is used to specify the HTML element which contains the required text, but I hope to add xpath functionality in the future too.

### How to run:

First make sure you have Rust installed. To do this you can follow the instructions found [here](https://www.rust-lang.org/tools/install).

Clone the repo, cd into it and run:

```bash
cargo build --release
```

To run for a text file:


```bash
./target/release/sentiment analyse text -p path/to/file/foo.txt
```

To run for an HTML file:

```bash
./target/release/sentiment analyse html -p path/to/file/foo.html -s "div > div > p"
```

To scrape and run for a webpage:

```bash
./target/release/sentiment analyse html -p https://page-to-scrape.com -s "div > div > p"
```

For more info about the CLI tool, run:

```bash
./target/release/sentiment help
```

or

```bash
./target/release/sentiment analyse help
```
