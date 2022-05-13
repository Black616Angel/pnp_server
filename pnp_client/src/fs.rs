use crate::user::User;
use macroquad::{file::*, prelude::load_texture};

pub struct File {}

impl File {
    pub async fn read_sid(session_id: String) -> User {
        return serde_json::from_str(
            &load_string(&format!("/api/users/get/{}?sid={}", session_id, session_id))
                .await
                .unwrap(),
        )
        .unwrap();
    }

    pub async fn read_string(path: &str) -> Result<String, FileError> {
        let path = Self::convert_path(path, User::get().await);
        Ok(load_string(&path).await?)
    }
    pub async fn read_file(path: &str) -> Result<Vec<u8>, FileError> {
        let path = Self::convert_path(path, User::get().await);
        Ok(load_file(&path).await?)
    }
    pub async fn read_texture(path: &str) -> Result<macroquad::texture::Texture2D, FileError> {
        let path = Self::convert_path(path, User::get().await);
        Ok(load_texture(&path).await?)
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn write(path: &str, content: String) -> Result<String, std::io::Error> {
        use std::ffi::CString;

        use macroquad::prelude::info;

        extern "C" {
            pub fn fs_write_file(
                path: *const i8,
                path_len: u32,
                content: *const i8,
                content_len: u32,
            );
        }
        let path = format!(
            "http://{}/{}?sid={}",
            Self::get_url(),
            path,
            User::get().await.session_id
        );
        info!("path: {}", path);
        info!("content: {}", content);
        let path = CString::new(path)?;
        let content = CString::new(content)?;
        unsafe {
            fs_write_file(
                path.as_ptr(),
                path.as_bytes().len() as u32,
                content.as_ptr(),
                content.as_bytes().len() as u32,
            );
        }
        Ok("success".to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn write(_path: &str, _content: String) -> Result<String, std::io::Error> {
        Ok("success".to_string())
    }

    // pub async fn write(path: &str, content: String) -> Result<String, reqwest::Error> {
    //     let path = Self::convert_path(path, User::get().await);
    //     let client = Client::new();
    //     let res = client.post(&path).body(content).send().await?;
    //     Ok(res.status().to_string())
    // }

    fn convert_path(path: &str, user: User) -> String {
        return path.to_string() + "?sid=" + &user.session_id;
    }

    #[cfg(target_arch = "wasm32")]
    pub fn get_url() -> String {
        use sapp_jsutils::JsObject;
        extern "C" {
            fn site_get_url() -> JsObject;
        }
        let mut key = String::new();
        unsafe {
            site_get_url().to_string(&mut key);
        }
        return key;
    }
}
