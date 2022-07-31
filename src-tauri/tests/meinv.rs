/*
 * @Author: mty
 * @Date: 2022-07-23 21:25:00
 * @LastEditTime: 2022-07-23 21:43:50
 * @LastEditors: anonymous
 * @Description: meinv spider test
 * @FilePath: \rwallpaper\src-tauri\tests\meinv.rs
 * no code no bug.
 */

// pub mod app::spiders;

use app::{spiders, ImageQuery};
use futures;

#[test]
fn test_meinv() {
    let spiders = spiders::get_all_spiders().unwrap();

    assert_eq!(spiders.len(), 2);
}

#[test]
fn test_crawl() {
    let spiders = spiders::get_all_spiders().unwrap();
    let spider = spiders.get(0).unwrap();
    let query = ImageQuery {
        page: 1,
        keyword: "".to_string(),
        tags: None,
        source: "2meinv".to_string(),
    };
    futures::executor::block_on(reqwest::get("https://www.2meinv.com/article-5578.html")).unwrap();
    assert!(spider.get_images().is_ok());
}
