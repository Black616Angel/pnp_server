use crate::user::User;
use macroquad::{file::*, prelude::load_texture};

use reqwest::Client;

pub struct File {}

impl File {
    pub async fn read_sid(session_id: String) -> User {
        return serde_json::from_str(
            &load_string(&("/user/".to_string() + &session_id))
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

    pub async fn write(path: &str, content: String) -> Result<String, reqwest::Error> {
        let path = Self::convert_path(path, User::get().await);
        let client = Client::new();
        let res = client.post(&path).body(content).send().await?;
        Ok(res.status().to_string())
    }

    fn convert_path(path: &str, user: User) -> String {
        return path.to_string() + "?sid=" + &user.session_id;
    }

    #[cfg(target_arch = "wasm32")]
    fn get_url() -> String {
        use sapp_jsutils::JsObject;
        extern "C" {
            fn js_get_url() -> JsObject;
        }
        let mut key = String::new();
        unsafe {
            js_get_url().to_string(&mut key);
        }
        return key;
    }
}
