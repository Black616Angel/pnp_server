use crate::user::User;
use tide::Request;

pub struct Files {}

impl Files {
    pub async fn post_files(user: User, request: Request<()>) -> Result<String, String> {
        if let Some(group) = request.url().path().split("/").nth(2) {
            match group {
                "games" => Self::post_games(user, request).await,
                "objects" => Self::post_objects(user, request).await,
                "rulesets" => Self::post_rulesets(user, request).await,
                _ => Err("unknown api group".to_string()),
            }
        } else {
            return Err("missing api group".to_string());
        }
    }
    async fn post_games(user: User, request: Request<()>) -> Result<String, String> {
        return Err("missing api group".to_string());
    }
    async fn post_objects(user: User, request: Request<()>) -> Result<String, String> {
        return Err("missing api group".to_string());
    }
    async fn post_rulesets(user: User, request: Request<()>) -> Result<String, String> {
        return Err("missing api group".to_string());
    }
}
