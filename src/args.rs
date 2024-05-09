use std::fs;

use clap::{Parser, Subcommand};
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

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
                Type::Text { path } => Self::import_file_from_path(&path),
                Type::Html { path, selector } => {
                    if path.starts_with("http") {
                        let html = Self::fetch_html_from_site(path)?;
                        Self::extract_text_from_html(&html, &selector)
                    } else {
                        let html = Self::import_file_from_path(&path)?;
                        Self::extract_text_from_html(&html, &selector)
                    }
                }
            },
        }
    }

    fn import_file_from_path(path: &str) -> Result<String, ErrorKind> {
        fs::read_to_string(path).map_err(|e| ErrorKind::ReadToString(e.to_string()))
    }

    fn fetch_html_from_site(address: String) -> Result<String, ErrorKind> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(&address)
            .header(
                USER_AGENT,
                "Mozilla/5.0 (iPad; CPU OS 12_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Mobile/15E148",
            )
            .send()
            .map_err(|e| ErrorKind::Request(address.clone(), e.to_string()))?;
        if !response.status().is_success() {
            Err(ErrorKind::Request(
                address,
                format!("Request failed with code {}", response.status().as_u16()),
            ))?;
        }
        response
            .text()
            .map_err(|e| ErrorKind::Decode(e.to_string()))
    }

    fn extract_text_from_html(html: &str, selector: &str) -> Result<String, ErrorKind> {
        let document = Html::parse_document(html);
        let selector =
            Selector::parse(selector).map_err(|e| ErrorKind::ParseHtml(e.to_string()))?;
        match document.select(&selector).next() {
            Some(item) => Ok(item.text().collect::<Vec<_>>().concat()),
            None => Err(ErrorKind::ParseHtml(
                "No text available at selector".to_string(),
            )),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Error making request to '{0}'. {1}")]
    Request(String, String),

    #[error("Error decoding HTML. {0}")]
    Decode(String),

    #[error("Error importing file from file system. {0}")]
    ReadToString(String),

    #[error("Error parsing HTML. {0}")]
    ParseHtml(String),
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
    fn test_extract_text_from_html_is_success() {
        let html = supply_test_html();
        let selector = "div#example p";
        let text = Args::extract_text_from_html(html, selector).expect("Unable to parse HTML");
        assert_eq!(text, "Hello, world!")
    }

    #[test]
    fn test_extract_text_from_html_is_failure() {
        let html = supply_test_html();
        let selector = "div#example a";
        let e = Args::extract_text_from_html(html, selector).unwrap_err();
        assert_eq!(
            e.to_string(),
            "Error parsing HTML. No text available at selector"
        )
    }

    #[test]
    fn test_import_file_from_path_is_success() {
        let file_path = "foo.txt";
        File::create(file_path).expect("Unable to create file for test");
        let text = Args::import_file_from_path(file_path).expect("Unable to import text in test");
        fs::remove_file(file_path).expect("Unable to remove file for test");
        assert_eq!(text, "")
    }

    #[test]
    fn test_import_file_from_path_is_failure() {
        let e = Args::import_file_from_path("foo.txt").unwrap_err();
        assert_eq!(
            e.to_string(),
            "Error importing file from file system. No such file or directory (os error 2)"
        );
    }

    #[tokio::test]
    async fn test_fetch_html_from_site_is_success() {
        let mock_server = MockServer::start().await;

        Mock::given(any())
            .and(path("/".to_owned()))
            .and(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(supply_test_html(), "text"))
            .expect(1)
            .mount(&mock_server)
            .await;

        thread::spawn(move || {
            let html = Args::fetch_html_from_site(mock_server.uri()).expect("Unable to get HTML");
            let selector = "div#example p";
            let text = Args::extract_text_from_html(&html, selector).expect("Unable to parse HTML");
            assert_eq!(text, "Hello, world!")
        })
        .join()
        .expect("Unable to make request");
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
