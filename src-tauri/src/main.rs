#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use app::{Image, ImageQuery, Spider, Wallhaven};
use std::{collections::HashMap, sync::Mutex};
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

lazy_static! {
    static ref IMAGES: Mutex<HashMap<String, Vec<Image>>> = Mutex::new(HashMap::new());
}

fn main() {
    let tray = SystemTray::new();
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    // let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let randomly = CustomMenuItem::new("randomly".to_string(), "随机壁纸");
    let tray_menu = SystemTrayMenu::new()
        // .add_item(hide)
        .add_item(randomly)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    tauri::Builder::default()
        .system_tray(tray.with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { position, size, .. } => {
                println!("left click {position:?} {size:?}");
                let window = app.get_window("main").unwrap();
                // window.set_always_on_top(true).unwrap();
                window.center().unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::RightClick { position, size, .. } => {
                println!("right click {position:?} {size:?}");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "randomly" => {
                    // let mut images = IMAGES.lock().unwrap();
                    // let image = images.get_mut("wallhaven").unwrap();
                    // let image = image.choose(&mut rand::thread_rng()).unwrap();
                    // set_background(&image.path).unwrap();
                    let window = app.get_window("main").unwrap();
                    // show todo dialog
                    tauri::api::dialog::message(Some(&window), "tips", "todo");
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                _ => {}
            },
            _ => {
                println!("other");
            }
        })
        .invoke_handler(tauri::generate_handler![
            hello,
            show_main_window,
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

#[tauri::command]
async fn show_main_window(window: tauri::Window) {
    let main = window.get_window("main").unwrap();
    main.show().unwrap();
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
