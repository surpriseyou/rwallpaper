/*
 * @Author: mty
 * @Date: 2022-07-23 20:14:59
 * @LastEditTi&me: 2022-07-23 22:42:14
 * @LastEditors: anonymous
 * @Description: https://www.2meinv.com/ 美女图片网站
 * @FilePath: \rwallpaper\src-tauri\src\spiders\meinv.rs
 * no code no bug.
 */
use std::cell::RefCell;

use anyhow::{Ok, Result};
use scraper::{Html, Selector};

use crate::{Image, ImageQuery};

use super::{Spider, SpiderElementRef, SpiderHtml};

use async_trait::async_trait;

pub struct Meinv {
    images: RefCell<Vec<Image>>,
}

impl Meinv {
    pub fn new() -> Self {
        Meinv {
            images: RefCell::new(Vec::new()),
        }
    }
}

impl Meinv {
    pub async fn crawl_inner(&self, url: String) {
        // let url = format!("https://www.2meinv.com/index-{}.html", page);
        let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
        let document = SpiderHtml::new(Html::parse_document(&resp));
        let selector = Selector::parse(".detail-list li").unwrap();

        let elements = document
            .0
            .select(&selector)
            .map(|e| SpiderElementRef::new(e))
            .collect::<Vec<_>>();

        for element in elements {
            let mut link: String = String::new();

            let mut image = Image::new("".to_string(), "".to_string());

            {
                let img = element
                    .0
                    .select(&Selector::parse("img").unwrap())
                    .next()
                    .unwrap();

                let thumbnail = img.value().attr("src").unwrap().to_string();

                image.thumbnail = thumbnail;

                let img_link = element
                    .0
                    .select(&Selector::parse("a").unwrap())
                    .next()
                    .unwrap();

                link = img_link.value().attr("href").unwrap().to_string();
            }

            let source = self.crawl_detail(&link).await;

            image.source = source;

            self.images.borrow_mut().push(image);
        }
    }

    pub async fn crawl_detail(&self, link: &str) -> String {
        // println!("link: {}", link);
        let resp = reqwest::get(link).await.unwrap().text().await.unwrap();
        // println!("resp: geted.");

        let document = Html::parse_document(&resp);
        let selector = Selector::parse(".pp img").unwrap();

        if let Some(src) = document.select(&selector).next() {
            if let Some(src) = src.value().attr("src") {
                // println!("crawl finished {}", src);
                return src.to_owned();
            }
        }
        "".to_owned()
    }
}

#[async_trait]
impl Spider for Meinv {
    async fn crawl(&self, query: &ImageQuery) {
        // do we need to clear up?
        self.images.borrow_mut().clear();

        let url = format!("https://www.2meinv.com/index-{}.html", query.page);

        self.crawl_inner(url).await;
    }

    fn name(&self) -> String {
        String::from("2meinv")
    }
    fn get_images(&self) -> Result<Vec<Image>> {
        Ok(self.images.borrow().clone())
    }
}

unsafe impl Sync for Meinv {}
unsafe impl Send for Meinv {}
