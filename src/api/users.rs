use crate::{user::User, ROOT};
use tide::Request;

pub struct Users {}

impl Users {
    pub async fn get(user: User, request: Request<()>) -> Result<String, String> {
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
    pub async fn post(user: User, request: Request<()>) -> Result<String, String> {
        if let Some(call) = request.url().path().split("/").nth(3) {
            match call {
                "set" => {
                    return Err("Missing api action".to_string());
                }
                _ => {
                    return Err("Missing api action".to_string());
                }
            }
        } else {
            return Err("Missing api action".to_string());
        }
    }
}
