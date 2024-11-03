use std::{env, path::PathBuf, str::FromStr, time::Duration};

use eyre::OptionExt;
use serde::Deserialize;
use thirtyfour::prelude::*;
use url::Url;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config_path = PathBuf::from_str("./config.yaml")?;
    let yaml = std::fs::read_to_string(config_path)?;
    let serde_yaml: BookDetails = serde_yaml::from_str(&yaml)?;

    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", caps.clone()).await?;
    driver.maximize_window().await?;
    driver
        .goto("https://novelbin.org/novelbin/ze-tian-ji/chapter-1")
        .await?;
    std::thread::sleep(Duration::from_secs(2));
    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct BookDetails {
    book_title: String,
    links: Links,
    identifiers: Identifiers,
}

#[derive(Deserialize, Debug)]
struct Links {
    source_url: String,
}

#[derive(Deserialize, Debug)]
struct Identifiers {
    next_chapter: ElementSelector,
    content: ElementSelector,
    title: ElementSelector,
}

#[derive(Deserialize, Debug)]
struct ElementSelector {
    tag: String,
    #[serde(rename = "type")]
    identifier_type: HtmlIdentifier,
    name: String,
    attribute: Option<String>,
}

#[derive(Deserialize, Debug)]
enum HtmlIdentifier {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "class_")]
    Class,
}

impl AsRef<str> for HtmlIdentifier {
    fn as_ref(&self) -> &str {
        match self {
            HtmlIdentifier::Id => "id",
            HtmlIdentifier::Class => "class",
        }
    }
}
