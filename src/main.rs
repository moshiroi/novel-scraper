use std::{path::PathBuf, str::FromStr, time::Duration};

use eyre::OptionExt;
use models::config::{BookDetails, ElementSelector, HtmlIdentifier};
use thirtyfour::prelude::*;

mod models;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let config_path = PathBuf::from_str("./config.yaml")?;
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", caps.clone()).await?;

    let scraper = Scraper::new(config_path, driver)?;
    scraper.driver.maximize_window().await?;
    scraper
        .driver
        .goto("https://novelbin.org/novelbin/ze-tian-ji/chapter-1")
        .await?;
    std::thread::sleep(Duration::from_secs(2));
    scraper.scrape().await?;
    // Always explicitly close the browser.
    scraper.driver.quit().await?;

    Ok(())
}

// See: https://docs.rs/thirtyfour/latest/thirtyfour/session/handle/struct.SessionHandle.html#method.find
pub struct Scraper {
    pub driver: WebDriver,
    pub book_details: BookDetails,
}

impl Scraper {
    pub fn new(config_path: PathBuf, driver: WebDriver) -> eyre::Result<Self> {
        let yaml = std::fs::read_to_string(config_path)?;
        let book_details: BookDetails = serde_yaml::from_str(&yaml)?;

        Ok(Self {
            driver,
            book_details,
        })
    }

    pub async fn scrape(&self) -> eyre::Result<()> {
        let title = self
            .retrieve_element(&self.book_details.identifiers.title)
            .await?
            .text()
            .await?;

        dbg!(title);

        let contents = self
            .retrieve_element(&self.book_details.identifiers.content)
            .await?
            .text()
            .await?;

        dbg!(contents);

        let next_chapter_link = self
            .retrieve_element(&self.book_details.identifiers.next_chapter)
            .await?
            .attr("href")
            .await?
            .ok_or_eyre("Could not get href of next chapter link <a>")?;

        dbg!(next_chapter_link);

        Ok(())
    }

    pub async fn retrieve_element(&self, element: &ElementSelector) -> eyre::Result<WebElement> {
        let web_element = match element.identifier_type {
            HtmlIdentifier::Id => self.driver.find(By::Id(element.name.clone())),
            HtmlIdentifier::Class => self.driver.find(By::ClassName(element.name.clone())),
        }
        .await?;

        Ok(web_element)
    }
}
