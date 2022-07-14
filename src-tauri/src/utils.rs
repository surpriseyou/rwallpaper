/*
 * @Author: mty
 * @Date: 2022-07-12 18:59:34
 * @LastEditTime: 2022-07-14 21:44:21
 * @LastEditors: anonymous
 * @Description:
 * @FilePath: \rwallpaper\src-tauri\src\utils.rs
 * no code no bug.
 */

use anyhow::{Ok, Result};
use reqwest;
use std::{io::Write, path::Path};
use wallpaper;

use crate::WallPapaerError;

pub async fn download(source_url: &str, file_name: &str) -> String {
    let resp = reqwest::get(source_url).await.unwrap();
    let bytes = resp.bytes().await.unwrap();

    // current dir
    let current_path = std::env::current_dir().unwrap();

    // append image folder
    let mut file_path = current_path.join("images");

    if !file_path.exists() {
        std::fs::create_dir_all(file_path.clone()).unwrap();
    }

    file_path.push(file_name);

    // if file exists, skip it
    if file_path.exists() {
        return file_path.to_str().unwrap().to_string();
    }

    let local_path = file_path.clone();

    // write bytes to file
    let mut file = std::fs::File::create(file_path).unwrap();

    file.write(bytes.as_ref()).unwrap();

    return local_path.to_str().unwrap().to_string();
}

pub fn set_background(image_path: &str) -> Result<()> {
    // check image_path is valid
    if image_path.is_empty() {
        return Ok(());
    }

    // check image exists
    if !Path::new(image_path).exists() {
        return Err(WallPapaerError::FileNotExists(image_path.to_owned()).into());
    }

    wallpaper::set_from_path(image_path).unwrap();
    wallpaper::set_mode(wallpaper::Mode::Stretch).unwrap();
    Ok(())
}
