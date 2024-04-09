use std::fs;

use scraper::{Html, Selector};

pub mod args;

fn parse_html(html: &str, selector: &str) -> Result<String, ErrorKind> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(selector).map_err(|e| ErrorKind::ParseHtml(e.to_string()))?;
    match document.select(&selector).next() {
        Some(item) => Ok(item.text().collect::<Vec<_>>().concat()),
        None => Err(ErrorKind::ParseHtml(
            "No text available at selector".to_string(),
        )),
    }
}

fn import_from_file_path(path: &str) -> Result<String, ErrorKind> {
    fs::read_to_string(path).map_err(|e| ErrorKind::ReadToString(e.to_string()))
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error("Error making request to '{0}'. {1}")]
    Request(String, String),

    #[error("Error decoding HTML. {0}")]
    Decode(String),

    #[error("Error fetching file from file system. {0}")]
    ReadToString(String),

    #[error("Error parsing HTML. {0}")]
    ParseHtml(String),
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;

    use crate::{import_from_file_path, parse_html};

    #[test]
    fn test_parse_html_is_success() {
        let html = r#"
                <html>
                    <body>
                        <div id="example">
                            <p>Hello, world!</p>
                        </div>
                    </body>
                </html>
            "#;
        let selector = "div#example p";
        let text = parse_html(html, selector).expect("Unable to parse HTML");
        assert_eq!(text, "Hello, world!")
    }

    #[test]
    fn test_parse_html_fails() {
        let html = r#"
                <html>
                    <body>
                        <div id="example">
                            <p>Hello, world!</p>
                        </div>
                    </body>
                </html>
            "#;
        let selector = "div#example a";
        let e = parse_html(html, selector).unwrap_err();
        assert_eq!(
            e.to_string(),
            "Error parsing HTML. No text available at selector"
        )
    }

    #[test]
    fn test_import_from_file_path_is_succeeds() {
        let file_path = "foo.txt";
        File::create(file_path).expect("Error creating file for test");
        let text = import_from_file_path(file_path).expect("Unable to import text in test");
        fs::remove_file(file_path).expect("Unable to remove file for test");
        assert_eq!(text, "")
    }

    #[test]
    fn test_import_from_file_path_fails() {
        let e = import_from_file_path("foo.txt").unwrap_err();
        assert_eq!(
            e.to_string(),
            "Error fetching file from file system. No such file or directory (os error 2)"
        );
    }
}
