#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use app::{Image, ImageQuery, Spider, Wallhaven};
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref IMAGES: Mutex<HashMap<String, Vec<Image>>> = Mutex::new(HashMap::new());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            hello,
            get_images,
            set_background,
            download_image
        ])
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
async fn get_images(page: u8, keyword: String) -> Vec<Image> {
    let mut spider = Wallhaven::new();
    // if IMAGES.lock().unwrap().contains_key(&spider.name()) {
    //     return IMAGES.lock().unwrap()[&spider.name()].clone();
    // }

    let query = ImageQuery {
        keyword,
        page,
        tags: None,
    };

    println!("query: {query:?} crawl...");

    spider.crawl(query).await;

    // push to static IMAGES
    IMAGES
        .lock()
        .unwrap()
        .insert(spider.name(), spider.images.clone());

    spider.images
}

#[tauri::command]
async fn download_image(source: String) -> String {
    let mut image = Image::new("".to_string(), source);
    image.download().await.unwrap();
    format!("{}", image.local_path)
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
