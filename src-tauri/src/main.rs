#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use app::{Image, ImageQuery, Spider, Wallhaven};
use std::{collections::HashMap, sync::Mutex};
use tauri::Manager;

lazy_static! {
    static ref IMAGES: Mutex<HashMap<String, Vec<Image>>> = Mutex::new(HashMap::new());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            hello,
            close_splashscreen,
            get_images,
            set_background,
            download_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn hello() -> String {
    format!("Hello, world!\n")
}

// Create the command:
// This command must be async so that it doesn't run on the main thread.
#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
async fn get_images(page: u8, keyword: String) -> Vec<Image> {
    let mut spider = Wallhaven::new();

    // todo: use a cache to avoid querying the same page twice
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
    // let mut images = IMAGES.lock().unwrap();
    // if images.contains_key(&spider.name()) {
    //     images
    //         .get_mut(&spider.name())
    //         .unwrap()
    //         .append(&mut spider.images.clone());
    // } else {
    //     images.insert(spider.name().clone(), spider.images.clone());
    // }
    println!("{:?}", &spider.images);
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
