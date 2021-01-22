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

async fn say_hello(name: String) -> Resp {
    Ok(Box::new(format!("Hello, {}!", name)))
}

#[tokio::main]
async fn main() {
    // Log at the info level by default.
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "diplo=info");
    }
    pretty_env_logger::init();

    println!("{}", CONFIG.targets["foo"].key);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).and_then(say_hello);

    let routes = hello.with(warp::log("diplo"));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
