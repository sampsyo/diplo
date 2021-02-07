#[macro_use]
extern crate lazy_static;

mod config;
use config::{Config, Target};

use warp::Filter;
use simple_logger::SimpleLogger;

lazy_static! {
    static ref CONFIG: Config = Config::new().expect("could not load config");
}

type Resp = Result<Box<dyn warp::Reply>, warp::Rejection>;

async fn get_target(target: &'static Target, method: http::Method) -> Resp {
    match method {
        http::Method::GET => Ok(Box::new(format!("dest: {}", target.dest))),
        _ => Err(warp::reject()),
    }
}

async fn lookup_target(name: String) -> Result<&'static Target, warp::Rejection> {
    match CONFIG.targets.get(&name) {
        Some(target) => Ok(target),
        None => Err(warp::reject::not_found()),
    }
}

#[tokio::main]
async fn main() {
    // Log at the info level by default.
    SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap();

    // Routes.
    let target_route = warp::path!("target" / String)
        .and_then(lookup_target)
        .and(warp::method())
        .and_then(get_target);
    let routes = target_route.with(warp::log("diplo"));

    // Start server.
    let serve = warp::serve(routes).run(CONFIG.host);
    log::info!("running at http://{}", CONFIG.host);
    serve.await;
}
