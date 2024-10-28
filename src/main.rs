use std::time::Duration;

use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let mut caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:4444", caps.clone()).await?;
    caps.set_binary("chromium")?;
    // caps.add_chrome_option("headless", false)?; // Set to true if you want headless

     // Navigate to https://wikipedia.org.
     driver.goto("https://wikipedia.org").await?;
     std::thread::sleep(Duration::from_secs(2));
     let elem_form = driver.find(By::Id("search-form")).await?;

     // Find element from element.
     // let elem_text = elem_form.find(By::Id("searchInput")).await?;

     // Type in the search terms.
     // elem_text.send_keys("selenium").await?;

     // // Click the search button.
     // let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
     // elem_button.click().await?;

     // // Look for header to implicitly wait for the page to load.
     // driver.find(By::ClassName("firstHeading")).await?;
     // assert_eq!(driver.title().await?, "Selenium - Wikipedia");
    
     // Always explicitly close the browser.
     driver.quit().await?;

     Ok(())
}
