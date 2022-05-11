pub mod api;
pub mod diff;
pub mod server;
pub mod server_jsons;
pub mod user;

use crate::api::Api;
use crate::server::Server;

use log::{info, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use tide::{Response, StatusCode};

pub static ROOT: &str = "/var/www/";

#[tokio::main]
async fn main() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("output.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config).unwrap();
    info!("Start1");

    let mut serv = Server::new("0.0.0.0", "8080", ROOT.to_string());
    serv.add_static("/", "");
    serv.server.at("/root/pnp_client.wasm").get(|_| async move {
        println!("test");
        let res = Response::builder(StatusCode::Ok)
            .content_type(tide::http::mime::WASM)
            .body_file(format!("{}pnp_client.wasm", ROOT.to_string()))
            .await;
        match res {
            Ok(res) => return Ok(res),
            Err(e) => {
                println!("{:}", e);
                return Ok(Response::builder(StatusCode::Ok)
                    .content_type(tide::http::mime::HTML)
                    .body(""));
            }
        }
    });
    serv.server.at("/api/*").get(Api::call_tide);
    serv.server.at("/games/*").post(|_| async { Ok("") });
    serv.server.at("/objects/*").post(|_| async { Ok("") });
    serv.server.at("/rulesets/*").post(|_| async { Ok("") });
    serv.run().await.unwrap();
}
