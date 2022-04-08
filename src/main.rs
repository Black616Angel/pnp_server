pub mod diff;

use rouille::*;
use crate::diff::Diff;

fn main() {
    rouille::start_server_with_pool("localhost:8080", Some(8), move |request| {
        router!(request,
        (GET) (/) => {
            rouille::Response::text("klappt")
        },
        (GET) (/{_game_name: String}/{_filename: String}) => {
            //TODO: check credentials etc.
            let resp = rouille::match_assets(&request, "files/games");
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
