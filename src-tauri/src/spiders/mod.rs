/*
 * @Author: mty
 * @Date: 2022-07-23 10:15:25
 * @LastEditTime: 2022-07-23 14:16:08
 * @LastEditors: anonymous
 * @Description: images spider crawlers
 * @FilePath: \rwallpaper\src-tauri\src\spiders\mod.rs
 * no code no bug.
 */
use async_trait::async_trait;

use crate::{Image, ImageQuery};
use anyhow::{Ok, Result};

mod wallhaven;

#[async_trait]
pub trait Spider {
    async fn crawl(&self, query: &ImageQuery);
    fn name(&self) -> String;
    // fn new() -> Self;
    fn get_images(&self) -> Result<Vec<Image>>;
}

pub fn get_all_spiders() -> Result<Vec<Box<dyn Spider + Sync>>> {
    let mut spiders: Vec<Box<dyn Spider + Sync>> = vec![];

    let wallhaven = wallhaven::Wallhaven::new();

    spiders.push(Box::new(wallhaven));

    Ok(spiders)
}
