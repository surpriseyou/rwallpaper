#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use app::{Image, Spider, Wallhaven};
use std::collections::HashMap;

lazy_static! {
    static ref IMAGES: HashMap<String, Vec<Image>> = HashMap::new();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![hello, get_images, set_background])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn hello() -> String {
    let resp = reqwest::get("https://httpbin.org/get").await.unwrap();
    let body = resp.text().await.unwrap();
    format!("Hello, world!\n{}", body)
}

#[tauri::command]
async fn get_images() -> Vec<Image> {
    let mut spider = Wallhaven::new();
    spider.crawl().await;

    for image in spider.images.iter_mut() {
        // image.download().await.unwrap();
        // println!("{:?}", image);
    }

    spider.images
}

#[tauri::command]
async fn set_background(source: String, thumbnail: String) -> bool {
    let mut image = Image::new(thumbnail, source);
    println!("-----> set background {:?}", image);
    image.download().await.unwrap();
    println!("{:?}", image);
    image.set_background().unwrap();
    println!("OK!");
    true
}
