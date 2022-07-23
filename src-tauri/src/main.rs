#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::{utils, Image, ImageQuery};
use rand::Rng;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

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
                    let local_images = utils::list_images();
                    match local_images {
                        Ok(images) => {
                            if images.is_empty() {
                                tauri::api::dialog::message(
                                    Some(&app.get_window("main").unwrap()),
                                    "提醒",
                                    "没有可用的图片，请先下载图片！",
                                );
                                return;
                            }
                            let image = images.get(rand::thread_rng().gen_range(0..images.len()));
                            if let Some(image) = image {
                                utils::set_background(&image).unwrap();
                            } else {
                                tauri::api::dialog::message(
                                    Some(&app.get_window("main").unwrap()),
                                    "error",
                                    "获取本地图片出错",
                                );
                            }
                        }
                        Err(e) => {
                            tauri::api::dialog::message(
                                Some(&app.get_window("main").unwrap()),
                                "error",
                                &format!("获取本地图片出错：{}", e),
                            );
                        }
                    }
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
            download_image,
            get_image_sources
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
async fn get_images(source: String, page: u8, keyword: String) -> Vec<Image> {
    let query = ImageQuery {
        source,
        keyword,
        page,
        tags: None,
    };

    println!("query: {query:?} crawl...");

    let images = app::get_images(&query).await;
    if let Ok(images) = images {
        return images;
    }

    return vec![];
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

#[tauri::command]
async fn get_image_sources() -> Vec<String> {
    if let Ok(sources) = app::get_all_sources().await {
        return sources;
    }
    return vec![];
}
