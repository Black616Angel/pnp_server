pub mod diff;

use rouille::*;
use crate::diff::Diff;

use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

fn main() {
    let root: String = "/var/www/".to_string();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info)).unwrap();

    log4rs::init_config(config).unwrap();
    log::info!("Start");
    rouille::start_server("0.0.0.0:8080", move |request| {
        router!(request,
        (GET) (/) => {
            log::info!("klappt");
            let index = std::fs::read_to_string(root.clone() + "index.html")
            .unwrap_or("Something went wrong reading the file".to_string());
            rouille::Response::html(index)
        },
        (GET) (/{_game_name: String}/{_filename: String}) => {
            //TODO: check credentials etc.
            let resp = rouille::match_assets(&request, &(root.clone() + "games"));
            if resp.is_success() {
                resp
            } else {
                rouille::Response::text("klappt")
            }
        },
        (GET) (/{game_name: String}/{hash: String}) => {
            rouille::Response::text(&Diff::get_diff(game_name, hash))
        },
        _ => {
            rouille::Response::empty_404()
        }
        )
    })
}
