extern crate pretty_env_logger;
#[macro_use]
extern crate lazy_static;

mod config;

use config::Config;

use std::env;
use warp::Filter;

lazy_static! {
    static ref CONFIG: Config = Config::new().expect("could not load config");
}

type Resp = Result<Box<dyn warp::Reply>, warp::Rejection>;

async fn get_target(name: String) -> Resp {
    match CONFIG.targets.get(&name) {
        Some(target) => Ok(Box::new(format!("dest: {}", target.dest))),
        None => Err(warp::reject::not_found()),
    }
}

#[tokio::main]
async fn main() {
    // Log at the info level by default.
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "diplo=info");
    }
    pretty_env_logger::init();

    // Routes.
    let target_route = warp::path!("target" / String).and_then(get_target);
    let routes = target_route.with(warp::log("diplo"));

    // Start server.
    warp::serve(routes).run(([127, 0, 0, 1], CONFIG.port)).await;
}
