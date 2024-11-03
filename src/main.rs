use std::{env, path::PathBuf, str::FromStr, time::Duration};

use eyre::OptionExt;
use serde::Deserialize;
use thirtyfour::prelude::*;
use url::Url;
use yaml_rust::{Yaml, YamlLoader};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config_path = PathBuf::from_str("./config.yaml")?;
    let yaml = std::fs::read_to_string(config_path)?;
    let serde_yaml: BookDetails = serde_yaml::from_str(&yaml)?;
    // let yaml = BookDetails::from_path(config_path)?;

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
    #[serde(rename = "tag")]
    html_tag: String,
    #[serde(rename = "type")]
    identifier_type: HtmlIdentifier,
    #[serde(rename = "name")]
    identifier_name: String,
    attribute: Option<String>,
}

#[derive(Deserialize, Debug)]
enum HtmlIdentifier {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "class_")]
    Class,
}
impl FromStr for HtmlIdentifier {
    type Err = eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if (s.eq_ignore_ascii_case("id")) {
            return Ok(HtmlIdentifier::Id);
        } else if (s.eq_ignore_ascii_case("class")) {
            return Ok(HtmlIdentifier::Class);
        } else {
            return Err(eyre::eyre!(
                "Invalid input for constructing HtmlIdentifier, expected class or id"
            ));
        }
    }
}
