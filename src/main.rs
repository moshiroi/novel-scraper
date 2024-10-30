use std::{path::PathBuf, str::FromStr, time::Duration};

use eyre::OptionExt;
use thirtyfour::prelude::*;
use url::Url;
use yaml_rust::{Yaml, YamlLoader};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config_path = PathBuf::from_str("config.yaml")?;
    let yaml = BookDetails::from_yaml(config_path)?;

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

pub struct BookDetails {
    book_title: String,
    source_url: Url,
    chapter_title: ElementSelector,
    chapter_content: ElementSelector,
    next_chapter_link: ElementSelector,
}

impl BookDetails {
    // TODO: make generic trait?
    pub fn from_yaml(path: PathBuf) -> eyre::Result<Yaml> {
        let yaml_string = std::fs::read_to_string(path)?;
        // Multi doc support, we only care about [0]
        let yaml = YamlLoader::load_from_str(&yaml_string)?;
        let yaml = yaml
            .first()
            .ok_or_eyre(
                "Yaml file could not be parsed - are you sure it exists in the path described?",
            )?
            .to_owned();

        println!("{:?}", yaml);

        Ok(yaml)
    }
}

struct ElementSelector {
    html_tag: String,
    identifier_type: HtmlIdentifier,
    identifier_name: String,
    attribute: Option<String>,
}

enum HtmlIdentifier {
    Class,
    Id,
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
