/*
 * @Author: mty
 * @Date: 2022-07-23 10:15:25
 * @LastEditTime: 2022-07-30 22:48:01
 * @LastEditors: anonymous
 * @Description: images spider crawlers
 * @FilePath: \rwallpaper\src-tauri\src\spiders\mod.rs
 * no code no bug.
 */
use async_trait::async_trait;

use crate::{Image, ImageQuery};
use anyhow::{Ok, Result};
use scraper::{ElementRef, Html};

mod meinv;
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

    let meinv = meinv::Meinv::new();

    spiders.push(Box::new(meinv));
    spiders.push(Box::new(wallhaven));

    Ok(spiders)
}

struct SpiderElementRef<'a>(ElementRef<'a>);

impl<'a> SpiderElementRef<'a> {
    pub fn new(element: ElementRef<'a>) -> Self {
        SpiderElementRef(element)
    }
}

unsafe impl Send for SpiderElementRef<'_> {}
unsafe impl Sync for SpiderElementRef<'_> {}

struct SpiderHtml(Html);

impl SpiderHtml {
    pub fn new(html: Html) -> Self {
        SpiderHtml(html)
    }
}

unsafe impl Send for SpiderHtml {}
unsafe impl Sync for SpiderHtml {}
