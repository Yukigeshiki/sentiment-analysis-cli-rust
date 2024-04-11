use clap::{Parser, Subcommand};

use crate::{extract_text_from_html, fetch_html_from_site, import_file_from_path, ErrorKind};

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
                        let html = fetch_html_from_site(path)?;
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

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::{fs, thread};

    use wiremock::matchers::{any, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::args::Args;
    use crate::args::Cmd::Analyse;
    use crate::args::Type::{Html, Text};

    fn supply_test_html() -> &'static str {
        r#"
            <html>
                <body>
                    <div id="example">
                        <p>Hello, world!</p>
                    </div>
                </body>
            </html>
        "#
    }

    #[test]
    fn test_get_text_for_text_file() {
        let file_path = "bar.txt";
        let mut buffer = File::create(file_path).expect("Unable to create file for test");
        buffer
            .write_all(b"Hello world!")
            .expect("Unable to write to file");
        let args = Args {
            cmd: Analyse({
                Text {
                    path: "bar.txt".to_string(),
                }
            }),
        };
        let text = args.get_text().expect("Unable to get text");
        fs::remove_file(file_path).expect("Unable to remove file for test");
        assert_eq!(text, "Hello world!")
    }

    #[test]
    fn test_get_text_for_html_file() {
        let file_path = "bar.html";
        let mut buffer = File::create(file_path).expect("Unable to create file for test");
        buffer
            .write_all(supply_test_html().as_bytes())
            .expect("Unable to write to file");
        let args = Args {
            cmd: Analyse({
                Html {
                    path: "bar.html".to_string(),
                    selector: "div#example p".to_string(),
                }
            }),
        };
        let text = args.get_text().expect("Unable to get text");
        fs::remove_file(file_path).expect("Unable to remove file for test");
        assert_eq!(text, "Hello, world!")
    }

    #[tokio::test]
    async fn test_get_text_for_site() {
        let mock_server = MockServer::start().await;
        let file_path = "bar.html";
        let mut buffer = File::create(file_path).expect("Unable to create file for test");
        buffer
            .write_all(supply_test_html().as_bytes())
            .expect("Unable to write to file");
        let args = Args {
            cmd: Analyse({
                Html {
                    path: mock_server.uri(),
                    selector: "div#example p".to_string(),
                }
            }),
        };

        Mock::given(any())
            .and(path("/".to_owned()))
            .and(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(supply_test_html(), "text"))
            .expect(1)
            .mount(&mock_server)
            .await;

        thread::spawn(move || {
            let text = args.get_text().expect("Unable to get text");
            fs::remove_file(file_path).expect("Unable to remove file for test");
            assert_eq!(text, "Hello, world!")
        })
        .join()
        .expect("Unable to make request");
    }
}
