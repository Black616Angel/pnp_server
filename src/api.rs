pub mod files;
pub mod games;
pub mod users;

use crate::api::files::*;
use crate::api::games::*;
use crate::api::users::*;
use crate::user::User;
use crate::ROOT;
use tide::http::convert::Deserialize;
use tide::http::Method;
use tide::{Request, Response, StatusCode};

pub struct Api {}

#[derive(Deserialize)]
struct Sid {
    sid: String,
}

impl Api {
    pub async fn call_tide(request: Request<()>) -> tide::Result {
        // this also works if there are more query parameters
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
        let res = match request.method() {
            Method::Get => Self::get_all(user, request).await,
            Method::Post => Self::post_all(user, request).await,
            _ => {
                return Ok(Response::builder(StatusCode::NotFound)
                    .body("unsupported HTTP method")
                    .build());
            }
        };
        match res {
            Ok(response) => return Ok(Response::builder(StatusCode::Ok).body(response).build()),
            Err(response) => {
                println!("{}", response);
                return Ok(Response::builder(StatusCode::NotFound)
                    .body(response)
                    .build());
            }
        }
    }

    async fn get_all(user: User, mut request: Request<()>) -> Result<String, String> {
        if let Some(group) = request.url().path().split("/").nth(2) {
            match group {
                "games" => Games::get(user, request).await,
                "users" => Users::get(user, request).await,
                _ => Err("unknown api group".to_string()),
            }
        } else {
            return Err("missing api group".to_string());
        }
    }

    async fn post_all(user: User, mut request: Request<()>) -> Result<String, String> {
        if let Some(first) = request.url().path().split("/").nth(1) {
            match first {
                "api" => {
                    if let Some(group) = request.url().path().split("/").nth(2) {
                        match group {
                            "games" => Games::post(user, request).await,
                            "users" => Users::post(user, request).await,
                            _ => Err("unknown api group".to_string()),
                        }
                    } else {
                        return Err("missing api group".to_string());
                    }
                }
                _ => return Files::post_files(user, request).await,
            }
        } else {
            return Err("can't post to root".to_string());
        }
    }

    fn test_auth(sid: String) -> Result<User, ()> {
        match User::read_sids(sid, ROOT.to_string()) {
            Some(user) => Ok(user),
            None => Err(()),
        }
    }
}
