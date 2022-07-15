/*
 * @Author: mty
 * @Date: 2022-07-12 16:57:15
 * @LastEditTime: 2022-07-15 20:33:36
 * @LastEditors: anonymous
 * @Description:
 * @FilePath: \rwallpaper\src-tauri\src\lib.rs
 * no code no bug.
 */
pub mod error;
pub mod utils;

use anyhow::Result;
use async_trait::async_trait;
pub use error::WallPapaerError;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde::Serialize;

#[async_trait]
pub trait Spider {
    async fn crawl(&mut self, query: ImageQuery);
    fn name(&self) -> String;
    fn new() -> Self;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageQuery {
    pub page: u8,
    pub tags: Option<Vec<String>>,
    pub keyword: String,
}

impl ImageQuery {
    pub fn new(page: u8, keyword: String, tags: Option<Vec<String>>) -> Self {
        ImageQuery {
            page,
            tags,
            keyword,
        }
    }
}

impl ToString for ImageQuery {
    fn to_string(&self) -> String {
        let mut query = String::new();
        if self.page > 1 {
            query.push_str(&format!("?page={}", self.page));
        }
        if !self.keyword.is_empty() {
            let s = if query.is_empty() { "?" } else { "&" };
            query.push_str(&format!("{}q={}", s, self.keyword));
        }
        return query;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Image {
    pub source: String,
    pub thumbnail: String,
    pub local_path: String,
}

impl Image {
    pub fn new(thumbnail: String, source: String) -> Self {
        Self {
            source,
            thumbnail,
            local_path: "".to_string(),
        }
    }

    pub async fn download(&mut self) -> Result<()> {
        if self.source.is_empty() {
            return Err(WallPapaerError::InvalidOperate("source is empty".to_owned()).into());
        }
        // get file name
        let file_name = self.source.split("/").last().unwrap();

        // download image
        let local_path = utils::download(&self.source, &file_name).await;

        self.local_path = local_path;

        Ok(())
    }
    pub fn set_background(&self) -> Result<()> {
        if self.local_path.is_empty() {
            return Err(WallPapaerError::InvalidOperate("set_background".to_owned()).into());
        }
        utils::set_background(&self.local_path)
    }
}

pub struct Wallhaven {
    pub images: Vec<Image>,
}

#[async_trait]
impl Spider for Wallhaven {
    async fn crawl(&mut self, query: ImageQuery) {
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
                let file_name = thumbnail.split("/").last().unwrap();
                let source = thumbnail
                    .clone()
                    .replace("th.wallhaven.cc", "w.wallhaven.cc")
                    .replace("small", "full")
                    .replace(file_name, format!("wallhaven-{}", file_name).as_str());

                self.images.push(Image::new(thumbnail, source));
            }
        }
    }

    fn name(&self) -> String {
        return String::from("wallhaven");
    }

    fn new() -> Wallhaven {
        Wallhaven { images: vec![] }
    }
}
