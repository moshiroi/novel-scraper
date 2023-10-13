import bs4 as bs
from urllib.request import Request, urlopen
from selenium import webdriver
import time

# TODO:
# -> Add retry on error
# -> superfluous errors cause script to break
# -> requires manual rerun from the last chapter scraped

# TODO: Split main into multiple functions & optimize iterations/loops


def main():
    forbidden_text = {"Chapter end", "Report"}
    driver = webdriver.Chrome()

    urls = retrieve_urls()
    urls.reverse()

    chapter_counter = 0

    f = open(
        "novel/novel-txt.docx", "a", encoding="utf-8")

    print(f"About to scrape: {len(urls)} chapters")

    for url in urls:
        print(
            f"--------------- scraped url: {url} ----------------")

        driver.get(url)
        time.sleep(5)
        html = driver.page_source

        soup = bs.BeautifulSoup(html, 'lxml')
        chapter_content_tags = soup.find_all(
            "div", class_=("chapter-reading-section"))

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


def retrieve_urls():
    # extracting urls of all available chapters from first chapter
    URL = "https://www.novelcool.com/chapter/Damn-Reincarnation-CH-0/8786442/"
    req = Request(URL, headers={'User-Agent': 'Mozilla/5.0'})
    webpage = urlopen(req).read()
    soup = bs.BeautifulSoup(webpage, 'lxml')
    link_tags = soup.find_all("a")

    urls_to_scrape = []

    for tag in link_tags:
        if "href" in tag.attrs:
            urls_to_scrape.append(tag["href"])

    # Removing the first 7 and last 16 <a> tags as unwanted links
    urls_to_scrape = urls_to_scrape[7:-16]

    return urls_to_scrape


if __name__ == "__main__":
    main()
