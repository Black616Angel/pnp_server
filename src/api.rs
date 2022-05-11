use std::error::Error;

use log::info;
use pnp_client::json::*;
use tide::http::convert::Deserialize;
use tide::{Request, Response, StatusCode};

use crate::user::User;
use crate::ROOT;

pub struct Api {}

#[derive(Deserialize)]
struct Sid {
    sid: String,
}

impl Api {
    pub async fn call_tide(request: Request<()>) -> tide::Result {
        // this also works if there are more query parameters
        println!("{:?}", request);
        let sid = if let Ok(Sid { sid }) = request.query() {
            sid
        } else {
            println!("SID missing");
            return Ok(Response::builder(StatusCode::NotFound)
                .body("SID missing")
                .build());
        };
        let user: User = match Self::test_auth(sid) {
            Ok(user) => user,
            Err(_) => {
                println!("403");
                return Ok(Response::builder(StatusCode::Forbidden)
                    .body("Nope")
                    .build());
            }
        };
        match Self::call_all(user, request) {
            Ok(response) => return Ok(Response::builder(StatusCode::Ok).body(response).build()),
            Err(response) => {
                println!("{}", response);
                return Ok(Response::builder(StatusCode::NotFound)
                    .body(response)
                    .build());
            }
        }
    }

    fn call_all(user: User, request: Request<()>) -> Result<String, String> {
        if let Some(group) = request.url().path().split("/").nth(2) {
            match group {
                "games" => Self::call_games(user, request),
                "users" => Self::call_users(user, request),
                _ => Err("unknown api group".to_string()),
            }
        } else {
            return Err("missing api group".to_string());
        }
    }

    fn call_games(user: User, request: Request<()>) -> Result<String, String> {
        if let Some(call) = request.url().path().split("/").nth(3) {
            match call {
                "get_all" => {
                    if let Ok(ret) = Api::get_game_folders(&(ROOT.to_string() + "/games")) {
                        let ret =
                            serde_json::ser::to_string(&StringVecJson::from_vec(ret)).unwrap();
                        Ok(ret)
                    } else {
                        Err("games folder not found???".to_string())
                    }
                }
                "hash" => {
                    if let Ok(ret) = Api::get_game_folders(&(ROOT.to_string() + "/games")) {
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

    fn call_users(user: User, request: Request<()>) -> Result<String, String> {
        if let Some(call) = request.url().path().split("/").nth(3) {
            match call {
                "get" => {
                    if let Some(sid) = request.url().path().split("/").nth(4) {
                        if let Some(user) = User::read_sids(sid.to_string(), ROOT.to_string()) {
                            let ret = serde_json::ser::to_string(&user).unwrap();
                            Ok(ret)
                        } else {
                            Err("unknown SID".to_string())
                        }
                    } else {
                        Err("No SID provided".to_string())
                    }
                }
                _ => Err("unknown api call".to_string()),
            }
        } else {
            return Err("Missing api action".to_string());
        }
    }

    fn test_auth(sid: String) -> Result<User, ()> {
        match User::read_sids(sid, ROOT.to_string()) {
            Some(user) => Ok(user),
            None => Err(()),
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
