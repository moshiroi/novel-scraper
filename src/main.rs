use std::time::Duration;

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let mut caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:4444", caps.clone()).await?;
     driver.maximize_window().await?;
     driver.goto("https://novelbin.org/novelbin/ze-tian-ji/chapter-1").await?;
     std::thread::sleep(Duration::from_secs(2));
     // Always explicitly close the browser.
     driver.quit().await?;

     Ok(())
}
