import bs4 as bs
from urllib.request import Request, urlopen
from selenium import webdriver
import time
import yaml
from loguru import logger

# TODO:
# -> Add retry on error
# -> superfluous errors cause script to break
# -> requires manual rerun from the last chapter scraped

# TODO: Split main into multiple functions & optimize iterations/loops

class NovelScraper:
    def __init__(self, config_path):
        with open(config_path, 'r') as file:
            logger.info("extracting config")
            self.config = yaml.safe_load(file)
            logger.success("Intialized NovelScraper Class")

    def from_config(self):
        self.book_title = self.config["book_title"]
        self.source_url = self.config["links"]["source_url"]
        self.next_chapter_class = self.config["identifiers"]["next_chapter"]
        self.content_class = self.config["identifiers"]["content"]
        self.current_url = self.source_url
        logger.success("Loaded config into NovelScraper")

    def get_next_chapter_url(self, soup):
        link_tags = soup.find_all("a", id=self.next_chapter_class)

        logger.info("retrieving next chapter link")
        for tag in link_tags:
            if "href" in tag.attrs:
                return tag["href"]

        return
    
    def scrape(self):
        forbidden_text = {"Chapter end", "Report"}
        driver = webdriver.Chrome()
        chapter_counter = 0

        f = open(
            f"novel/{self.book_title}.docx", "a", encoding="utf-8")

        logger.info(f"About to begin scraping")

        while self.current_url:
            logger.info(
                f"scraping url: {self.current_url}")

            driver.get(self.current_url)
            time.sleep(5)
            html = driver.page_source

            soup = bs.BeautifulSoup(html, 'lxml')
            chapter_content_tags = soup.find_all(
                "div", class_=(self.content_class))

            for tag in chapter_content_tags:
                for content in tag.contents:
                    content_text = content.get_text()
                    if content_text.strip() not in forbidden_text and "Reddit" not in content_text.strip() and "ʟɪɢʜᴛɴᴏᴠᴇʟᴡᴏʀʟᴅ.ᴄᴏᴍ" not in content_text.strip():
                        f.write(content_text)
                        f.write("\n\n")
            logger.success(
                f"scraped chapter {chapter_counter}: self.current_url")

            chapter_counter += 1 

            # updating current url w/ next chapter url
            self.current_url = self.get_next_chapter_url(soup)

        return


def main():
    novel_scraper= NovelScraper("config.yaml")
    novel_scraper.from_config()
    novel_scraper.scrape()
    return

if __name__ == "__main__":
    main()
