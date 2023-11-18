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
        self.links_url = self.config["links"]["url"]
        self.links_class = self.config["links"]["class"]
        self.content_class = self.config["content"]["class"]
        self.content_urls = []
        logger.success("Loaded config into NovelScraper")

    def get_content_urls(self):
        req = Request(self.links.url, headers={'User-Agent': 'Mozilla/5.0'})
        webpage = urlopen(req).read()
        soup = bs.BeautifulSoup(webpage, 'lxml')
        link_tags = soup.find_all("a")

        logger.info("retrieving links of interest")
        for tag in link_tags:
            if self.links_class in tag.attrs:
                self.content_urls.append(tag[self.links_class])
        return

    def get_content(self):
        forbidden_text = {"Chapter end", "Report"}
        driver = webdriver.Chrome()
        chapter_counter = 0

        f = open(
            "novel/novel.docx", "a", encoding="utf-8")

        print(f"About to scrape: {len(self.content_urls)} chapters")

        for url in self.content_urls:
            print(
                f"--------------- scraped url: {url} ----------------")

            driver.get(url)
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
            print(
                f"---------------scraped chapter {chapter_counter} -------------------")
            chapter_counter += 1

        return


def main():
    novel_scraper= NovelScraper("config.yaml")
    novel_scraper.from_config()

    return

if __name__ == "__main__":
    main()
