/*
 * @Author: mty
 * @Date: 2022-07-23 21:44:33
 * @LastEditTime: 2022-07-30 21:22:19
 * @LastEditors: anonymous
 * @Description: request demo
 * @FilePath: \rwallpaper\src-tauri\examples\request.rs
 * no code no bug.
 */

use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    // let url = format!("https://www.2meinv.com/index-{}.html", 1);
    // let resp = reqwest::get(url).await.unwrap().text().await.unwrap();
    // let document = Html::parse_document(&resp);
    // let selector = Selector::parse(".detail-list li").unwrap();
    // let mut count = 0;
    // for element in document.select(&selector) {
    //     let mut thumbnail = String::new();
    //     let mut source = String::new();

    //     element
    //         .select(&Selector::parse("img").unwrap())
    //         .for_each(|img| {
    //             if let Some(src) = img.value().attr("src") {
    //                 thumbnail = src.to_owned();
    //             }
    //         });

    //     // async fn crawl_inner(thumbnail: String, link: &str) -> Image {
    //     //     let resp = reqwest::get(link).await.unwrap().text().await.unwrap();
    //     //     let document = Html::parse_document(&resp);
    //     //     let selector = Selector::parse(".pp img").unwrap();

    //     //     if let Some(src) = document.select(&selector).next() {
    //     //         if let Some(src) = src.value().attr("src") {
    //     //             return Image::new(thumbnail, src.to_owned());
    //     //         }
    //     //     }
    //     //     // Err(anyhow::anyhow!("failed to get source"))
    //     //     Image::new(thumbnail, "".to_owned())
    //     // }
    //     // let sub_selector = Selector::parse("a").unwrap();
    //     // let links = element.select(&sub_selector);
    //     // for link in links {
    //     //     if let Some(href) = link.value().attr("href") {
    //     //         let source = crawl_inner(thumbnail.clone(), href).await;
    //     //         println!("{:?}", source);
    //     //     }
    //     // }

    //     // let sub_selector = Selector::parse("a").unwrap();
    //     // let links = element.select(&sub_selector);
    //     // for link in links {
    //     //     if let Some(href) = link.value().attr("href") {
    //     //         println!("href: {}", href);

    //     //         let resp = futures::executor::block_on(reqwest::get(href))
    //     //             .unwrap()
    //     //             .text();

    //     //         let resp = futures::executor::block_on(resp).unwrap();

    //     //         let document = Html::parse_document(&resp);
    //     //         let selector = Selector::parse(".pp img").unwrap();
    //     //         if let Some(src) = document.select(&selector).next() {
    //     //             if let Some(src) = src.value().attr("src") {
    //     //                 source = src.to_owned();
    //     //                 println!("source: {}", source);
    //     //             }
    //     //         }
    //     //     }
    //     // }

    //     for link in element.select(&Selector::parse("a").unwrap()) {
    //         if let Some(href) = link.value().attr("href") {
    //             let resp = reqwest::get(href).await.unwrap().text().await.unwrap();
    //             let document = Html::parse_document(&resp);
    //             let selector = Selector::parse(".pp img").unwrap();

    //             if let Some(src) = document.select(&selector).next() {
    //                 if let Some(src) = src.value().attr("src") {
    //                     source = src.to_owned();
    //                 }
    //             }
    //         }
    //     }
    //     count += 1;
    //     println!("{} thumbnail: {}, source: {}", count, thumbnail, source);
    // }

    let resp = reqwest::get("https://www.2meinv.com/index-1.html")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let document = Html::parse_document(&resp);
    let selector = Selector::parse(".detail-list li").unwrap();
    let mut count = 0;
    for element in document.select(&selector) {
        let mut thumbnail = String::new();
        let mut source = String::new();

        element
            .select(&Selector::parse("img").unwrap())
            .for_each(|img| {
                if let Some(src) = img.value().attr("src") {
                    thumbnail = src.to_owned();
                }
            });
        for link in element.select(&Selector::parse("a").unwrap()) {
            if let Some(href) = link.value().attr("href") {
                let resp = reqwest::get(href).await.unwrap().text().await.unwrap();
                let document = Html::parse_document(&resp);
                let selector = Selector::parse(".pp img").unwrap();

                if let Some(src) = document.select(&selector).next() {
                    if let Some(src) = src.value().attr("src") {
                        source = src.to_owned();
                    }
                }
            }
        }
        count += 1;
        println!("{} thumbnail: {}, source: {}", count, thumbnail, source);
    }
}
