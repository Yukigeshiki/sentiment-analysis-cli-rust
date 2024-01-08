# sentiment-analysis-cli-rust

[![build](https://github.com/Yukigeshiki/sentiment-analysis-cli-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/Yukigeshiki/sentiment-analysis-cli-rust/actions/workflows/ci.yml)

A CLI tool to perform simple sentiment analysis written in Rust, using the Rust port of [VADER](https://github.com/ckw017/vader-sentiment-rust).

### How to run:

First make sure you have Rust installed. To do this you can follow the instructions found [here](https://www.rust-lang.org/tools/install).

Clone the repo, cd into it and run:

```bash
cargo build --release
```

Then run:

```bash
./target/release/sentiment analyse --path foo.txt --format txt
```

For more info about the CLI tool, run:

```bash
./target/release/sentiment help
```

or

```bash
./target/release/sentiment help analyse
```
