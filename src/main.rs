use std::{path::PathBuf, str::FromStr, time::Duration};

use models::config::{BookDetails, HtmlIdentifier};
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
    scraper.get_chapter_title().await?;
    // Always explicitly close the browser.
    scraper.driver.quit().await?;

    Ok(())
}

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

    pub fn scrape(&self) {
        todo!();
        self.get_chapter_title();
        self.get_chapter_contents();
        self.get_next_chapter_link();
    }

    pub async fn get_chapter_title(&self) -> eyre::Result<()> {
        let title_details = &self.book_details.identifiers.title;
        let title = match title_details.identifier_type {
            HtmlIdentifier::Id => self.driver.find(By::Id(title_details.name.clone())),
            HtmlIdentifier::Class => self.driver.find(By::ClassName(title_details.name.clone())),
        }
        .await?;

        dbg!(title.text().await?);

        Ok(())
    }

    pub fn get_chapter_contents(&self) {
        todo!()
    }

    pub fn get_next_chapter_link(&self) {
        todo!()
    }
}
