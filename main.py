import bs4 as bs
from urllib.request import Request, urlopen
from selenium import webdriver
import time
import yaml
from loguru import logger

# TODO:
# -> Add retry on response error


class NovelScraper:
    def __init__(self, config_path):
        with open(config_path, 'r') as file:
            logger.info("extracting config")
            self.config = yaml.safe_load(file)
            logger.success("Intialized NovelScraper Class")

    def from_config(self):
        # Title
        self.book_title = self.config["book_title"]

        # Urls
        self.prefix_url = self.config["links"]["prefix_url"]
        self.source_url = self.config["links"]["source_url"]
        self.current_url = self.config["links"]["source_url"]

        # Next chapter details
        self.next_chapter_tag = self.config["identifiers"]["next_chapter"]["tag"]
        self.next_chapter_id_type = self.config["identifiers"]["next_chapter"]["type"]
        self.next_chapter_id_name = self.config["identifiers"]["next_chapter"]["name"]

        # Content details
        self.content_tag = self.config["identifiers"]["content"]["tag"]
        self.content_id_type = self.config["identifiers"]["content"]["type"]
        self.content_id_name = self.config["identifiers"]["content"]["name"]

        # Title details
        self.chapter_title_tag = self.config["identifiers"]["title"]["tag"]
        self.chapter_title_id_type = self.config["identifiers"]["title"]["type"]
        self.chapter_title_id_name = self.config["identifiers"]["title"]["name"]

        if "attribute" in self.config["identifiers"]["title"]:
            logger.info("chapter_title_attribute found")
            self.chapter_title_attribute = self.config["identifiers"]["title"]["attribute"]

        self.filter = self.config["text_filter"]

        logger.success("Loaded config into NovelScraper")

    def get_next_chapter_url(self, soup):
        link_tags = soup.find_all(
            self.next_chapter_tag, **{self.next_chapter_id_type: self.next_chapter_id_name})

        logger.info("retrieving next chapter link")
        for tag in link_tags:
            if "href" in tag.attrs:
                return self.prefix_url + tag["href"]
        return

    def get_chapter_title(self, soup):
        chapter_title_tags = soup.find_all(
            self.chapter_title_tag, **{self.chapter_title_id_type: self.chapter_title_id_name})

        logger.info("retrieving chapter title")
        for tag in chapter_title_tags:
            if hasattr(self, 'chapter_title_attribute'):
                if self.chapter_title_attribute in tag.attrs:
                    logger.info(
                        f"title is {tag[self.chapter_title_attribute]}")
                    return tag[self.chapter_title_attribute]
            else:
                # TODO: Refine logic for this section
                logger.info("Couldn't find title, trying alternative")
                return tag.contents[0].get_text().strip()
        logger.error("No chapter title found")
        return

    def scrape(self):
        forbidden_text = set(self.filter)
        driver = webdriver.Chrome()

        f = open(
            f"novel/{self.book_title}.docx", "a", encoding="utf-8")

        logger.info("About to begin scraping")

        while self.current_url:
            logger.info(
                f"scraping url: {self.current_url}")

            driver.get(self.current_url)
            time.sleep(5)
            html = driver.page_source

            soup = bs.BeautifulSoup(html, 'lxml')

            chapter_title = self.get_chapter_title(soup)
            f.write(chapter_title)
            f.write("\n\n")
            logger.info("Wrote chapter title to file")

            chapter_content_tags = soup.find_all(
                self.content_tag, **{self.content_id_type: self.content_id_name})

            for tag in chapter_content_tags:
                for content in tag.contents:
                    content_text = content.get_text()
                    if content_text.strip() not in forbidden_text:
                        f.write(content_text)
                        f.write("\n\n")
                        logger.trace("wrote to output file")

            logger.success(
                f"scraped chapter: {self.current_url}")

            # updating current url w/ next chapter url
            self.current_url = self.get_next_chapter_url(soup)

        return


def main():
    novel_scraper = NovelScraper("config.yaml")
    novel_scraper.from_config()
    novel_scraper.scrape()
    return


if __name__ == "__main__":
    main()
