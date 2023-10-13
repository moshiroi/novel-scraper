import bs4 as bs
import urllib.request
from urllib.request import Request, urlopen
from selenium import webdriver
from selenium.webdriver.common.by import By
import time
import aspose.pdf as ap

def main():
    forbidden_text = {"Chapter end", "Report"}
    driver = webdriver.Chrome()
    
    urls = retrieve_urls()
    urls.reverse()
    urls = urls[:2]
    
    chapter_counter = 0
    for url in urls:
        driver.get(url)
        # this is just to ensure that the page is loaded
        time.sleep(2)
        html = driver.page_source

        soup = bs.BeautifulSoup(html, 'lxml')
        link_tags = soup.find_all("div", class_=("chapter-reading-section"))
        f = open(f"novel/novel-chapter-{chapter_counter}-txt.docx", "a", encoding="utf-8")
              
        for tag in link_tags:
            for content in tag.contents:
                content_text = content.get_text()
                if content_text.strip()  not in forbidden_text and "Reddit" not in content_text.strip():                  
                    f.write(content_text)
                    f.write("\n\n")
        print(f"---------------scraped chapter {chapter_counter}-------------------")
        chapter_counter += 1
    return


def retrieve_urls():
    URL = "https://www.novelcool.com/chapter/Damn-Reincarnation-CH-0/8786442/"
    req = Request(URL, headers={'User-Agent': 'Mozilla/5.0'})
    webpage = urlopen(req).read()
    soup = bs.BeautifulSoup(webpage, 'lxml')
    link_tags = soup.find_all("a")

    urls_to_scrape = []
    for tag in link_tags:
        if "href" in tag.attrs:
            urls_to_scrape.append(tag["href"])

    urls_to_scrape = urls_to_scrape[7:-16]

    return urls_to_scrape


def get_chapter_contents():
    return


def bs4_scrape():
    URL = "https://www.novelcool.com/chapter/Damn-Reincarnation-CH-0/8786442/"
    req = Request(URL, headers={'User-Agent': 'Mozilla/5.0'})
    webpage = urlopen(req).read()
    soup = bs.BeautifulSoup(webpage, 'lxml')
    link_tags = soup.find_all("a")

    urls_to_scrape = []
    for tag in link_tags:
        if "href" in tag.attrs:
            urls_to_scrape.append(tag["href"])

    urls_to_scrape = urls_to_scrape[7:-16]

    print(f"scraping chapter {urls_to_scrape[0]}")
    chapter_contents = extract_chapter(urls_to_scrape[0])
    for chapter in chapter_contents:
        print(chapter)
        print("------------------------")

    print(soup.select('div.chapter-reading-section'))
    f = open("novel-txt.txt", "a", encoding="utf-8")
    f.write(str(soup))
    return


def extract_chapter(url):
    req = Request(url, headers={'User-Agent': 'Mozilla/5.0'})
    webpage = urlopen(req).read()
    soup = bs.BeautifulSoup(webpage, 'lxml')
    link_tags = soup.find_all("div", class_=(
        "chapter-reading-section", "position-relative"))
    # for tag in link_tags:
    #     if 'class' in tag.attrs and len(tag['class'])>1:
    #         print(tag['class'])
    return link_tags


def div_filter(tag):
    return tag.has_attr('class') and ("chapter-reading-section" in tag['class'])


if __name__ == "__main__":
    main()
