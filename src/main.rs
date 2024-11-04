use std::{path::PathBuf, str::FromStr, time::Duration};

use models::config::BookDetails;
use thirtyfour::prelude::*;

mod models;

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
