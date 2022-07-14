/*
 * @Author: mty
 * @Date: 2022-07-12 18:41:44
 * @LastEditTime: 2022-07-14 11:46:22
 * @LastEditors: anonymous
 * @Description:
 * @FilePath: \tauri-app\src-tauri\src\error.rs
 * no code no bug.
 */
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WallPapaerError {
    #[error("invalid operate: {0}")]
    InvalidOperate(String),
    #[error("image path: {0} does not exists!")]
    FileNotExists(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for WallPapaerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
