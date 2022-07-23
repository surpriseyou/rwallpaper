/*
 * @Author: mty
 * @Date: 2022-07-12 16:57:15
 * @LastEditTime: 2022-07-23 14:38:00
 * @LastEditors: anonymous
 * @Description:
 * @FilePath: \rwallpaper\src-tauri\src\lib.rs
 * no code no bug.
 */
#[macro_use]
extern crate lazy_static;
pub mod error;
pub mod spiders;
pub mod utils;

use anyhow::Ok;
use anyhow::Result;
pub use error::WallPapaerError;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use spiders::Spider;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageQuery {
    pub page: u8,
    pub tags: Option<Vec<String>>,
    pub keyword: String,
    pub source: String,
}

impl ImageQuery {
    pub fn new(source: String, page: u8, keyword: String, tags: Option<Vec<String>>) -> Self {
        ImageQuery {
            page,
            tags,
            keyword,
            source,
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
        query
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
        let file_name = self.source.split('/').last().unwrap();

        // download image
        let local_path = utils::download(&self.source, file_name).await;

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

lazy_static! {
    static ref SPIDERS: HashMap<String, Box<dyn Spider + Sync>> = {
        let mut map = HashMap::new();
        let spiders = spiders::get_all_spiders().unwrap();
        for spider in spiders {
            map.insert(spider.name(), spider);
        }
        map
    };
}

pub async fn get_images(query: &ImageQuery) -> Result<Vec<Image>> {
    let spider = SPIDERS.get(&query.source);

    if let Some(spider) = spider {
        spider.crawl(query).await;
        let images = spider.get_images()?;
        return Ok(images);
    }

    Err(WallPapaerError::UnknownImageSource(query.source.to_owned()).into())
}

pub async fn get_all_sources() -> Result<Vec<String>> {
    let sources: Vec<String> = SPIDERS.keys().map(|key| key.to_owned()).collect();
    Ok(sources)
    // Err(WallPapaerError::DoesNotHaveAnyImageSource.into())
}
