pub mod api;
pub mod diff;

use crate::api::Api;
use crate::diff::Diff;

use log::{info, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use warp::Filter;

#[tokio::main]
async fn main() {
    let root: String = "/var/www/".to_string();
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

    let file_index = root.clone() + "root/index.html";
    let folder_main = root.clone() + "root/";
    let folder_games = root.clone() + "games/";
    let folder_objects = root.clone() + "objects/";
    let folder_rulesets = root.clone() + "rulesets/";

    let index = warp::get()
    .and(warp::path::end())
    .and(warp::fs::file(file_index));

    // dir already requires GET...
    let main = warp::path("root").and(warp::fs::dir(folder_main));
    let games = warp::path("games").and(warp::fs::dir(folder_games));
    let objects = warp::path("objects").and(warp::fs::dir(folder_objects));
    let rulesets = warp::path("rulesets").and(warp::fs::dir(folder_rulesets));

    let root_api_all = root.clone();
    let api_all = warp::path!("api" / String ).map(move |call| {
        let call: String = call;
        log::info!("{}", call);
        format!("{}", Api::call_all(call.clone(), root_api_all.clone()).unwrap_or("".to_string()))
    });

    let root_api_games = root.clone();
    let api_games = warp::path!("api" / String / String ).map(move |game_name: String, call: String| {
        log::info!("{}: {}", call, game_name);
        format!("{}", Api::call_game(call.clone(), game_name.clone(), root_api_games.clone()).unwrap_or("".to_string()))
    });

    let routes = warp::get().and(
                index
                    .or(main)
                    .or(games)
                    .or(objects)
                    .or(rulesets)
                    .or(api_all)
                    .or(api_games));

    warp::serve(routes).run(([0,0,0,0], 8080)).await

    //     (GET) (/api/{game_name: String}/{hash: String}) => {
    //         log::info!("{}", &request.raw_url());
    //         rouille::Response::text(&Diff::get_diff(game_name, hash))
    //     },
    //     _ => {
    //         log::info!("FAIL: {}", &request.raw_url());
    //         rouille::Response::empty_404()
    //     }
    //     )
    // })
}
