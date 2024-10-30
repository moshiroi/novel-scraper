use std::{path::PathBuf, str::FromStr, time::Duration};

use eyre::OptionExt;
use thirtyfour::prelude::*;
use url::Url;
use yaml_rust::{Yaml, YamlLoader};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config_path = PathBuf::from_str("config.yaml")?;
    let yaml = BookDetails::from_path(config_path)?;

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
    pub fn from_path(path: PathBuf) -> eyre::Result<Self> {
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
        let book_title = yaml["book_title"]
            .as_str()
            .ok_or_eyre("no book_title field in config yaml")?
            .to_string();

        let source_url = Url::parse(
            yaml["links"]["source_ul"]
                .as_str()
                .ok_or_eyre("no book_title field in config yaml")?,
        )?;

        let chapter_title = ElementSelector::from_yaml(&yaml["identifiers"]["title"])?;
        let chapter_content = ElementSelector::from_yaml(&yaml["identifiers"]["content"])?;
        let next_chapter_link = ElementSelector::from_yaml(&yaml["identifiers"]["next_chapter"])?;

        Ok(Self {
            book_title,
            source_url,
            chapter_title,
            chapter_content,
            next_chapter_link,
        })
    }
}

struct ElementSelector {
    html_tag: String,
    identifier_type: HtmlIdentifier,
    identifier_name: String,
    attribute: Option<String>,
}

impl ElementSelector {
    pub fn from_yaml(yaml: &Yaml) -> eyre::Result<Self> {
        let html_tag = yaml["tag"]
            .as_str()
            .ok_or_eyre("yaml does not have field tag")?
            .to_string();

        let identifier_type = HtmlIdentifier::from_str(
            yaml["type"]
                .as_str()
                .ok_or_eyre("yaml does not have field type")?,
        )?;

        let identifier_name = yaml["name"]
            .as_str()
            .ok_or_eyre("yaml does not have field tag")?
            .to_string();

        let attribute = if (yaml["attribute"].is_badvalue()) {
            None
        } else {
            Some(
                yaml["attribute"]
                    .as_str()
                    .ok_or_eyre("attribute tag does not exist")?
                    .to_string(),
            )
        };

        Ok(Self {
            html_tag,
            identifier_type,
            identifier_name,
            attribute,
        })
    }
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
