/*
// Layout:
// /index.html
// /root/ <- static files
// /games/ <- files that are only needed in that game
// /objects/ <- files that are used in multiple games
// /rulesets/ <- rulesets, that bind together objects
//
// /api/ <- dynamic stuff, that is handled by the backend
// /api/users/ <- get/set user-data
// /api/games/ <- get data to games alltogether like what games exist etc.
// Api-Stuff always look like this:
// /api/<group of things>/<Api-call-name>/<single value>(?possibly=query)
*/

use tide::Redirect;

pub struct Server {
    ip: String,
    port: String,
    root: String,
    pub server: tide::Server<()>,
}

impl Server {
    pub fn new(ip: &str, port: &str, root: String) -> Self {
        // tide::log::start();
        let mut server = tide::new();
        server.with(tide::log::LogMiddleware::new());
        Self {
            ip: ip.to_string(),
            port: port.to_string(),
            root,
            server,
        }
    }

    pub fn add_static(&mut self, url: &str, relative_path: &str) {
        self.server
            .at(&(url.to_string() + "*"))
            .serve_dir(self.root.clone() + relative_path)
            .unwrap();
        self.server.at(url).get(Redirect::new("index.html"));
        // self.server
        //     .at(&(url.to_string()))
        //     .serve_file(self.root.clone() + relative_path + "index.html")
        //     .unwrap();
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let listener = format!("{}:{}", self.ip, &self.port);
        return self.server.listen(listener).await;
    }
}
