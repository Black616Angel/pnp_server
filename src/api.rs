use std::{collections::HashMap, error::Error};

use pnp_client::json::*;

pub struct Api {}

impl Api {
    pub fn call_all(call: String, root_folder: String) -> Result<String, String> {
        let call: &str = &call;
        match call {
            "games" => {
                if let Ok(ret) = Api::get_game_folders(&(root_folder + "/games")) {
                    let ret = serde_json::ser::to_string(&StringVecJson::from_vec(ret)).unwrap();
                    Ok(ret)
                } else {
                    Err("games folder not found???".to_string())
                }
            }
            _ => Err("unknown api call".to_string()),
        }
    }

    pub fn call_game(
        call: String,
        game_name: String,
        root_folder: String,
        query_param: HashMap<String, String>,
    ) -> Result<String, String> {
        let call: &str = &call;
        match call {
            "hash" => {
                if let Ok(ret) = Api::get_game_folders(&(root_folder + "/games")) {
                    let ret = serde_json::ser::to_string(&StringVecJson::from_vec(ret)).unwrap();
                    Ok(ret)
                } else {
                    Err("games folder not found???".to_string())
                }
            }
            _ => Err("unknown api call".to_string()),
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
