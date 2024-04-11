use std::fs;

use scraper::{Html, Selector};

pub mod args;

fn extract_text_from_html(html: &str, selector: &str) -> Result<String, ErrorKind> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(selector).map_err(|e| ErrorKind::ParseHtml(e.to_string()))?;
    match document.select(&selector).next() {
        Some(item) => Ok(item.text().collect::<Vec<_>>().concat()),
        None => Err(ErrorKind::ParseHtml(
            "No text available at selector".to_string(),
        )),
    }
}

fn import_file_from_path(path: &str) -> Result<String, ErrorKind> {
    fs::read_to_string(path).map_err(|e| ErrorKind::ReadToString(e.to_string()))
}

fn fetch_html_from_site(address: String) -> Result<String, ErrorKind> {
    let response = reqwest::blocking::get(&address)
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
    use std::{fs, thread};

    use wiremock::matchers::{any, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::{extract_text_from_html, fetch_html_from_site, import_file_from_path};

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
        let text = extract_text_from_html(html, selector).expect("Unable to parse HTML");
        assert_eq!(text, "Hello, world!")
    }

    #[test]
    fn test_extract_text_from_html_is_failure() {
        let html = supply_test_html();
        let selector = "div#example a";
        let e = extract_text_from_html(html, selector).unwrap_err();
        assert_eq!(
            e.to_string(),
            "Error parsing HTML. No text available at selector"
        )
    }

    #[test]
    fn test_import_file_from_path_is_success() {
        let file_path = "foo.txt";
        File::create(file_path).expect("Unable to create file for test");
        let text = import_file_from_path(file_path).expect("Unable to import text in test");
        fs::remove_file(file_path).expect("Unable to remove file for test");
        assert_eq!(text, "")
    }

    #[test]
    fn test_import_file_from_path_is_failure() {
        let e = import_file_from_path("foo.txt").unwrap_err();
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
            let html = fetch_html_from_site(mock_server.uri()).expect("Unable to get HTML");
            let selector = "div#example p";
            let text = extract_text_from_html(&html, selector).expect("Unable to parse HTML");
            assert_eq!(text, "Hello, world!")
        })
        .join()
        .expect("Unable to make request");
    }
}
