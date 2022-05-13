use crate::{diff::Diff, user::User, ROOT};
use pnp_client::json::{DiffJson, StringVecJson};
use std::error::Error;
use tide::Request;

pub struct Games {}

impl Games {
    pub async fn get(user: User, mut request: Request<()>) -> Result<String, String> {
        if let Some(call) = request.url().path().split("/").nth(3) {
            match call {
                "get_all" => {
                    if let Ok(ret) = Self::get_game_folders(&(ROOT.to_string() + "/games")) {
                        let ret =
                            serde_json::ser::to_string(&StringVecJson::from_vec(ret)).unwrap();
                        Ok(ret)
                    } else {
                        Err("games folder not found???".to_string())
                    }
                }
                "hash" => {
                    if let Ok(ret) = Self::get_game_folders(&(ROOT.to_string() + "/games")) {
                        let ret =
                            serde_json::ser::to_string(&StringVecJson::from_vec(ret)).unwrap();
                        Ok(ret)
                    } else {
                        Err("games folder not found???".to_string())
                    }
                }
                _ => Err("unknown api call".to_string()),
            }
        } else {
            return Err("Missing api action".to_string());
        }
    }
    pub async fn post(user: User, mut request: Request<()>) -> Result<String, String> {
        if let Some(call) = request.url().path().split("/").nth(3) {
            match call {
                "diff" => {
                    if let Some(game_name) = request.url().path().split("/").nth(4) {
                        let game_name = game_name.to_string();
                        let diff: DiffJson = request.body_json().await.unwrap();
                        return Diff::write_diff(game_name, diff);
                    } else {
                        return Err("Missing game name".to_string());
                    }
                }
                _ => {
                    return Err("Missing api action".to_string());
                }
            }
        } else {
            return Err("Missing api action".to_string());
        }
    }

    fn get_game_folders(folder: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let files = std::fs::read_dir(&folder)?;
        let mut ret: Vec<String> = Vec::new();
        for file in files {
            if let Ok(file) = file {
                // we only want the folders, since each folder is a game
                if let Ok(filetype) = file.file_type() {
                    if !filetype.is_dir() {
                        continue;
                    }
                }
                // if it is a folder, we return it
                if let Ok(filename) = file.file_name().into_string() {
                    ret.push(filename);
                }
            }
        }
        return Ok(ret);
    }
}
