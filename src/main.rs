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
        tracing::info!("Succesfully parsed scraping config");
        Ok(Self {
            driver,
            book_details,
        })
    }

    pub async fn scrape(&self) -> eyre::Result<()> {
        self.driver.maximize_window().await?;
        // Go to source url - begin crawling
        self.driver
            .goto(&self.book_details.links.source_url)
            .await?;
        // EXCEPTION: need wait for the page to load before scraping contents
        std::thread::sleep(Duration::from_secs(1));
        let mut next_link = self.scrape_current_chapter().await?;

        while !next_link.is_empty() {
            self.driver.goto(next_link).await?;
            std::thread::sleep(Duration::from_secs(1));
            next_link = self.scrape_current_chapter().await?;
        }

        Ok(())
    }

    pub async fn scrape_current_chapter(&self) -> eyre::Result<String> {
        tracing::info!("Starting scraping");
        let title = self
            .retrieve_element(&self.book_details.identifiers.title)
            .await?
            .text()
            .await?;

        tracing::info!("Retrieved chapter title");
        dbg!(title);

        let contents = self
            .retrieve_element(&self.book_details.identifiers.content)
            .await?
            .text()
            .await?;

        tracing::info!("Retrieved chapter contents");
        dbg!(contents);

        let next_chapter_link = self
            .retrieve_element(&self.book_details.identifiers.next_chapter)
            .await?
            .attr("href")
            .await?
            .ok_or_eyre("Could not get href of next chapter link <a>")?;

        tracing::info!("Retrieved next chapter link");
        dbg!(&next_chapter_link);

        Ok(next_chapter_link)
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
