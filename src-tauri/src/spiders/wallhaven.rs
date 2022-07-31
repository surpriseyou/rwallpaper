/*
 * @Author: mty
 * @Date: 2022-07-23 10:21:49
 * @LastEditTime: 2022-07-23 20:16:37
 * @LastEditors: anonymous
 * @Description: wallhaven spider
 * @FilePath: \rwallpaper\src-tauri\src\spiders\wallhaven.rs
 * no code no bug.
 */

use std::cell::RefCell;

use anyhow::Result;
use scraper::{Html, Selector};

use crate::{Image, ImageQuery};

use super::Spider;

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

        let document = Html::parse_document(&resp);

        let selector = Selector::parse("#thumbs .thumb-listing-page img").unwrap();

        for element in document.select(&selector) {
            if let Some(ss) = element.value().attr("data-src") {
                let thumbnail = ss.to_owned();
                // thumbnial https://th.wallhaven.cc/small/y8/y8pr1d.jpg
                // source https://w.wallhaven.cc/full/y8/wallhaven-y8pr1d.jpg
                // from thumbnail convert to source
                // get file name
                let file_name = thumbnail.split('/').last().unwrap();
                let source = thumbnail
                    .clone()
                    .replace("th.wallhaven.cc", "w.wallhaven.cc")
                    .replace("small", "full")
                    .replace(file_name, format!("wallhaven-{}", file_name).as_str());

                self.images.borrow_mut().push(Image::new(thumbnail, source));
            }
        }
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
}

unsafe impl Sync for Wallhaven {}
