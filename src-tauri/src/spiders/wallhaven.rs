/*
 * @Author: mty
 * @Date: 2022-07-23 10:21:49
 * @LastEditTime: 2022-08-06 17:34:08
 * @LastEditors: anonymous
 * @Description: wallhaven spider
 * @FilePath: \rwallpaper\src-tauri\src\spiders\wallhaven.rs
 * no code no bug.
 */

use std::cell::RefCell;

use anyhow::Result;
use scraper::{Html, Selector};

use crate::{Image, ImageQuery};

use super::{Spider, SpiderElementRef, SpiderHtml};

use async_trait::async_trait;

pub struct Wallhaven {
    images: RefCell<Vec<Image>>,
}

#[async_trait]
impl Spider for Wallhaven {
    async fn crawl(&self, query: &ImageQuery) {
        // do we need to clear up?
        self.images.borrow_mut().clear();

        let url = String::from("https://wallhaven.cc/toplist") + &query.to_string();

        println!("url: {}", url);

        let resp = reqwest::get(url).await.unwrap().text().await.unwrap();

        let document = SpiderHtml::new(Html::parse_document(&resp));

        let selector = Selector::parse("#thumbs .thumb-listing-page figure").unwrap();

        let img_selector = Selector::parse("img").unwrap(); // get data-src
        let link_selector = Selector::parse("a").unwrap(); // get href

        let elements = document
            .0
            .select(&selector)
            .map(|e| SpiderElementRef::new(e))
            .collect::<Vec<_>>();

        for element in elements {
            let mut image = Image::new("".to_string(), "".to_string());
            {
                let img = element.0.select(&img_selector).next().unwrap();
                if let Some(src) = img.value().attr("data-src") {
                    image.thumbnail = src.to_string();
                }
            }

            let link = element
                .0
                .select(&link_selector)
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap();

            image.source = self.crawl_detail(link).await;

            println!("{:?}", image);

            self.images.borrow_mut().push(image);
        }

        // for element in document.select(&selector) {
        //     let img = element.select(&img_selector).next();

        //     if let Some(ss) = img.value().attr("data-src") {
        //         let thumbnail = ss.to_owned();
        //         // thumbnial https://th.wallhaven.cc/small/y8/y8pr1d.jpg
        //         // source https://w.wallhaven.cc/full/y8/wallhaven-y8pr1d.jpg
        //         // from thumbnail convert to source
        //         // get file name
        //         let file_name = thumbnail.split('/').last().unwrap();
        //         let source = thumbnail
        //             .clone()
        //             .replace("th.wallhaven.cc", "w.wallhaven.cc")
        //             .replace("small", "full")
        //             .replace(file_name, format!("wallhaven-{}", file_name).as_str());

        //         self.images.borrow_mut().push(Image::new(thumbnail, source));
        //     }
        // }
    }

    fn get_images(&self) -> Result<Vec<Image>> {
        Ok(self.images.borrow().clone())
    }

    fn name(&self) -> String {
        String::from("wallhaven")
    }
}

impl Wallhaven {
    pub fn new() -> Self {
        Wallhaven {
            images: RefCell::new(vec![]),
        }
    }

    pub async fn crawl_detail(&self, link: &str) -> String {
        println!("link: {}", link);
        let resp = reqwest::get(link).await.unwrap().text().await.unwrap();
        println!("resp: geted.");

        let document = Html::parse_document(&resp);
        let selector = Selector::parse("#wallpaper").unwrap();

        if let Some(src) = document.select(&selector).next() {
            // println!("{src:?}");
            if let Some(src) = src.value().attr("data-cfsrc") {
                println!("crawl finished {}", src);
                return src.to_owned();
            } else {
                println!("get src failed");
            }
        } else {
            println!("css selector failed");
        }
        "".to_owned()
    }
}

unsafe impl Sync for Wallhaven {}
