use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use eyre::OptionExt;
use models::config::{BookDetails, ElementSelector, HtmlIdentifier};
use thirtyfour::prelude::*;

mod models;

const SCRAPING_DELAY: Duration = Duration::from_secs(3);

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let config_path = PathBuf::from_str("./config.yaml")?;
    let mut caps = DesiredCapabilities::chrome();

    caps.add_arg("--disable-popup-blocking")?;
    // caps.add_arg("--headless")?;
    caps.add_experimental_option(
        "excludeSwitches",
        vec!["--disable-popup-blocking", "disable-popup-blocking"],
    )?;
    let user_agent = 
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.50 Safari/537.36".to_string();
    let user_arg = format!("user-agent={}", user_agent);
    caps.add_arg(&user_arg)?;
    let driver = WebDriver::new("http://localhost:4444", caps.clone()).await?;

    let scraper = Scraper::new(config_path, driver)?;
    scraper.scrape().await?;
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
        let mut f = File::create(&self.book_details.book_title)?;
        std::thread::sleep(SCRAPING_DELAY);
        let mut next_link = self.scrape_current_chapter(&mut f).await?;

        while !next_link.is_empty() {
            self.driver.goto(next_link).await?;
            std::thread::sleep(SCRAPING_DELAY);
            next_link = self.scrape_current_chapter(&mut f).await?;
        }

        Ok(())
    }

    pub async fn scrape_current_chapter(&self, f: &mut File) -> eyre::Result<String> {
        tracing::info!("Scraping chapter");
        self.driver.screenshot(Path::new("debug.png")).await?;
        self.close_all_popups().await?;
        self.driver.screenshot(Path::new("debug2.png")).await?;
        let title = self
            .retrieve_element(&self.book_details.identifiers.title)
            .await?
            .text()
            .await?;

        f.write(title.as_bytes())?;
        f.write(b"\n")?;
        f.write(b"\n")?;
        tracing::info!("Retrieved chapter title");

        let contents = self
            .retrieve_element(&self.book_details.identifiers.content)
            .await?
            .text()
            .await?;

        f.write(contents.as_bytes())?;
        f.write(b"\n");
        tracing::info!("Retrieved chapter contents");

        let next_chapter_link = self
            .retrieve_element(&self.book_details.identifiers.next_chapter)
            .await?
            .attr("href")
            .await?
            .ok_or_eyre("Could not get href of next chapter link <a>")?;
        tracing::info!("Retrieved next chapter link: {}", &next_chapter_link);

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

    async fn close_all_popups(&self) -> eyre::Result<()>{
        let xpaths = vec![
            // Common close buttons
            "//*[contains(@class, 'close')]",
            "//*[@aria-label='Close']",
            "//*[@aria-label='Dismiss']",
            "//*[text()='Close']",
            "//*[text()='Dismiss']",
            // Close buttons within modals
            "//*[@role='dialog']//*[contains(@class, 'close')]",
            "//*[@role='dialog']//*[@aria-label='Close']",
        ];
        for xpath in xpaths {
            if let Ok(element) = self.driver.find(By::XPath(xpath)).await {
                dbg!(element.clone());
                element.click().await?;
            }
        }

        Ok(())
    }

}
